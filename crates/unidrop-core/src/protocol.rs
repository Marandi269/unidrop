//! 协议抽象 - 所有协议实现必须实现的 trait
//!
//! 这是整个架构的核心，定义了协议的统一接口。
//! 上层业务代码通过这个 trait 与协议交互，不关心具体实现。

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{Device, DeviceId, Event, Result, TransferIntent, TransferProgress, TransferRequest};

/// 协议标识符
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProtocolId(pub String);

impl ProtocolId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProtocolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ProtocolId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// 协议元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// 协议 ID
    pub id: ProtocolId,
    /// 协议显示名称
    pub name: String,
    /// 协议版本
    pub version: String,
    /// 协议描述
    pub description: String,
    /// 是否支持当前平台
    pub supported: bool,
    /// 优先级（用于多协议选择，数字越大优先级越高）
    pub priority: u32,
}

/// 协议配置 - 传递给协议实现的配置
#[derive(Debug, Clone)]
pub struct ProtocolConfig {
    /// 本机显示名称
    pub device_name: String,
    /// 监听端口（0 表示自动分配）
    pub port: u16,
    /// 文件保存目录
    pub save_dir: PathBuf,
    /// 是否启用加密
    pub encryption: bool,
    /// 可选 PIN 码
    pub pin: Option<String>,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            device_name: "UniDrop".to_string(),
            port: 0,
            save_dir: std::env::temp_dir(),
            encryption: true,
            pin: None,
        }
    }
}

/// 协议 trait - 所有协议实现必须实现此 trait
///
/// 设计原则：
/// 1. 协议无关性：上层业务只依赖此 trait
/// 2. 事件驱动：通过事件通道通知上层
/// 3. 异步安全：所有方法都是异步且线程安全的
#[async_trait]
pub trait Protocol: Send + Sync {
    // === 元信息 ===

    /// 获取协议信息
    fn info(&self) -> &ProtocolInfo;

    /// 获取协议 ID
    fn id(&self) -> &ProtocolId {
        &self.info().id
    }

    // === 生命周期 ===

    /// 启动协议服务
    ///
    /// 包括：设备发现服务、传输服务器等
    async fn start(&self, config: ProtocolConfig) -> Result<()>;

    /// 停止协议服务
    async fn stop(&self) -> Result<()>;

    /// 协议是否正在运行
    fn is_running(&self) -> bool;

    // === 设备发现 ===

    /// 获取当前在线设备列表
    async fn devices(&self) -> Vec<Device>;

    /// 根据 ID 获取设备
    async fn device(&self, id: &DeviceId) -> Option<Device> {
        self.devices()
            .await
            .into_iter()
            .find(|d| d.id() == id)
    }

    /// 主动扫描一次
    async fn scan(&self) -> Result<()>;

    // === 传输操作 ===

    /// 发送文件到设备
    ///
    /// 返回传输 ID，进度通过事件通道推送
    async fn send(&self, intent: TransferIntent) -> Result<String>;

    /// 使用 QUIC 发送文件（可选实现）
    ///
    /// 默认返回不支持错误，协议可以覆盖此方法提供 QUIC 支持
    async fn send_quic(&self, _intent: TransferIntent) -> Result<String> {
        Err(crate::Error::Protocol("QUIC transport not supported by this protocol".into()))
    }

    /// 接受传输请求
    async fn accept(&self, request_id: &str, save_dir: PathBuf) -> Result<()>;

    /// 拒绝传输请求
    async fn reject(&self, request_id: &str) -> Result<()>;

    /// 取消传输
    async fn cancel(&self, transfer_id: &str) -> Result<()>;

    // === 事件订阅 ===

    /// 订阅事件流
    ///
    /// 返回事件接收端，调用方通过此通道接收事件
    fn subscribe(&self) -> mpsc::Receiver<Event>;
}

/// 协议工厂 trait - 用于创建协议实例
pub trait ProtocolFactory: Send + Sync {
    /// 创建协议实例
    fn create(&self) -> Arc<dyn Protocol>;

    /// 获取协议信息（不创建实例）
    fn info(&self) -> ProtocolInfo;
}

/// 协议构建器 - 便捷的协议创建方式
pub struct ProtocolBuilder {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub priority: u32,
}

impl ProtocolBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: String::new(),
            version: "1.0".to_string(),
            description: String::new(),
            priority: 0,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn build_info(self) -> ProtocolInfo {
        ProtocolInfo {
            id: ProtocolId::new(&self.id),
            name: if self.name.is_empty() {
                self.id.clone()
            } else {
                self.name
            },
            version: self.version,
            description: self.description,
            supported: true,
            priority: self.priority,
        }
    }
}
