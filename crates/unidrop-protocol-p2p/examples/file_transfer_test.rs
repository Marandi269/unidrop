//! P2P 文件传输测试工具
//!
//! 用法：
//!   发送方: file_transfer_test send <file_path> <peer_addr>
//!   接收方: file_transfer_test receive

use clap::{Parser, Subcommand};
use futures::StreamExt;
use libp2p::{
    dcutr, identify, noise, ping, relay,
    request_response::{self, cbor::Behaviour as CborBehaviour, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, StreamProtocol, SwarmBuilder,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

/// 中继服务器地址 (unidrop-relay: 500MB limit, 1800s duration)
const RELAY_ADDR: &str =
    "/ip4/156.225.28.220/tcp/9001/p2p/12D3KooWSwNjkoZaVMYYbZECzMGLijHQqrrt8H6x33YWfEunN6ym";

#[derive(Parser)]
#[command(name = "file_transfer_test")]
#[command(about = "P2P 文件传输测试")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 发送文件
    Send {
        /// 要发送的文件路径
        file: PathBuf,
        /// 接收方地址
        peer_addr: String,
    },
    /// 接收文件
    Receive,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct P2pFileInfo {
    id: String,
    name: String,
    size: u64,
    mime_type: Option<String>,
}

/// 文件传输请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileRequest {
    transfer_id: String,
    files: Vec<P2pFileInfo>,
}

/// 文件传输响应
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileResponse {
    transfer_id: String,
    accepted: bool,
    message: Option<String>,
}

/// 文件数据块（分块传输）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileChunk {
    transfer_id: String,
    file_id: String,
    file_name: String,
    chunk_index: u64,
    total_chunks: u64,
    data: Vec<u8>,
}

/// 文件数据确认
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileChunkAck {
    transfer_id: String,
    file_id: String,
    chunk_index: u64,
    success: bool,
}

/// 块大小 (64KB，确保在 128KB 中继限制内)
const CHUNK_SIZE: usize = 64 * 1024;

#[derive(NetworkBehaviour)]
struct TransferBehaviour {
    relay_client: relay::client::Behaviour,
    dcutr: dcutr::Behaviour,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    file_transfer: CborBehaviour<FileRequest, FileResponse>,
    file_data: CborBehaviour<FileChunk, FileChunkAck>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    // 创建 swarm
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|keypair, relay_client| TransferBehaviour {
            relay_client,
            dcutr: dcutr::Behaviour::new(keypair.public().to_peer_id()),
            identify: identify::Behaviour::new(identify::Config::new(
                "/unidrop/1.0.0".to_string(),
                keypair.public(),
            )),
            ping: ping::Behaviour::new(
                ping::Config::default().with_interval(Duration::from_secs(15)),
            ),
            file_transfer: CborBehaviour::new(
                [(
                    StreamProtocol::new("/unidrop/file/1.0.0"),
                    ProtocolSupport::Full,
                )],
                request_response::Config::default(),
            ),
            file_data: CborBehaviour::new(
                [(
                    StreamProtocol::new("/unidrop/data/1.0.0"),
                    ProtocolSupport::Full,
                )],
                request_response::Config::default().with_request_timeout(Duration::from_secs(60)),
            ),
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(300)))
        .build();

    let local_peer_id = *swarm.local_peer_id();
    info!("本地 Peer ID: {}", local_peer_id);

    // 监听本地
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 连接中继服务器
    let relay_addr: Multiaddr = RELAY_ADDR.parse()?;
    info!("连接中继服务器: {}", relay_addr);
    swarm.dial(relay_addr.clone())?;

    match args.command {
        Commands::Send { file, peer_addr } => {
            run_sender(&mut swarm, file, peer_addr, local_peer_id).await?;
        }
        Commands::Receive => {
            run_receiver(&mut swarm, local_peer_id).await?;
        }
    }

    Ok(())
}

