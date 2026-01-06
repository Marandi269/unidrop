//! UniDrop Engine - 核心引擎
//!
//! Engine 是整个系统的入口点，上层业务代码只与 Engine 交互。

use futures::StreamExt;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error, info, warn};

use unidrop_core::{
    Device, DeviceId, Event, EventKind, Protocol, ProtocolConfig, ProtocolFactory, ProtocolId,
    ProtocolInfo, Result, TransferIntent, TransferRequest,
};

use crate::{ProtocolRegistry, TransferRouter};

/// Engine 配置
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// 本机设备名称
    pub device_name: String,
    /// 文件保存目录
    pub save_dir: PathBuf,
    /// 是否启用加密
    pub encryption: bool,
    /// 可选 PIN 码
    pub pin: Option<String>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            device_name: hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "UniDrop".to_string()),
            save_dir: dirs::download_dir()
                .or_else(dirs::home_dir)
                .unwrap_or_else(std::env::temp_dir)
                .join("UniDrop"),
            encryption: true,
            pin: None,
        }
    }
}

impl From<EngineConfig> for ProtocolConfig {
    fn from(config: EngineConfig) -> Self {
        ProtocolConfig {
            device_name: config.device_name,
            port: 0,
            save_dir: config.save_dir,
            encryption: config.encryption,
            pin: config.pin,
        }
    }
}

/// UniDrop Engine
///
/// 上层业务的唯一入口点，提供：
/// - 协议管理
/// - 设备发现（聚合多协议）
/// - 文件传输（自动路由）
/// - 事件订阅
pub struct Engine {
    config: RwLock<EngineConfig>,
    registry: Arc<ProtocolRegistry>,
    router: Arc<TransferRouter>,
    devices: RwLock<HashMap<DeviceId, Device>>,
    event_tx: broadcast::Sender<Event>,
    running: RwLock<bool>,
}

impl Engine {
    /// 创建新的 Engine 实例
    pub fn new(config: EngineConfig) -> Self {
        let (event_tx, _) = broadcast::channel(256);
        let registry = Arc::new(ProtocolRegistry::new());
        let router = Arc::new(TransferRouter::new(registry.clone()));

        Self {
            config: RwLock::new(config),
            registry,
            router,
            devices: RwLock::new(HashMap::new()),
            event_tx,
            running: RwLock::new(false),
        }
    }

    /// 使用 Builder 模式创建
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    // === 协议管理 ===

