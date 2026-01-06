//! 事件系统 - 协议无关的事件类型

use crate::{Device, DeviceId, TransferProgress, TransferRequest};

/// 事件类型
#[derive(Debug, Clone)]
pub enum EventKind {
    // === 设备事件 ===
    /// 发现新设备
    DeviceDiscovered(Device),
    /// 设备离线
    DeviceLost(DeviceId),
    /// 设备信息更新
    DeviceUpdated(Device),

    // === 传输事件 ===
    /// 收到传输请求
    TransferRequested(TransferRequest),
    /// 传输进度更新
    TransferProgress(TransferProgress),
    /// 传输完成
    TransferCompleted { transfer_id: String },
    /// 传输失败
    TransferFailed { transfer_id: String, error: String },

    // === 系统事件 ===
    /// 协议已启动
    ProtocolStarted { protocol: String },
    /// 协议已停止
    ProtocolStopped { protocol: String },
    /// 错误
    Error { source: String, message: String },
}

/// 统一事件结构
#[derive(Debug, Clone)]
pub struct Event {
    /// 事件类型
    pub kind: EventKind,
    /// 时间戳（Unix 毫秒）
    pub timestamp: u64,
    /// 来源协议（可选）
    pub protocol: Option<String>,
}

impl Event {
    pub fn new(kind: EventKind) -> Self {
        Self {
            kind,
            timestamp: current_timestamp_ms(),
            protocol: None,
        }
    }

    pub fn with_protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }

    // === 便捷构造方法 ===

    pub fn device_discovered(device: Device) -> Self {
        let protocol = device.protocol().to_string();
        Self::new(EventKind::DeviceDiscovered(device)).with_protocol(protocol)
    }

    pub fn device_lost(id: DeviceId) -> Self {
        let protocol = id.protocol.to_string();
        Self::new(EventKind::DeviceLost(id)).with_protocol(protocol)
    }

    pub fn transfer_requested(request: TransferRequest) -> Self {
        let protocol = request.from.protocol().to_string();
        Self::new(EventKind::TransferRequested(request)).with_protocol(protocol)
    }

    pub fn transfer_progress(progress: TransferProgress) -> Self {
        Self::new(EventKind::TransferProgress(progress))
    }

    pub fn transfer_completed(transfer_id: impl Into<String>) -> Self {
        Self::new(EventKind::TransferCompleted {
            transfer_id: transfer_id.into(),
        })
    }

    pub fn transfer_failed(transfer_id: impl Into<String>, error: impl Into<String>) -> Self {
        Self::new(EventKind::TransferFailed {
            transfer_id: transfer_id.into(),
            error: error.into(),
        })
    }

    pub fn error(source: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(EventKind::Error {
            source: source.into(),
            message: message.into(),
        })
    }
}

fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
