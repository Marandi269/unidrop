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
use crate::multicast::MulticastDiscovery;
use crate::quic::{QuicClient, QuicServer, QUIC_PORT_OFFSET};
use crate::server::HttpServer;
use crate::{DEFAULT_PORT, PROTOCOL_ID, PROTOCOL_VERSION};

/// LocalSend 协议实现
pub struct LocalSendProtocol {
    info: ProtocolInfo,
    cert: CertInfo,
    running: RwLock<bool>,
    discovery: RwLock<Option<DiscoveryService>>,
    multicast: RwLock<Option<MulticastDiscovery>>,
    client: RwLock<Option<HttpClient>>,
    quic_client: RwLock<Option<QuicClient>>,
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
            multicast: RwLock::new(None),
            client: RwLock::new(None),
            quic_client: RwLock::new(None),
            event_tx,
            event_rx: RwLock::new(Some(event_rx)),
            local_info: RwLock::new(None),
        }
    }

    /// 使用 QUIC 发送文件
    pub async fn send_quic(&self, intent: TransferIntent) -> Result<String> {
        let quic_client = self
            .quic_client
            .read()
            .as_ref()
            .cloned()
            .ok_or_else(|| unidrop_core::Error::Protocol("Protocol not started".into()))?;

        let device = self
            .device(&intent.target)
            .await
            .ok_or_else(|| unidrop_core::Error::DeviceNotFound(intent.target.to_string()))?;

        // QUIC 端口 = HTTP 端口 + 1
        let quic_addr = std::net::SocketAddr::new(device.ip, device.port + QUIC_PORT_OFFSET);
        let session_id = quic_client.send_files(quic_addr, intent.files).await?;
        Ok(session_id)
    }

    /// 获取证书信息（用于外部创建 QUIC 服务器）
    pub fn cert(&self) -> &CertInfo {
        &self.cert
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
        *self.quic_client.write() = Some(QuicClient::new()?);

        // 启动 mDNS 发现服务
        let mut discovery = DiscoveryService::new(local_info.clone(), self.event_tx.clone());
        discovery.start()?;
        *self.discovery.write() = Some(discovery);

        // 启动 UDP multicast 发现服务（LocalSend 主要发现机制）
        let mut multicast = MulticastDiscovery::new(local_info.clone(), self.event_tx.clone());
        if let Err(e) = multicast.start() {
            tracing::warn!("Failed to start multicast discovery: {} (mDNS only)", e);
        } else {
            *self.multicast.write() = Some(multicast);
        }

        // 启动 HTTPS 服务器（在后台任务中）
        let server = HttpServer::new(
            local_info,
            config.save_dir.clone(),
            config.pin.clone(),
            self.event_tx.clone(),
            &self.cert,
        )?;

        tokio::spawn(async move {
            if let Err(e) = server.start().await {
                tracing::error!("HTTPS server error: {}", e);
            }
        });

        // 启动 QUIC 服务器（可选，用于 UniDrop 之间的高速传输）
        let quic_port = port + QUIC_PORT_OFFSET;
        let cert_clone = self.cert.clone();
        let quic_save_dir = config.save_dir.clone();

        tokio::spawn(async move {
            match QuicServer::new(quic_port, &cert_clone, quic_save_dir) {
                Ok(quic_server) => {
                    if let Err(e) = quic_server.run().await {
                        tracing::error!("QUIC server error: {}", e);
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to start QUIC server: {} (QUIC disabled)", e);
                }
            }
        });

        *self.running.write() = true;
        info!("LocalSend protocol started on port {} (QUIC: {})", port, quic_port);

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
        let mut devices = std::collections::HashMap::new();

        // 从 mDNS 获取设备
        if let Some(discovery) = self.discovery.read().as_ref() {
            for device in discovery.devices() {
                devices.insert(device.peer.id.fingerprint.clone(), device);
            }
        }

        // 从 multicast 获取设备（可能有重复，用 fingerprint 去重）
        if let Some(multicast) = self.multicast.read().as_ref() {
            for device in multicast.devices() {
                devices.insert(device.peer.id.fingerprint.clone(), device);
            }
        }

        devices.into_values().collect()
    }

    async fn device(&self, id: &DeviceId) -> Option<Device> {
        if id.protocol.as_str() != PROTOCOL_ID {
            return None;
        }

        // 先从 mDNS 查找
        if let Some(device) = self.discovery.read().as_ref().and_then(|d| d.device(&id.fingerprint)) {
            return Some(device);
        }

        // 再从 multicast 查找
        self.multicast.read().as_ref().and_then(|m| m.device(&id.fingerprint))
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

    async fn send_quic(&self, intent: TransferIntent) -> Result<String> {
        let quic_client = self
            .quic_client
            .read()
            .as_ref()
            .cloned()
            .ok_or_else(|| unidrop_core::Error::Protocol("Protocol not started".into()))?;

        let device = self
            .device(&intent.target)
            .await
            .ok_or_else(|| unidrop_core::Error::DeviceNotFound(intent.target.to_string()))?;

        // QUIC 端口 = HTTP 端口 + 1
        let quic_addr = std::net::SocketAddr::new(device.ip, device.port + QUIC_PORT_OFFSET);
        info!("Sending via QUIC to {}", quic_addr);

        let session_id = quic_client.send_files(quic_addr, intent.files).await?;
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
        // 取出 event_rx（只能取一次）
        self.event_rx
            .write()
            .take()
            .expect("subscribe() can only be called once")
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