/// 运行发送端
async fn run_sender(
    swarm: &mut libp2p::Swarm<TransferBehaviour>,
    file_path: PathBuf,
    peer_addr: String,
    local_peer_id: PeerId,
) -> anyhow::Result<()> {
    let file_name = file_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let file_size = tokio::fs::metadata(&file_path).await?.len();

    info!("准备发送文件: {} ({} bytes)", file_name, file_size);

    let mut relay_reserved = false;
    let mut peer_connected = false;
    let mut target_peer_id: Option<PeerId> = None;

    // 生成传输信息
    let transfer_id = uuid::Uuid::new_v4().to_string();
    let file_id = uuid::Uuid::new_v4().to_string();

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                if address.to_string().contains("p2p-circuit") {
                    info!("✓ 中继监听地址: {}", address);
                } else {
                    debug!("本地监听: {}", address);
                }
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                info!("✓ 连接建立: {}", peer_id);

                // 请求中继预约
                if endpoint.is_dialer() && !relay_reserved {
                    let relay_listen_addr =
                        RELAY_ADDR.parse::<Multiaddr>()?.with(libp2p::multiaddr::Protocol::P2pCircuit);
                    if swarm.listen_on(relay_listen_addr).is_ok() {
                        info!("请求中继预约...");
                    }
                }
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::RelayClient(
                relay::client::Event::ReservationReqAccepted { .. },
            )) => {
                relay_reserved = true;
                info!("═══════════════════════════════════════════════════════");
                info!("✓ 中继预约成功!");
                info!("本机地址: {}/p2p-circuit/p2p/{}", RELAY_ADDR, local_peer_id);
                info!("═══════════════════════════════════════════════════════");

                // 连接对端
                info!("连接对端: {}", peer_addr);
                let addr: Multiaddr = peer_addr.parse()?;
                swarm.dial(addr)?;
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::Identify(
                identify::Event::Received { peer_id, info, .. },
            )) => {
                // 从目标地址提取 peer id (获取最后一个 P2p 协议，即真正的目标)
                let target_peer_id_from_addr: Option<PeerId> = peer_addr
                    .parse::<Multiaddr>()
                    .ok()
                    .and_then(|addr| {
                        addr.iter()
                            .filter_map(|p| {
                                if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
                                    Some(peer_id)
                                } else {
                                    None
                                }
                            })
                            .last()  // 获取最后一个（目标 peer，不是 relay）
                    });

                // 忽略中继服务器（检查是否是目标 peer）
                let is_target = target_peer_id_from_addr
                    .map(|target| peer_id == target)
                    .unwrap_or(false);

                if !is_target {
                    // 不是目标，跳过（可能是 relay 节点）
                    debug!("跳过非目标节点: {} - {}", peer_id, info.agent_version);
                    continue;
                }

                info!("识别目标节点: {} - {}", peer_id, info.agent_version);

                // 找到目标节点，发送文件请求
                if !peer_connected {
                    peer_connected = true;
                    target_peer_id = Some(peer_id);

                    let request = FileRequest {
                        transfer_id: transfer_id.clone(),
                        files: vec![P2pFileInfo {
                            id: file_id.clone(),
                            name: file_name.clone(),
                            size: file_size,
                            mime_type: None,
                        }],
                    };

                    info!("发送文件请求: {:?}", request);
                    swarm
                        .behaviour_mut()
                        .file_transfer
                        .send_request(&peer_id, request);
                }
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::FileTransfer(
                request_response::Event::Message { message, .. },
            )) => {
                if let request_response::Message::Response { response, .. } = message {
                    info!("收到响应: {:?}", response);

                    if response.accepted {
                        info!("对方接受了文件，开始传输...");

                        // 开始发送文件数据
                        if let Some(peer_id) = target_peer_id {
                            send_file(
                                swarm,
                                peer_id,
                                &transfer_id,
                                &file_id,
                                &file_name,
                                &file_path,
                            )
                            .await?;
                            info!("文件传输完成!");
                            return Ok(());
                        }
                    } else {
                        warn!("对方拒绝了文件: {:?}", response.message);
                        return Ok(());
                    }
                }
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::FileData(
                request_response::Event::Message { message, .. },
            )) => {
                if let request_response::Message::Response { response: ack, .. } = message {
                    debug!("收到确认: file={}, success={}", ack.file_id, ack.success);
                }
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                error!("连接失败 {:?}: {}", peer_id, error);
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                warn!("连接关闭: {}", peer_id);
            }
            _ => {}
        }
    }
}