    /// 注册协议
    pub fn register_protocol<F: ProtocolFactory + 'static>(&self, factory: F) {
        self.registry.register(factory);
    }

    /// 获取已注册的协议列表
    pub fn protocols(&self) -> Vec<ProtocolInfo> {
        self.registry.list()
    }

    // === 生命周期 ===

    /// 启动引擎（启动所有协议）
    pub async fn start(&self) -> Result<()> {
        if *self.running.read() {
            return Ok(());
        }

        info!("Starting UniDrop Engine");

        let config: ProtocolConfig = self.config.read().clone().into();
        let protocols = self.registry.sorted_by_priority();

        for info in protocols {
            if let Some(protocol) = self.registry.get_or_create(&info.id) {
                match protocol.start(config.clone()).await {
                    Ok(_) => {
                        info!("Started protocol: {}", info.name);
                        self.spawn_event_forwarder(protocol.clone());
                    }
                    Err(e) => {
                        error!("Failed to start protocol {}: {}", info.name, e);
                    }
                }
            }
        }

        *self.running.write() = true;
        self.emit(Event::new(EventKind::ProtocolStarted {
            protocol: "engine".to_string(),
        }));

        Ok(())
    }

    /// 停止引擎
    pub async fn stop(&self) -> Result<()> {
        if !*self.running.read() {
            return Ok(());
        }

        info!("Stopping UniDrop Engine");

        for protocol in self.registry.instances() {
            if let Err(e) = protocol.stop().await {
                error!("Failed to stop protocol {}: {}", protocol.id(), e);
            }
        }

        *self.running.write() = false;
        self.devices.write().clear();
        self.emit(Event::new(EventKind::ProtocolStopped {
            protocol: "engine".to_string(),
        }));

        Ok(())
    }

    /// 是否正在运行
    pub fn is_running(&self) -> bool {
        *self.running.read()
    }

    // === 设备发现 ===

    /// 获取所有在线设备（聚合所有协议）
    pub async fn devices(&self) -> Vec<Device> {
        let mut all_devices = Vec::new();

        for protocol in self.registry.instances() {
            if protocol.is_running() {
                all_devices.extend(protocol.devices().await);
            }
        }

        // 更新本地缓存
        {
            let mut cache = self.devices.write();
            for device in &all_devices {
                cache.insert(device.id().clone(), device.clone());
            }
        }

        all_devices
    }

    /// 根据 ID 获取设备
    pub async fn device(&self, id: &DeviceId) -> Option<Device> {
        // 先查缓存
        if let Some(device) = self.devices.read().get(id) {
            return Some(device.clone());
        }

        // 查协议
        let protocol = self.registry.get(&id.protocol)?;
        protocol.device(id).await
    }

    /// 主动扫描
    pub async fn scan(&self) -> Result<()> {
        for protocol in self.registry.instances() {
            if protocol.is_running() {
                if let Err(e) = protocol.scan().await {
                    warn!("Scan failed for {}: {}", protocol.id(), e);
                }
            }
        }
        Ok(())
    }

    // === 传输操作 ===

    /// 发送文件到设备
    pub async fn send(&self, intent: TransferIntent) -> Result<String> {
        self.router.send(intent).await
    }

    /// 发送文件（便捷方法）
    pub async fn send_files(&self, target: DeviceId, files: Vec<PathBuf>) -> Result<String> {
        self.send(TransferIntent::new(target, files)).await
    }

    /// 接受传输请求
    pub async fn accept(&self, request: &TransferRequest) -> Result<()> {
        let protocol = self
            .registry
            .get(request.from.protocol())
            .ok_or_else(|| {
                unidrop_core::Error::ProtocolNotFound(request.from.protocol().to_string())
            })?;

        let save_dir = self.config.read().save_dir.clone();
        protocol.accept(&request.id, save_dir).await
    }

    /// 拒绝传输请求
    pub async fn reject(&self, request: &TransferRequest) -> Result<()> {
        let protocol = self
            .registry
            .get(request.from.protocol())
            .ok_or_else(|| {
                unidrop_core::Error::ProtocolNotFound(request.from.protocol().to_string())
            })?;

        protocol.reject(&request.id).await
    }

    /// 取消传输
    pub async fn cancel(&self, transfer_id: &str, protocol_id: &ProtocolId) -> Result<()> {
        let protocol = self
            .registry
            .get(protocol_id)
            .ok_or_else(|| unidrop_core::Error::ProtocolNotFound(protocol_id.to_string()))?;

        protocol.cancel(transfer_id).await
    }

    // === 事件订阅 ===

    /// 订阅事件
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.event_tx.subscribe()
    }

    // === 内部方法 ===

    fn emit(&self, event: Event) {
        let _ = self.event_tx.send(event);
    }

    /// 启动协议事件转发任务
    fn spawn_event_forwarder(&self, protocol: Arc<dyn Protocol>) {
        let event_tx = self.event_tx.clone();
        let devices = Arc::new(RwLock::new(HashMap::new())); // 独立的设备缓存
        let protocol_id = protocol.id().clone();

        tokio::spawn(async move {
            let mut rx = protocol.subscribe();

            while let Some(event) = rx.recv().await {
                // 更新设备缓存
                match &event.kind {
                    EventKind::DeviceDiscovered(device) => {
                        devices.write().insert(device.id().clone(), device.clone());
                    }
                    EventKind::DeviceLost(id) => {
                        devices.write().remove(id);
                    }
                    EventKind::DeviceUpdated(device) => {
                        devices.write().insert(device.id().clone(), device.clone());
                    }
                    _ => {}
                }

                // 转发事件
                let _ = event_tx.send(event);
            }

            debug!("Event forwarder for {} stopped", protocol_id);
        });
    }
}

/// Engine Builder
#[derive(Default)]
pub struct EngineBuilder {
    config: EngineConfig,
    factories: Vec<Arc<dyn ProtocolFactory>>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, config: EngineConfig) -> Self {
        self.config = config;
        self
    }

    pub fn device_name(mut self, name: impl Into<String>) -> Self {
        self.config.device_name = name.into();
        self
    }

    pub fn save_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config.save_dir = dir.into();
        self
    }

    pub fn with_protocol<F: ProtocolFactory + 'static>(mut self, factory: F) -> Self {
        self.factories.push(Arc::new(factory));
        self
    }

    pub fn build(self) -> Engine {
        let engine = Engine::new(self.config);

        for factory in self.factories {
            engine.registry.register_arc(factory);
        }

        engine
    }
}
