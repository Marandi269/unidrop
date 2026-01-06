//! LocalSend HTTP 客户端 - 发送文件

use reqwest::Client;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::{debug, info};

use unidrop_core::{Device, Result};

use crate::models::*;

/// HTTP 客户端
#[derive(Clone)]
pub struct HttpClient {
    http: Client,
    local_info: DeviceInfo,
}

impl HttpClient {
    pub fn new(local_info: DeviceInfo) -> Self {
        let http = Client::builder()
            .danger_accept_invalid_certs(true) // LocalSend 使用自签名证书
            .build()
            .expect("Failed to create HTTP client");

        Self { http, local_info }
    }

    /// 发送文件到设备
    pub async fn send_files(&self, target: &Device, files: Vec<PathBuf>) -> Result<String> {
        let base_url = format!("https://{}:{}/api/localsend/v2", target.ip, target.port);

        // 1. 构建文件信息
        let mut file_infos = HashMap::new();
        for (idx, path) in files.iter().enumerate() {
            let metadata = tokio::fs::metadata(path).await?;
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let file_id = format!("file_{}", idx);
            file_infos.insert(
                file_id.clone(),
                FileInfo {
                    id: file_id,
                    file_name,
                    size: metadata.len(),
                    file_type: guess_mime_type(path),
                    sha256: None,
                    preview: None,
                },
            );
        }

        // 2. 准备上传
        let prepare_request = PrepareUploadRequest {
            info: self.local_info.clone(),
            files: file_infos.clone(),
        };

        info!("Preparing upload to {}", target.name());

        let response = self
            .http
            .post(format!("{}/prepare-upload", base_url))
            .json(&prepare_request)
            .send()
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::FORBIDDEN {
            return Err(unidrop_core::Error::Rejected);
        }

        if !response.status().is_success() {
            return Err(unidrop_core::Error::Network(format!(
                "Prepare failed: {}",
                response.status()
            )));
        }

        let prepare_response: PrepareUploadResponse = response
            .json()
            .await
            .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?;

        let session_id = prepare_response.session_id.clone();
        info!("Upload session created: {}", session_id);

        // 3. 上传每个文件
        for (idx, path) in files.iter().enumerate() {
            let file_id = format!("file_{}", idx);
            let token = prepare_response
                .files
                .get(&file_id)
                .ok_or_else(|| unidrop_core::Error::Protocol("Missing token".into()))?;

            self.upload_file(&base_url, &session_id, &file_id, token, path)
                .await?;
        }

        info!("All files sent successfully");
        Ok(session_id)
    }

    /// 上传单个文件
    async fn upload_file(
        &self,
        base_url: &str,
        session_id: &str,
        file_id: &str,
        token: &str,
        path: &PathBuf,
    ) -> Result<()> {
        let url = format!(
            "{}/upload?sessionId={}&fileId={}&token={}",
            base_url, session_id, file_id, token
        );

        debug!("Uploading file: {:?}", path);

        // 读取文件
        let mut file = File::open(path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        // 构建 multipart
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();

        let part = reqwest::multipart::Part::bytes(contents).file_name(file_name);
        let form = reqwest::multipart::Form::new().part("file", part);

        let response = self
            .http
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(unidrop_core::Error::TransferFailed(format!(
                "Upload failed: {}",
                response.status()
            )));
        }

        debug!("File uploaded successfully");
        Ok(())
    }

    /// 取消传输
    pub async fn cancel(&self, target: &Device, session_id: &str) -> Result<()> {
        let url = format!(
            "https://{}:{}/api/localsend/v2/cancel",
            target.ip, target.port
        );

        self.http
            .post(&url)
            .json(&CancelRequest {
                session_id: session_id.to_string(),
            })
            .send()
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        Ok(())
    }
}

/// 猜测 MIME 类型
fn guess_mime_type(path: &PathBuf) -> String {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "txt" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        _ => "application/octet-stream",
    }
    .to_string()
}
