//! LocalSend API 数据模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 设备信息消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub alias: String,
    pub version: String,
    pub fingerprint: String,
    pub port: u16,
    pub protocol: String, // "http" | "https"
    #[serde(rename = "deviceModel", skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
}

impl DeviceInfo {
    pub fn new(alias: String, fingerprint: String, port: u16) -> Self {
        Self {
            alias,
            version: crate::PROTOCOL_VERSION.to_string(),
            fingerprint,
            port,
            protocol: "https".to_string(),
            device_model: None,
            device_type: Some("desktop".to_string()),
            download: Some(false),
        }
    }
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub size: u64,
    #[serde(rename = "fileType")]
    pub file_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

/// 准备上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareUploadRequest {
    pub info: DeviceInfo,
    pub files: HashMap<String, FileInfo>,
}

/// 准备上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareUploadResponse {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub files: HashMap<String, String>, // file_id -> token
}

/// 取消请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRequest {
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

/// 组播发现消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MulticastAnnounce {
    pub alias: String,
    pub version: String,
    pub fingerprint: String,
    pub port: u16,
    pub protocol: String,
    #[serde(rename = "deviceModel", skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    pub announce: bool,
}

impl From<DeviceInfo> for MulticastAnnounce {
    fn from(info: DeviceInfo) -> Self {
        Self {
            alias: info.alias,
            version: info.version,
            fingerprint: info.fingerprint,
            port: info.port,
            protocol: info.protocol,
            device_model: info.device_model,
            device_type: info.device_type,
            announce: true,
        }
    }
}
