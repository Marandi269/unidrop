//! LocalSend 协议实现
//!
//! 实现 LocalSend v2 协议，与官方 LocalSend 客户端兼容。
//!
//! 协议特点：
//! - mDNS 设备发现（224.0.0.167:53317）
//! - HTTPS REST API
//! - 自签名证书
//! - 支持文件和文本传输

mod cert;
mod client;
mod discovery;
mod models;
mod multicast;
mod protocol;
pub mod quic;
mod server;

pub use protocol::{LocalSendProtocol, LocalSendFactory};

/// LocalSend 协议 ID
pub const PROTOCOL_ID: &str = "localsend";

/// LocalSend 协议版本
pub const PROTOCOL_VERSION: &str = "2.1";

/// 默认端口
pub const DEFAULT_PORT: u16 = 53317;

/// mDNS 服务类型
pub const SERVICE_TYPE: &str = "_localsend._tcp.local.";

/// 组播地址
pub const MULTICAST_ADDR: &str = "224.0.0.167";
