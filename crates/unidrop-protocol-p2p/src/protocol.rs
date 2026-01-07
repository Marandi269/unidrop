//! P2P 协议实现

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use libp2p::{
    Multiaddr, PeerId, SwarmBuilder,
    dcutr, identify, noise, ping, relay, tcp, yamux,
    swarm::SwarmEvent,
    request_response::{self, OutboundRequestId},
};
use parking_lot::RwLock;
use tokio::sync::{mpsc, oneshot};
use tracing::{info, warn, debug};

use unidrop_core::{
    Device, DeviceId, DeviceType, Event, Protocol, ProtocolBuilder, ProtocolConfig,
    ProtocolFactory, ProtocolInfo, ProtocolId, Peer, Result, TransferIntent,
    TransferRequest, FileInfo,
};

use crate::behaviour::{
    P2pClientBehaviour, P2pClientBehaviourEvent, FileRequest, FileResponse,
    FileChunk, FileChunkAck,
};
use crate::transfer::{TransferManager, TransferSession};

/// P2P 协议 ID
pub const P2P_PROTOCOL_ID: &str = "p2p";

/// 默认的公共中继服务器
pub const DEFAULT_RELAY_SERVERS: &[&str] = &[
    // 自建中继服务器 (hk01)
    "/ip4/156.225.28.220/tcp/9001/p2p/12D3KooWCXsQB737PXEosCDxeBTd7Ze4NGsba8WJiUTddjqBkCGg",
];

/// Swarm 命令
enum SwarmCommand {
    /// 连接到对端
    Dial { addr: Multiaddr, reply: oneshot::Sender<anyhow::Result<()>> },
    /// 发送文件请求
    SendRequest { peer_id: PeerId, request: FileRequest, reply: oneshot::Sender<OutboundRequestId> },
    /// 获取本地 Peer ID
    GetLocalPeerId { reply: oneshot::Sender<PeerId> },
    /// 发送文件数据块
    SendFileChunk { peer_id: PeerId, chunk: FileChunk, reply: oneshot::Sender<OutboundRequestId> },
}

/// P2P 协议配置
#[derive(Debug, Clone)]
pub struct P2pConfig {
    /// 中转服务器地址列表
    pub relay_servers: Vec<Multiaddr>,
    /// 本地监听端口
    pub port: u16,
    /// 是否使用默认 bootstrap 节点
    pub use_default_bootstrap: bool,
}

impl Default for P2pConfig {
    fn default() -> Self {
        Self {
            relay_servers: vec![],
            port: 4002,
            use_default_bootstrap: true,
        }
    }
}

/// 发送中的文件信息
#[derive(Debug, Clone)]
struct SendingFile {
    /// 文件 ID
    file_id: String,
    /// 文件路径
    path: PathBuf,
    /// 文件大小
    size: u64,
    /// 已发送的块数
    chunks_sent: u64,
    /// 总块数
    total_chunks: u64,
}

/// 接收中的文件信息
#[derive(Debug, Clone)]
struct ReceivingFile {
    /// 文件 ID
    file_id: String,
    /// 文件名
    name: String,
    /// 文件大小
    size: u64,
    /// 保存目录
    save_dir: PathBuf,
    /// 已接收的块数
    chunks_received: u64,
    /// 总块数
    total_chunks: u64,
}

/// 发送会话
#[derive(Debug)]
struct SendSession {
    /// 目标 Peer ID
    peer_id: PeerId,
    /// 待发送的文件列表
    files: Vec<SendingFile>,
    /// 当前发送的文件索引
    current_file: usize,
}

/// 接收会话
#[derive(Debug)]
struct ReceiveSession {
    /// 来源 Peer ID
    peer_id: PeerId,
    /// 待接收的文件列表
    files: Vec<ReceivingFile>,
    /// 保存目录
    save_dir: PathBuf,
}

/// P2P 协议实现
pub struct P2pProtocol {
    info: ProtocolInfo,
    config: RwLock<Option<P2pConfig>>,
    running: RwLock<bool>,
    devices: RwLock<Vec<Device>>,
    transfers: Arc<TransferManager>,
    event_tx: RwLock<Option<mpsc::Sender<Event>>>,
    local_peer_id: RwLock<Option<PeerId>>,
    shutdown_tx: RwLock<Option<oneshot::Sender<()>>>,
    command_tx: RwLock<Option<mpsc::Sender<SwarmCommand>>>,
    /// 待处理的传输请求
    pending_requests: RwLock<HashMap<String, TransferRequest>>,
    /// 中继地址
    relay_addr: RwLock<Option<String>>,
    /// 发送会话
    send_sessions: RwLock<HashMap<String, SendSession>>,
    /// 接收会话
    receive_sessions: RwLock<HashMap<String, ReceiveSession>>,
}

