//! UniDrop 中转服务器
//!
//! 提供 NAT 穿透的中转服务，支持:
//! - Circuit Relay v2 协议
//! - 打洞协调 (DCUtR)
//! - 节点发现

use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use libp2p::{
    identify, noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, SwarmBuilder,
};
use std::time::Duration;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "unidrop-relay")]
#[command(about = "UniDrop relay server for NAT traversal")]
struct Args {
    /// 监听端口
    #[arg(short, long, default_value = "4001")]
    port: u16,

    /// 外部地址 (用于公告)
    #[arg(short, long)]
    external_addr: Option<Multiaddr>,

    /// 每个电路允许的最大字节数 (MB)
    #[arg(long, default_value = "100")]
    max_circuit_mb: u64,

    /// 每个电路允许的最大持续时间 (秒)
    #[arg(long, default_value = "600")]
    max_circuit_duration: u64,
}

/// 中转服务器行为
#[derive(NetworkBehaviour)]
struct RelayServerBehaviour {
    relay: relay::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    info!("启动 UniDrop 中转服务器...");
    info!(
        "中继配置: max_circuit_bytes={}MB, max_circuit_duration={}s",
        args.max_circuit_mb, args.max_circuit_duration
    );

    // 提取配置参数供闭包使用
    let max_circuit_bytes = args.max_circuit_mb * 1024 * 1024;
    let max_circuit_duration = Duration::from_secs(args.max_circuit_duration);

    // 创建 libp2p swarm
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|keypair| {
            // 配置中继参数
            let relay_config = relay::Config {
                max_circuit_bytes,
                max_circuit_duration,
                ..Default::default()
            };

            RelayServerBehaviour {
                relay: relay::Behaviour::new(
                    keypair.public().to_peer_id(),
                    relay_config,
                ),
                ping: ping::Behaviour::new(ping::Config::default()),
                identify: identify::Behaviour::new(identify::Config::new(
                    "/unidrop-relay/1.0.0".to_string(),
                    keypair.public(),
                )),
            }
        })?
        .build();

    // 监听地址
    let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", args.port).parse()?;
    swarm.listen_on(listen_addr.clone())?;

    // QUIC 监听
    let quic_addr: Multiaddr = format!("/ip4/0.0.0.0/udp/{}/quic-v1", args.port).parse()?;
    swarm.listen_on(quic_addr)?;

    // 添加外部地址
    if let Some(external) = args.external_addr {
        swarm.add_external_address(external);
    }

    let local_peer_id = *swarm.local_peer_id();
    info!("本地 Peer ID: {}", local_peer_id);
    info!("监听端口: {}", args.port);

    // 事件循环
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("监听地址: {}/p2p/{}", address, local_peer_id);
            }
            SwarmEvent::Behaviour(RelayServerBehaviourEvent::Relay(
                relay::Event::ReservationReqAccepted { src_peer_id, .. },
            )) => {
                info!("接受来自 {} 的中转预约", src_peer_id);
            }
            SwarmEvent::Behaviour(RelayServerBehaviourEvent::Relay(
                relay::Event::CircuitReqAccepted {
                    src_peer_id,
                    dst_peer_id,
                },
            )) => {
                info!("建立中转连接: {} <-> {}", src_peer_id, dst_peer_id);
            }
            SwarmEvent::Behaviour(RelayServerBehaviourEvent::Identify(
                identify::Event::Received { peer_id, info, .. },
            )) => {
                info!(
                    "识别节点 {}: {} ({})",
                    peer_id,
                    info.protocol_version,
                    info.agent_version
                );
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("建立连接: {}", peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                info!("关闭连接: {} (原因: {:?})", peer_id, cause);
            }
            SwarmEvent::IncomingConnection { local_addr, .. } => {
                info!("收到连接请求: {}", local_addr);
            }
            _ => {}
        }
    }
}
