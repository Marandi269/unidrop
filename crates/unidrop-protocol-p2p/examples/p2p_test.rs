//! P2P 连接测试工具
//!
//! 用法：
//!   p2p_test              # 只连接中继
//!   p2p_test <peer_addr>  # 连接中继后，通过中继连接指定节点

use clap::Parser;
use futures::StreamExt;
use libp2p::{
    identify, noise, ping, relay, dcutr, tcp, yamux,
    swarm::{NetworkBehaviour, SwarmEvent, dial_opts::DialOpts},
    Multiaddr, SwarmBuilder,
};
use std::time::Duration;
use tracing::{info, warn, error};
use tracing_subscriber::EnvFilter;

/// 中继服务器地址
const RELAY_ADDR: &str = "/ip4/156.225.28.220/tcp/9001/p2p/12D3KooWCXsQB737PXEosCDxeBTd7Ze4NGsba8WJiUTddjqBkCGg";

#[derive(Parser)]
#[command(name = "p2p_test")]
#[command(about = "P2P 连接测试")]
struct Args {
    /// 要连接的对端地址 (可选)
    peer_addr: Option<String>,
}

#[derive(NetworkBehaviour)]
struct TestBehaviour {
    relay_client: relay::client::Behaviour,
    dcutr: dcutr::Behaviour,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let args = Args::parse();

    info!("启动 P2P 测试...");

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
        .with_behaviour(|keypair, relay_client| {
            TestBehaviour {
                relay_client,
                dcutr: dcutr::Behaviour::new(keypair.public().to_peer_id()),
                identify: identify::Behaviour::new(identify::Config::new(
                    "/unidrop/1.0.0".to_string(),
                    keypair.public(),
                )),
                ping: ping::Behaviour::new(ping::Config::default().with_interval(Duration::from_secs(10))),
            }
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    let local_peer_id = *swarm.local_peer_id();
    info!("本地 Peer ID: {}", local_peer_id);

    // 监听本地
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 连接中继服务器
    let relay_addr: Multiaddr = RELAY_ADDR.parse()?;
    info!("连接中继服务器: {}", relay_addr);
    swarm.dial(relay_addr.clone())?;

    let mut relay_reserved = false;
    let peer_to_dial = args.peer_addr.clone();

    // 事件循环
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                if address.to_string().contains("p2p-circuit") {
                    info!("✓ 中继监听地址: {}/p2p/{}", address, local_peer_id);
                } else {
                    info!("本地监听: {}", address);
                }
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                info!("✓ 连接建立: {} ({:?})", peer_id, endpoint);

                // 如果连接到中继，请求预约
                if endpoint.is_dialer() && !relay_reserved {
                    info!("请求中继预约...");

                    // 通过中继监听
                    let relay_listen_addr = RELAY_ADDR.parse::<Multiaddr>()?
                        .with(libp2p::multiaddr::Protocol::P2pCircuit);

                    if let Err(e) = swarm.listen_on(relay_listen_addr.clone()) {
                        warn!("无法通过中继监听: {}", e);
                    } else {
                        info!("请求通过中继监听: {}", relay_listen_addr);
                    }
                }
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                warn!("连接关闭: {} ({:?})", peer_id, cause);
            }
            SwarmEvent::Behaviour(TestBehaviourEvent::RelayClient(
                relay::client::Event::ReservationReqAccepted { relay_peer_id, renewal, .. },
            )) => {
                relay_reserved = true;
                info!("═══════════════════════════════════════════════════════");
                info!("✓ 中继预约成功! relay={}, renewal={}", relay_peer_id, renewal);
                info!("其他节点可通过以下地址连接本机:");
                info!("  {}/p2p-circuit/p2p/{}", RELAY_ADDR, local_peer_id);
                info!("═══════════════════════════════════════════════════════");

                // 如果指定了对端地址，连接它
                if let Some(ref addr_str) = peer_to_dial {
                    info!("尝试连接对端: {}", addr_str);
                    match addr_str.parse::<Multiaddr>() {
                        Ok(addr) => {
                            if let Err(e) = swarm.dial(addr) {
                                error!("连接对端失败: {}", e);
                            }
                        }
                        Err(e) => error!("解析地址失败: {}", e),
                    }
                }
            }
            SwarmEvent::Behaviour(TestBehaviourEvent::Identify(
                identify::Event::Received { peer_id, info, .. },
            )) => {
                info!("识别节点: {} - {} ({})", peer_id, info.agent_version, info.protocol_version);
            }
            SwarmEvent::Behaviour(TestBehaviourEvent::Ping(ping::Event { peer, result, .. })) => {
                match result {
                    Ok(rtt) => info!("Ping {} = {:?}", peer, rtt),
                    Err(e) => warn!("Ping {} 失败: {}", peer, e),
                }
            }
            SwarmEvent::Behaviour(TestBehaviourEvent::Dcutr(dcutr::Event {
                remote_peer_id, result
            })) => {
                match result {
                    Ok(_) => info!("✓ DCUtR 直连成功: {}", remote_peer_id),
                    Err(e) => warn!("DCUtR 失败: {} - {:?}", remote_peer_id, e),
                }
            }
            SwarmEvent::IncomingConnection { local_addr, send_back_addr, .. } => {
                info!("收到连接: {} <- {}", local_addr, send_back_addr);
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                error!("连接失败 {:?}: {}", peer_id, error);
            }
            _ => {}
        }
    }
}