/// 发送文件数据（分块）
async fn send_file(
    swarm: &mut libp2p::Swarm<TransferBehaviour>,
    peer_id: PeerId,
    transfer_id: &str,
    file_id: &str,
    file_name: &str,
    file_path: &PathBuf,
) -> anyhow::Result<()> {
    // 读取整个文件
    let file_data_bytes = tokio::fs::read(file_path).await?;
    let file_size = file_data_bytes.len();
    let total_chunks = (file_size + CHUNK_SIZE - 1) / CHUNK_SIZE;

    info!("发送文件: {} bytes, {} 块", file_size, total_chunks);

    // 分块发送，每个块发送后等待确认
    for chunk_index in 0..total_chunks {
        let start = chunk_index * CHUNK_SIZE;
        let end = std::cmp::min(start + CHUNK_SIZE, file_size);
        let chunk_data = file_data_bytes[start..end].to_vec();

        info!("发送块 {}/{}: {} bytes", chunk_index + 1, total_chunks, chunk_data.len());

        let chunk = FileChunk {
            transfer_id: transfer_id.to_string(),
            file_id: file_id.to_string(),
            file_name: file_name.to_string(),
            chunk_index: chunk_index as u64,
            total_chunks: total_chunks as u64,
            data: chunk_data,
        };

        swarm
            .behaviour_mut()
            .file_data
            .send_request(&peer_id, chunk);

        // 等待这个块的确认
        let expected_chunk = chunk_index as u64;
        loop {
            match tokio::time::timeout(Duration::from_secs(30), swarm.next()).await {
                Ok(Some(SwarmEvent::Behaviour(TransferBehaviourEvent::FileData(
                    request_response::Event::Message { message, .. },
                )))) => {
                    if let request_response::Message::Response { response: ack, .. } = message {
                        if ack.chunk_index == expected_chunk {
                            debug!("块 {} 确认: success={}", ack.chunk_index, ack.success);
                            if !ack.success {
                                anyhow::bail!("块 {} 传输失败", chunk_index);
                            }
                            break;
                        }
                    }
                }
                Ok(Some(_)) => {
                    // 处理其他事件
                }
                Ok(None) => {
                    anyhow::bail!("Swarm closed");
                }
                Err(_) => {
                    anyhow::bail!("等待块 {} 确认超时", chunk_index);
                }
            }
        }
    }

    info!("所有 {} 块发送完成", total_chunks);
    Ok(())
}

