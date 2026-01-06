//! 传输相关类型 - 协议无关

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{Device, DeviceId};

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件 ID（会话内唯一）
    pub id: String,
    /// 文件名
    pub name: String,
    /// 文件大小（字节）
    pub size: u64,
    /// MIME 类型
    pub mime_type: String,
    /// SHA256 哈希（可选）
    pub hash: Option<String>,
    /// 缩略图预览 Base64（可选）
    pub preview: Option<String>,
}

impl FileInfo {
    pub fn new(id: impl Into<String>, name: impl Into<String>, size: u64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            size,
            mime_type: "application/octet-stream".to_string(),
            hash: None,
            preview: None,
        }
    }

    pub fn with_mime(mut self, mime: impl Into<String>) -> Self {
        self.mime_type = mime.into();
        self
    }
}

/// 传输请求 - 收到的入站传输
#[derive(Debug, Clone)]
pub struct TransferRequest {
    /// 请求 ID
    pub id: String,
    /// 发送方设备
    pub from: Device,
    /// 文件列表
    pub files: Vec<FileInfo>,
    /// 总大小
    pub total_size: u64,
    /// 附加消息
    pub message: Option<String>,
    /// 协议特定数据（用于协议层处理）
    pub protocol_data: Option<Vec<u8>>,
}

impl TransferRequest {
    pub fn new(id: impl Into<String>, from: Device, files: Vec<FileInfo>) -> Self {
        let total_size = files.iter().map(|f| f.size).sum();
        Self {
            id: id.into(),
            from,
            files,
            total_size,
            message: None,
            protocol_data: None,
        }
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// 传输意图 - 发起的出站传输
#[derive(Debug, Clone)]
pub struct TransferIntent {
    /// 目标设备 ID
    pub target: DeviceId,
    /// 要发送的文件路径
    pub files: Vec<PathBuf>,
    /// 附加消息
    pub message: Option<String>,
}

impl TransferIntent {
    pub fn new(target: DeviceId, files: Vec<PathBuf>) -> Self {
        Self {
            target,
            files,
            message: None,
        }
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }
}

/// 传输状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferState {
    /// 等待接收方确认
    Pending,
    /// 传输中
    Transferring,
    /// 已完成
    Completed,
    /// 已拒绝
    Rejected,
    /// 已取消
    Cancelled,
    /// 失败
    Failed,
}

impl TransferState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Rejected | Self::Cancelled | Self::Failed
        )
    }
}

/// 传输进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgress {
    /// 传输 ID
    pub transfer_id: String,
    /// 当前状态
    pub state: TransferState,
    /// 当前文件 ID
    pub current_file: Option<String>,
    /// 已传输字节数
    pub bytes_transferred: u64,
    /// 总字节数
    pub bytes_total: u64,
    /// 已完成文件数
    pub files_completed: usize,
    /// 总文件数
    pub files_total: usize,
    /// 传输速度（字节/秒）
    pub speed_bps: Option<u64>,
    /// 错误消息
    pub error: Option<String>,
}

impl TransferProgress {
    pub fn new(transfer_id: impl Into<String>, bytes_total: u64, files_total: usize) -> Self {
        Self {
            transfer_id: transfer_id.into(),
            state: TransferState::Pending,
            current_file: None,
            bytes_transferred: 0,
            bytes_total,
            files_completed: 0,
            files_total,
            speed_bps: None,
            error: None,
        }
    }

    pub fn progress_percent(&self) -> f64 {
        if self.bytes_total == 0 {
            0.0
        } else {
            (self.bytes_transferred as f64 / self.bytes_total as f64) * 100.0
        }
    }
}

/// 接收策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AcceptPolicy {
    /// 每次都询问
    #[default]
    AlwaysAsk,
    /// 可信设备自动接收
    AutoAcceptTrusted,
    /// 自动接收所有
    AutoAcceptAll,
}
