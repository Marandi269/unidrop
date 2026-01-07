//! UniDrop P2P 协议 - 支持 NAT 穿透的点对点传输
//!
//! 基于 libp2p 实现，支持:
//! - 直连传输 (TCP/QUIC)
//! - NAT 打洞 (DCUtR)
//! - 中转传输 (Circuit Relay v2)

mod behaviour;
mod protocol;
mod transfer;

pub use protocol::{P2pConfig, P2pFactory, P2pProtocol, P2P_PROTOCOL_ID};