/// 运行接收端
async fn run_receiver(
    swarm: &mut libp2p::Swarm<TransferBehaviour>,
    local_peer_id: PeerId,
) -> anyhow::Result<()> {
    let mut relay_reserved = false;
    let mut _file_infos: HashMap<String, HashMap<String, P2pFileInfo>> = HashMap::new();
    // 接收缓冲区: transfer_id -> file_id -> (file_name, total_chunks, data)
    let mut receive_buffers: HashMap<String, HashMap<String, (String, u64, Vec<u8>)>> = HashMap::new();

    info!("启动接收模式，等待文件...");

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                if address.to_string().contains("p2p-circuit") {
                    info!("✓ 中继监听地址: {}", address);
                } else {
                    debug!("本地监听: {}", address);
                }
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                info!("✓ 连接建立: {}", peer_id);

                if endpoint.is_dialer() && !relay_reserved {
                    let relay_listen_addr =
                        RELAY_ADDR.parse::<Multiaddr>()?.with(libp2p::multiaddr::Protocol::P2pCircuit);
                    if swarm.listen_on(relay_listen_addr).is_ok() {
                        info!("请求中继预约...");
                    }
                }
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::RelayClient(
                relay::client::Event::ReservationReqAccepted { .. },
            )) => {
                relay_reserved = true;
                info!("═══════════════════════════════════════════════════════");
                info!("✓ 中继预约成功!");
                info!("本机地址: {}/p2p-circuit/p2p/{}", RELAY_ADDR, local_peer_id);
                info!("═══════════════════════════════════════════════════════");
                info!("等待其他节点连接...");
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::Identify(
                identify::Event::Received { peer_id, info, .. },
            )) => {
                if !info.agent_version.contains("rust-libp2p-server")
                    && !info.agent_version.contains("relayd")
                {
                    info!("识别节点: {} - {}", peer_id, info.agent_version);
                }
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::FileTransfer(
                request_response::Event::Message { peer: _, message },
            )) => {
                if let request_response::Message::Request {
                    request, channel, ..
                } = message
                {
                    info!("收到文件请求: {:?}", request);

                    // 保存文件信息
                    let mut infos = HashMap::new();
                    for file in &request.files {
                        info!(
                            "  - {} ({} bytes)",
                            file.name, file.size
                        );
                        infos.insert(file.id.clone(), file.clone());
                    }
                    _file_infos.insert(request.transfer_id.clone(), infos);

                    // 自动接受
                    let response = FileResponse {
                        transfer_id: request.transfer_id,
                        accepted: true,
                        message: None,
                    };
                    swarm
                        .behaviour_mut()
                        .file_transfer
                        .send_response(channel, response)
                        .ok();

                    info!("已接受文件请求");
                }
            }
            SwarmEvent::Behaviour(TransferBehaviourEvent::FileData(
                request_response::Event::Message { message, .. },
            )) => {
                if let request_response::Message::Request {
                    request: chunk,
                    channel,
                    ..
                } = message
                {
                    info!(
                        "收到块 {}/{}: file={}, size={}",
                        chunk.chunk_index + 1,
                        chunk.total_chunks,
                        chunk.file_name,
                        chunk.data.len()
                    );

                    // 保存数据到缓冲区
                    let transfer_buffers = receive_buffers
                        .entry(chunk.transfer_id.clone())
                        .or_insert_with(HashMap::new);
                    let (name, total, file_buffer) = transfer_buffers
                        .entry(chunk.file_id.clone())
                        .or_insert_with(|| (chunk.file_name.clone(), chunk.total_chunks, Vec::new()));
                    file_buffer.extend_from_slice(&chunk.data);

                    let is_last = chunk.chunk_index + 1 == *total;

                    // 发送确认
                    let ack = FileChunkAck {
                        transfer_id: chunk.transfer_id.clone(),
                        file_id: chunk.file_id.clone(),
                        chunk_index: chunk.chunk_index,
                        success: true,
                    };
                    swarm
                        .behaviour_mut()
                        .file_data
                        .send_response(channel, ack)
                        .ok();

                    // 如果是最后一块，保存文件
                    if is_last {
                        let save_path = format!("/tmp/{}", name);
                        info!("═══════════════════════════════════════════════════════");
                        info!("✓ 文件接收完成: {} ({} bytes)", name, file_buffer.len());
                        info!("═══════════════════════════════════════════════════════");

                        if let Err(e) = tokio::fs::write(&save_path, &file_buffer).await {
                            error!("保存文件失败: {}", e);
                        } else {
                            info!("文件已保存到: {}", save_path);
                        }

                        // 清理缓冲区
                        transfer_buffers.remove(&chunk.file_id);
                    }
                }
            }
            SwarmEvent::IncomingConnection {
                local_addr,
                send_back_addr,
                ..
            } => {
                info!("收到连接: {} <- {}", local_addr, send_back_addr);
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                warn!("连接关闭: {}", peer_id);
            }
            _ => {}
        }
    }
}