impl P2pProtocol {
    pub fn new() -> Self {
        let info = ProtocolBuilder::new(P2P_PROTOCOL_ID)
            .name("P2P Direct")
            .version("1.0.0")
            .description("点对点传输，支持 NAT 穿透")
            .priority(50)
            .build_info();

        Self {
            info,
            config: RwLock::new(None),
            running: RwLock::new(false),
            devices: RwLock::new(Vec::new()),
            transfers: Arc::new(TransferManager::new()),
            event_tx: RwLock::new(None),
            local_peer_id: RwLock::new(None),
            shutdown_tx: RwLock::new(None),
            command_tx: RwLock::new(None),
            pending_requests: RwLock::new(HashMap::new()),
            relay_addr: RwLock::new(None),
            send_sessions: RwLock::new(HashMap::new()),
            receive_sessions: RwLock::new(HashMap::new()),
        }
    }

    /// 获取所有中继服务器地址
    fn get_relay_servers(&self) -> Vec<Multiaddr> {
        let config = self.config.read();
        let p2p_config = config.as_ref().cloned().unwrap_or_default();

        let mut servers = p2p_config.relay_servers.clone();

        if p2p_config.use_default_bootstrap {
            for addr_str in DEFAULT_RELAY_SERVERS {
                if let Ok(addr) = addr_str.parse() {
                    servers.push(addr);
                }
            }
        }

        servers
    }

    /// 设置 P2P 配置
    pub fn with_config(self, config: P2pConfig) -> Self {
        *self.config.write() = Some(config);
        self
    }

    /// 发送事件
    fn emit_event(&self, event: Event) {
        if let Some(tx) = self.event_tx.read().as_ref() {
            let _ = tx.try_send(event);
        }
    }

    /// 创建 Device
    fn create_device(&self, peer_id: PeerId, name: String, relay_addr: Option<&str>) -> Device {
        let protocol_id = ProtocolId::new(P2P_PROTOCOL_ID);

        // 构建可连接地址
        let address = if let Some(relay) = relay_addr {
            format!("{}/p2p-circuit/p2p/{}", relay, peer_id)
        } else {
            peer_id.to_string()
        };

        let peer = Peer::new(protocol_id, peer_id.to_string(), name)
            .with_device_type(DeviceType::Desktop);

        Device::new(peer, IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
    }
}

impl Default for P2pProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Protocol for P2pProtocol {
    fn info(&self) -> &ProtocolInfo {
        &self.info
    }

    async fn start(&self, _config: ProtocolConfig) -> Result<()> {
        if *self.running.read() {
            return Ok(());
        }

        info!("启动 P2P 协议...");

        // 创建 libp2p swarm
        let mut swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?
            .with_quic()
            .with_relay_client(noise::Config::new, yamux::Config::default)
            .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?
            .with_behaviour(|keypair, relay_client| {
                P2pClientBehaviour::new(relay_client, keypair)
            })
            .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(300)))
            .build();

        let local_peer_id = *swarm.local_peer_id();
        *self.local_peer_id.write() = Some(local_peer_id);
        info!("本地 Peer ID: {}", local_peer_id);

        // 监听本地地址
        let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
        swarm.listen_on(listen_addr).map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?;

        // 创建通道
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let (command_tx, mut command_rx) = mpsc::channel::<SwarmCommand>(100);
        let (event_tx, _) = mpsc::channel(100);

        *self.shutdown_tx.write() = Some(shutdown_tx);
        *self.command_tx.write() = Some(command_tx);
        *self.event_tx.write() = Some(event_tx);

        // 克隆需要的数据
        let relay_servers = self.get_relay_servers();
        let devices = Arc::new(RwLock::new(Vec::<Device>::new()));
        let devices_clone = devices.clone();
        let event_tx_clone = self.event_tx.read().clone();
        let pending_requests = Arc::new(RwLock::new(HashMap::<String, TransferRequest>::new()));
        let pending_requests_clone = pending_requests.clone();
        let relay_addr_storage = Arc::new(RwLock::new(None::<String>));
        let relay_addr_clone = relay_addr_storage.clone();

