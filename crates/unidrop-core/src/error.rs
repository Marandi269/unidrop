//! 统一错误类型

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// 统一错误类型 - 协议无关
#[derive(Error, Debug)]
pub enum Error {
    // === 网络层错误 ===
    #[error("Network error: {0}")]
    Network(String),

    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("Request timeout")]
    Timeout,

    // === 发现层错误 ===
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Discovery failed: {0}")]
    Discovery(String),

    // === 传输层错误 ===
    #[error("Transfer rejected")]
    Rejected,

    #[error("Transfer cancelled")]
    Cancelled,

    #[error("Transfer failed: {0}")]
    TransferFailed(String),

    #[error("Invalid session: {0}")]
    InvalidSession(String),

    // === 协议层错误 ===
    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Protocol not found: {0}")]
    ProtocolNotFound(String),

    #[error("Protocol not supported: {0}")]
    ProtocolNotSupported(String),

    // === 文件系统错误 ===
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // === 配置错误 ===
    #[error("Configuration error: {0}")]
    Config(String),

    // === 其他 ===
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// 是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(self, Error::Timeout | Error::Network(_))
    }

    /// 是否是用户主动取消
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Error::Cancelled | Error::Rejected)
    }
}
