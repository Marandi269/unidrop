//! 设备模型 - 协议无关的设备表示

use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::IpAddr;

use crate::ProtocolId;

/// 设备唯一标识符
///
/// 由协议ID + 设备指纹组成，确保全局唯一
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeviceId {
    /// 来源协议
    pub protocol: ProtocolId,
    /// 协议内唯一标识（如 LocalSend 的 fingerprint）
    pub fingerprint: String,
}

impl DeviceId {
    pub fn new(protocol: ProtocolId, fingerprint: impl Into<String>) -> Self {
        Self {
            protocol,
            fingerprint: fingerprint.into(),
        }
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.protocol, self.fingerprint)
    }
}

/// 设备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Mobile,
    #[default]
    Desktop,
    Tablet,
    Web,
    Server,
    Unknown,
}

impl DeviceType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "mobile" | "phone" => Self::Mobile,
            "desktop" | "laptop" => Self::Desktop,
            "tablet" | "ipad" => Self::Tablet,
            "web" | "browser" => Self::Web,
            "server" | "headless" => Self::Server,
            _ => Self::Unknown,
        }
    }
}

/// 对等设备信息 - 协议无关的统一表示
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// 全局唯一 ID
    pub id: DeviceId,
    /// 显示名称
    pub name: String,
    /// 设备类型
    pub device_type: DeviceType,
    /// 设备型号（可选）
    pub model: Option<String>,
    /// 来源协议
    pub protocol: ProtocolId,
    /// 协议版本
    pub protocol_version: String,
}

impl Peer {
    /// 创建新的 Peer
    pub fn new(
        protocol: ProtocolId,
        fingerprint: String,
        name: String,
    ) -> Self {
        Self {
            id: DeviceId::new(protocol.clone(), fingerprint),
            name,
            device_type: DeviceType::default(),
            model: None,
            protocol,
            protocol_version: String::new(),
        }
    }

    /// 设置设备类型
    pub fn with_device_type(mut self, device_type: DeviceType) -> Self {
        self.device_type = device_type;
        self
    }

    /// 设置型号
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// 设置协议版本
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.protocol_version = version.into();
        self
    }
}

/// 在线设备 - 包含网络信息的 Peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// 设备基本信息
    #[serde(flatten)]
    pub peer: Peer,
    /// IP 地址
    pub ip: IpAddr,
    /// 端口
    pub port: u16,
    /// 最后在线时间戳（Unix 秒）
    pub last_seen: u64,
}

impl Device {
    pub fn new(peer: Peer, ip: IpAddr, port: u16) -> Self {
        Self {
            peer,
            ip,
            port,
            last_seen: current_timestamp(),
        }
    }

    /// 获取设备 ID
    pub fn id(&self) -> &DeviceId {
        &self.peer.id
    }

    /// 获取显示名称
    pub fn name(&self) -> &str {
        &self.peer.name
    }

    /// 获取协议 ID
    pub fn protocol(&self) -> &ProtocolId {
        &self.peer.protocol
    }

    /// 获取地址字符串
    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    /// 更新最后在线时间
    pub fn touch(&mut self) {
        self.last_seen = current_timestamp();
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