        // 接收文件的缓冲区: transfer_id -> (file_id -> data)
        let receive_buffers: Arc<RwLock<HashMap<String, HashMap<String, Vec<u8>>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let receive_buffers_clone = receive_buffers.clone();

        // 启动 swarm 事件循环
        tokio::spawn(async move {
            let mut relay_reserved = false;

            // 连接中继服务器
            for relay_addr in &relay_servers {
                info!("连接中继服务器: {}", relay_addr);
                if let Err(e) = swarm.dial(relay_addr.clone()) {
                    warn!("连接中继服务器失败: {}", e);
                }
            }

            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => {
                        info!("P2P swarm 收到关闭信号");
                        break;
                    }
                    cmd = command_rx.recv() => {
                        if let Some(cmd) = cmd {
                            match cmd {
                                SwarmCommand::Dial { addr, reply } => {
                                    let result = swarm.dial(addr).map_err(|e| anyhow::anyhow!("{}", e));
                                    let _ = reply.send(result);
                                }
                                SwarmCommand::SendRequest { peer_id, request, reply } => {
                                    let req_id = swarm.behaviour_mut().file_transfer.send_request(&peer_id, request);
                                    let _ = reply.send(req_id);
                                }
                                SwarmCommand::GetLocalPeerId { reply } => {
                                    let _ = reply.send(*swarm.local_peer_id());
                                }
                                SwarmCommand::SendFileChunk { peer_id, chunk, reply } => {
                                    let req_id = swarm.behaviour_mut().file_data.send_request(&peer_id, chunk);
                                    let _ = reply.send(req_id);
                                }
                            }
                        }
                    }
                    event = swarm.next() => {
                        if let Some(event) = event {
                            match event {
                                SwarmEvent::NewListenAddr { address, .. } => {
                                    if address.to_string().contains("p2p-circuit") {
                                        info!("✓ 中继监听地址: {}", address);
                                    } else {
                                        debug!("本地监听: {}", address);
                                    }
                                }
                                SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                                    info!("P2P 连接建立: {} ({:?})", peer_id, endpoint);

                                    // 连接到中继后请求预约
                                    if endpoint.is_dialer() && !relay_reserved {
                                        if let Some(relay_addr) = relay_servers.first() {
                                            let circuit_addr = relay_addr.clone()
                                                .with(libp2p::multiaddr::Protocol::P2pCircuit);
                                            if let Err(e) = swarm.listen_on(circuit_addr) {
                                                warn!("无法通过中继监听: {}", e);
                                            }
                                        }
                                    }
                                }
                                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                    info!("P2P 连接关闭: {}", peer_id);

                                    let mut devs = devices_clone.write();
                                    devs.retain(|d| d.id().fingerprint != peer_id.to_string());

                                    if let Some(ref tx) = event_tx_clone {
                                        let device_id = DeviceId::new(ProtocolId::new(P2P_PROTOCOL_ID), peer_id.to_string());
                                        let _ = tx.try_send(Event::device_lost(device_id));
                                    }
                                }
                                SwarmEvent::Behaviour(P2pClientBehaviourEvent::RelayClient(
                                    relay::client::Event::ReservationReqAccepted { relay_peer_id, .. },
                                )) => {
                                    relay_reserved = true;
                                    if let Some(relay_addr) = relay_servers.first() {
                                        let full_addr = format!("{}/p2p-circuit/p2p/{}", relay_addr, local_peer_id);
                                        info!("═══════════════════════════════════════════════════════");
                                        info!("✓ 中继预约成功! relay={}", relay_peer_id);
                                        info!("本机地址: {}", full_addr);
                                        info!("═══════════════════════════════════════════════════════");
                                        *relay_addr_clone.write() = Some(relay_addr.to_string());
                                    }
                                }
                                SwarmEvent::Behaviour(P2pClientBehaviourEvent::Identify(
                                    identify::Event::Received { peer_id, info, .. },
                                )) => {
                                    // 忽略中继服务器
                                    if info.agent_version.contains("rust-libp2p-server") || info.agent_version.contains("relayd") {
                                        debug!("忽略中继服务器: {}", peer_id);
                                        continue;
                                    }

                                    info!("发现节点: {} - {}", peer_id, info.agent_version);

                                    let device = {
                                        let protocol_id = ProtocolId::new(P2P_PROTOCOL_ID);
                                        let peer = Peer::new(protocol_id, peer_id.to_string(), info.agent_version.clone())
                                            .with_device_type(DeviceType::Desktop);
                                        Device::new(peer, IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
                                    };

                                    let mut devs = devices_clone.write();
                                    if !devs.iter().any(|d| d.id().fingerprint == peer_id.to_string()) {
                                        devs.push(device.clone());
                                        drop(devs);

                                        if let Some(ref tx) = event_tx_clone {
                                            let _ = tx.try_send(Event::device_discovered(device));
                                        }
                                    }
                                }
                                SwarmEvent::Behaviour(P2pClientBehaviourEvent::Ping(ping::Event { peer, result, .. })) => {
                                    match result {
                                        Ok(rtt) => debug!("Ping {} = {:?}", peer, rtt),
                                        Err(e) => debug!("Ping {} 失败: {}", peer, e),
                                    }
                                }
                                SwarmEvent::Behaviour(P2pClientBehaviourEvent::Dcutr(dcutr::Event { remote_peer_id, result })) => {
                                    match result {
                                        Ok(_) => info!("✓ DCUtR 直连成功: {}", remote_peer_id),
                                        Err(e) => debug!("DCUtR 失败: {} - {:?}", remote_peer_id, e),
                                    }
                                }
                                SwarmEvent::Behaviour(P2pClientBehaviourEvent::FileTransfer(
                                    request_response::Event::Message { peer, message }
                                )) => {
                                    match message {
                                        request_response::Message::Request { request, channel, .. } => {
                                            info!("收到文件请求: {:?} from {}", request, peer);

                                            // 创建 TransferRequest
                                            let protocol_id = ProtocolId::new(P2P_PROTOCOL_ID);
                                            let from_peer = Peer::new(protocol_id.clone(), peer.to_string(), peer.to_string())
                                                .with_device_type(DeviceType::Desktop);
                                            let from_device = Device::new(from_peer, IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

                                            let files: Vec<FileInfo> = request.files.iter().map(|f| {
                                                FileInfo {
                                                    id: f.id.clone(),
                                                    name: f.name.clone(),
                                                    size: f.size,
                                                    mime_type: f.mime_type.clone().unwrap_or_else(|| "application/octet-stream".to_string()),
                                                    hash: None,
                                                    preview: None,
                                                }
                                            }).collect();

                                            let transfer_req = TransferRequest::new(
                                                request.transfer_id.clone(),
                                                from_device,
                                                files,
                                            );

                                            pending_requests_clone.write().insert(request.transfer_id.clone(), transfer_req.clone());

                                            if let Some(ref tx) = event_tx_clone {
                                                let _ = tx.try_send(Event::transfer_requested(transfer_req));
                                            }

                                            // 发送响应
                                            let response = FileResponse {
                                                transfer_id: request.transfer_id,
                                                accepted: true,
                                                message: None,
                                            };
                                            let _ = swarm.behaviour_mut().file_transfer.send_response(channel, response);
                                        }
                                        request_response::Message::Response { response, .. } => {
                                            info!("收到文件响应: {:?}", response);
                                        }
                                    }
                                }
                                SwarmEvent::Behaviour(P2pClientBehaviourEvent::FileData(
                                    request_response::Event::Message { peer: _, message }
                                )) => {
                                    match message {
                                        request_response::Message::Request { request: chunk, channel, .. } => {
                                            info!("收到文件块: transfer={}, file={}, chunk={}/{}, size={}",
                                                chunk.transfer_id, chunk.file_id, chunk.chunk_index + 1, chunk.total_chunks, chunk.data.len());

                                            // 保存数据到缓冲区
                                            {
                                                let mut buffers = receive_buffers_clone.write();
                                                let transfer_buffers = buffers.entry(chunk.transfer_id.clone()).or_insert_with(HashMap::new);
                                                let file_buffer = transfer_buffers.entry(chunk.file_id.clone()).or_insert_with(Vec::new);
                                                file_buffer.extend_from_slice(&chunk.data);
                                            }

                                            // 发送确认
                                            let ack = FileChunkAck {
                                                transfer_id: chunk.transfer_id,
                                                file_id: chunk.file_id,
                                                chunk_index: chunk.chunk_index,
                                                success: true,
                                            };
                                            let _ = swarm.behaviour_mut().file_data.send_response(channel, ack);
                                        }
                                        request_response::Message::Response { response: ack, .. } => {
                                            info!("收到块确认: transfer={}, file={}, chunk={}, success={}",
                                                ack.transfer_id, ack.file_id, ack.chunk_index, ack.success);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            info!("P2P swarm 事件循环结束");
        });

        *self.running.write() = true;
        info!("P2P 协议已启动");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        if !*self.running.read() {
            return Ok(());
        }

        info!("停止 P2P 协议...");

        // 发送关闭信号
        if let Some(tx) = self.shutdown_tx.write().take() {
            let _ = tx.send(());
        }

        *self.running.write() = false;
        *self.command_tx.write() = None;

        Ok(())
    }

    fn is_running(&self) -> bool {
        *self.running.read()
    }

    async fn devices(&self) -> Vec<Device> {
        self.devices.read().clone()
    }

    async fn scan(&self) -> Result<()> {
        // P2P 协议通过中继发现设备
        Ok(())
    }

    async fn send(&self, intent: TransferIntent) -> Result<String> {
        let transfer_id = uuid::Uuid::new_v4().to_string();

        // 获取目标设备的 peer_id
        let peer_id_str = intent.target.fingerprint.clone();
        let peer_id: PeerId = peer_id_str.parse()
            .map_err(|e| unidrop_core::Error::Protocol(format!("Invalid peer id: {}", e)))?;

        // 构建文件请求
        let files: Vec<crate::behaviour::P2pFileInfo> = intent.files.iter().map(|path| {
            let name = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);

            crate::behaviour::P2pFileInfo {
                id: uuid::Uuid::new_v4().to_string(),
                name,
                size,
                mime_type: None,
            }
        }).collect();

        let request = FileRequest {
            transfer_id: transfer_id.clone(),
            files,
        };

        // 发送请求
        let tx = self.command_tx.read().clone();
        if let Some(tx) = tx {
            let (reply_tx, reply_rx) = oneshot::channel();
            let cmd = SwarmCommand::SendRequest {
                peer_id,
                request,
                reply: reply_tx,
            };
            tx.send(cmd).await.map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?;
            let _ = reply_rx.await;
        }

        // 创建传输会话
        let session = TransferSession::new(
            transfer_id.clone(),
            peer_id_str,
            intent.files.first()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            intent.files.iter()
                .filter_map(|p| std::fs::metadata(p).ok())
                .map(|m| m.len())
                .sum(),
        );
        self.transfers.add_session(session);

        Ok(transfer_id)
    }

    async fn accept(&self, request_id: &str, _save_dir: PathBuf) -> Result<()> {
        if let Some(_request) = self.pending_requests.write().remove(request_id) {
            info!("接受传输请求: {}", request_id);
            // TODO: 开始接收文件
        }
        Ok(())
    }

    async fn reject(&self, request_id: &str) -> Result<()> {
        if let Some(_request) = self.pending_requests.write().remove(request_id) {
            info!("拒绝传输请求: {}", request_id);
        }
        Ok(())
    }

    async fn cancel(&self, transfer_id: &str) -> Result<()> {
        self.transfers.cancel(transfer_id);
        Ok(())
    }

    fn subscribe(&self) -> mpsc::Receiver<Event> {
        let (tx, rx) = mpsc::channel(100);
        *self.event_tx.write() = Some(tx);
        rx
    }
}

/// P2P 协议工厂
pub struct P2pFactory;

impl P2pFactory {
    pub fn new() -> Self {
        Self
    }
}

impl ProtocolFactory for P2pFactory {
    fn create(&self) -> Arc<dyn Protocol> {
        Arc::new(P2pProtocol::new())
    }

    fn info(&self) -> ProtocolInfo {
        ProtocolBuilder::new(P2P_PROTOCOL_ID)
            .name("P2P Direct")
            .version("1.0.0")
            .description("点对点传输，支持 NAT 穿透")
            .priority(50)
            .build_info()
    }
}

impl Default for P2pFactory {
    fn default() -> Self {
        Self::new()
    }
}
