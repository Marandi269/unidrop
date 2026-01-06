//! LocalSend Protocol 实现 - 实现 Protocol trait

use async_trait::async_trait;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};

use unidrop_core::{
    Device, DeviceId, Event, Protocol, ProtocolBuilder, ProtocolConfig, ProtocolFactory,
    ProtocolId, ProtocolInfo, Result, TransferIntent,
};

use crate::cert::{generate_self_signed, CertInfo};
use crate::client::HttpClient;
use crate::discovery::DiscoveryService;
use crate::models::DeviceInfo;
use crate::server::HttpServer;
use crate::{DEFAULT_PORT, PROTOCOL_ID, PROTOCOL_VERSION};

/// LocalSend 协议实现
pub struct LocalSendProtocol {
    info: ProtocolInfo,
    cert: CertInfo,
    running: RwLock<bool>,
    discovery: RwLock<Option<DiscoveryService>>,
    client: RwLock<Option<HttpClient>>,
    event_tx: mpsc::Sender<Event>,
    event_rx: RwLock<Option<mpsc::Receiver<Event>>>,
    local_info: RwLock<Option<DeviceInfo>>,
}

impl LocalSendProtocol {
    pub fn new() -> Self {
        let info = ProtocolBuilder::new(PROTOCOL_ID)
            .name("LocalSend")
            .version(PROTOCOL_VERSION)
            .description("Cross-platform file sharing compatible with LocalSend")
            .priority(100)
            .build_info();

        let cert = generate_self_signed("UniDrop").expect("Failed to generate certificate");
        let (event_tx, event_rx) = mpsc::channel(256);

        Self {
            info,
            cert,
            running: RwLock::new(false),
            discovery: RwLock::new(None),
            client: RwLock::new(None),
            event_tx,
            event_rx: RwLock::new(Some(event_rx)),
            local_info: RwLock::new(None),
        }
    }
}

#[async_trait]
impl Protocol for LocalSendProtocol {
    fn info(&self) -> &ProtocolInfo {
        &self.info
    }

    async fn start(&self, config: ProtocolConfig) -> Result<()> {
        if *self.running.read() {
            return Ok(());
        }

        info!("Starting LocalSend protocol");

        let port = if config.port == 0 {
            DEFAULT_PORT
        } else {
            config.port
        };

        // 创建本地设备信息
        let local_info = DeviceInfo::new(
            config.device_name.clone(),
            self.cert.device_id.clone(),
            port,
        );

        *self.local_info.write() = Some(local_info.clone());

        // 创建客户端
        *self.client.write() = Some(HttpClient::new(local_info.clone()));

        // 启动发现服务
        let mut discovery = DiscoveryService::new(local_info.clone(), self.event_tx.clone());
        discovery.start()?;
        *self.discovery.write() = Some(discovery);

        // 启动 HTTP 服务器（在后台任务中）
        let server = HttpServer::new(
            local_info,
            config.save_dir.clone(),
            config.pin.clone(),
            self.event_tx.clone(),
        );

        tokio::spawn(async move {
            if let Err(e) = server.start().await {
                tracing::error!("HTTP server error: {}", e);
            }
        });

        *self.running.write() = true;
        info!("LocalSend protocol started on port {}", port);

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        if !*self.running.read() {
            return Ok(());
        }

        info!("Stopping LocalSend protocol");

        if let Some(mut discovery) = self.discovery.write().take() {
            discovery.stop();
        }

        *self.client.write() = None;
        *self.running.write() = false;

        info!("LocalSend protocol stopped");
        Ok(())
    }

    fn is_running(&self) -> bool {
        *self.running.read()
    }

    async fn devices(&self) -> Vec<Device> {
        self.discovery
            .read()
            .as_ref()
            .map(|d| d.devices())
            .unwrap_or_default()
    }

    async fn device(&self, id: &DeviceId) -> Option<Device> {
        if id.protocol.as_str() != PROTOCOL_ID {
            return None;
        }

        self.discovery
            .read()
            .as_ref()
            .and_then(|d| d.device(&id.fingerprint))
    }

    async fn scan(&self) -> Result<()> {
        // mDNS 是被动发现，这里可以触发一次主动查询
        debug!("Scan requested (mDNS is passive, no action needed)");
        Ok(())
    }

    async fn send(&self, intent: TransferIntent) -> Result<String> {
        let client = self
            .client
            .read()
            .as_ref()
            .cloned()
            .ok_or_else(|| unidrop_core::Error::Protocol("Protocol not started".into()))?;

        let device = self
            .device(&intent.target)
            .await
            .ok_or_else(|| unidrop_core::Error::DeviceNotFound(intent.target.to_string()))?;

        let session_id = client.send_files(&device, intent.files).await?;
        Ok(session_id)
    }

    async fn accept(&self, request_id: &str, _save_dir: PathBuf) -> Result<()> {
        // HTTP 服务器自动处理接收
        debug!("Accept transfer: {}", request_id);
        Ok(())
    }

    async fn reject(&self, request_id: &str) -> Result<()> {
        debug!("Reject transfer: {}", request_id);
        Ok(())
    }

    async fn cancel(&self, transfer_id: &str) -> Result<()> {
        debug!("Cancel transfer: {}", transfer_id);
        Ok(())
    }

    fn subscribe(&self) -> mpsc::Receiver<Event> {
        // 返回一个新的接收端
        // 注意：这里简化处理，实际应该使用 broadcast channel
        let (tx, rx) = mpsc::channel(256);

        // 转发事件
        let event_tx = self.event_tx.clone();
        tokio::spawn(async move {
            // 这里只是占位，实际需要更复杂的订阅机制
        });

        rx
    }
}

impl Default for LocalSendProtocol {
    fn default() -> Self {
        Self::new()
    }
}

// === Protocol Factory ===

/// LocalSend 协议工厂
pub struct LocalSendFactory;

impl LocalSendFactory {
    pub fn new() -> Self {
        Self
    }
}

impl ProtocolFactory for LocalSendFactory {
    fn create(&self) -> Arc<dyn Protocol> {
        Arc::new(LocalSendProtocol::new())
    }

    fn info(&self) -> ProtocolInfo {
        ProtocolBuilder::new(PROTOCOL_ID)
            .name("LocalSend")
            .version(PROTOCOL_VERSION)
            .description("Cross-platform file sharing compatible with LocalSend")
            .priority(100)
            .build_info()
    }
}

impl Default for LocalSendFactory {
    fn default() -> Self {
        Self::new()
    }
}
