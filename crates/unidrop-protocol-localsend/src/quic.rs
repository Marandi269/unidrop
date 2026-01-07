//! QUIC 传输层 - 高性能文件传输
//!
//! 使用 QUIC 协议替代 HTTPS，提供：
//! - 0-RTT 快速连接
//! - 多路复用（同时传多个文件）
//! - 内置 TLS 1.3 加密
//! - 更好的拥塞控制

use bytes::Bytes;
use quinn::{ClientConfig, Endpoint, RecvStream, SendStream, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, error, info, warn};

use crate::cert::CertInfo;

/// QUIC 传输端口（与 HTTP 端口区分）
pub const QUIC_PORT_OFFSET: u16 = 1; // 53318

/// 传输消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// 传输请求
    TransferRequest {
        session_id: String,
        files: Vec<FileMetadata>,
    },
    /// 传输响应
    TransferResponse {
        session_id: String,
        accepted: bool,
        tokens: Vec<String>, // 每个文件一个 token
    },
    /// 文件头
    FileHeader {
        file_id: String,
        token: String,
        file_name: String,
        size: u64,
    },
    /// 传输完成
    TransferComplete { session_id: String },
    /// 错误
    Error { message: String },
}

/// 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub mime_type: Option<String>,
}

/// QUIC 服务器
pub struct QuicServer {
    endpoint: Endpoint,
    save_dir: PathBuf,
}

impl QuicServer {
    /// 创建 QUIC 服务器
    pub fn new(port: u16, cert_info: &CertInfo, save_dir: PathBuf) -> unidrop_core::Result<Self> {
        let server_config = create_server_config(cert_info)?;
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

        let endpoint = Endpoint::server(server_config, addr)
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        info!("QUIC server listening on {}", addr);

        Ok(Self { endpoint, save_dir })
    }

    /// 启动服务器
    pub async fn run(&self) -> unidrop_core::Result<()> {
        info!("QUIC server started, waiting for connections...");

        while let Some(conn) = self.endpoint.accept().await {
            let save_dir = self.save_dir.clone();

            tokio::spawn(async move {
                match conn.await {
                    Ok(connection) => {
                        let remote = connection.remote_address();
                        info!("QUIC connection from {}", remote);

                        if let Err(e) = handle_connection(connection, save_dir).await {
                            error!("Connection error from {}: {}", remote, e);
                        }
                    }
                    Err(e) => {
                        error!("Connection failed: {}", e);
                    }
                }
            });
        }

        Ok(())
    }

    /// 获取本地地址
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.endpoint.local_addr().ok()
    }
}

/// QUIC 客户端
#[derive(Clone)]
pub struct QuicClient {
    endpoint: Endpoint,
}

impl QuicClient {
    /// 创建 QUIC 客户端
    pub fn new() -> unidrop_core::Result<Self> {
        let client_config = create_client_config()?;

        let mut endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap())
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        endpoint.set_default_client_config(client_config);

        Ok(Self { endpoint })
    }

    /// 发送文件到目标
    pub async fn send_files(
        &self,
        target: SocketAddr,
        files: Vec<PathBuf>,
    ) -> unidrop_core::Result<String> {
        let server_name = "unidrop"; // 自签名证书的名称

        info!("Connecting to {} via QUIC...", target);

        let connection = self
            .endpoint
            .connect(target, server_name)
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        info!("QUIC connection established");

        // 生成 session ID
        let session_id = uuid::Uuid::new_v4().to_string();

        // 收集文件元数据
        let mut file_metas = Vec::new();
        for (i, path) in files.iter().enumerate() {
            let metadata = tokio::fs::metadata(path).await?;
            file_metas.push(FileMetadata {
                id: format!("file_{}", i),
                name: path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                size: metadata.len(),
                mime_type: None,
            });
        }

        // 打开控制流
        let (mut send, mut recv) = connection
            .open_bi()
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        // 发送传输请求
        let request = Message::TransferRequest {
            session_id: session_id.clone(),
            files: file_metas.clone(),
        };
        send_message(&mut send, &request).await?;

        // 等待响应
        let response: Message = recv_message(&mut recv).await?;
        let tokens = match response {
            Message::TransferResponse {
                accepted: true,
                tokens,
                ..
            } => tokens,
            Message::TransferResponse {
                accepted: false, ..
            } => {
                return Err(unidrop_core::Error::Protocol("Transfer rejected".to_string()));
            }
            Message::Error { message } => {
                return Err(unidrop_core::Error::Protocol(message));
            }
            _ => {
                return Err(unidrop_core::Error::Protocol(
                    "Unexpected response".to_string(),
                ));
            }
        };

        // 发送每个文件（使用独立的流）
        for (i, path) in files.iter().enumerate() {
            let meta = &file_metas[i];
            let token = &tokens[i];

            info!("Sending file: {} ({} bytes)", meta.name, meta.size);

            // 为每个文件打开一个新的流
            let (mut file_send, _) = connection
                .open_bi()
                .await
                .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

            // 发送文件头
            let header = Message::FileHeader {
                file_id: meta.id.clone(),
                token: token.clone(),
                file_name: meta.name.clone(),
                size: meta.size,
            };
            send_message(&mut file_send, &header).await?;

            // 发送文件内容
            let mut file = File::open(path).await?;
            let mut buffer = vec![0u8; 64 * 1024]; // 64KB buffer
            let mut sent = 0u64;

            loop {
                let n = file.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }
                file_send
                    .write_all(&buffer[..n])
                    .await
                    .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;
                sent += n as u64;

                // 进度
                let progress = (sent as f64 / meta.size as f64 * 100.0) as u32;
                debug!("Progress: {}% ({}/{})", progress, sent, meta.size);
            }

            file_send
                .finish()
                .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

            info!("File sent: {}", meta.name);
        }

        // 发送完成消息
        let complete = Message::TransferComplete {
            session_id: session_id.clone(),
        };
        send_message(&mut send, &complete).await?;

        info!("Transfer complete: {}", session_id);
        Ok(session_id)
    }
}

impl Default for QuicClient {
    fn default() -> Self {
        Self::new().expect("Failed to create QUIC client")
    }
}

/// 处理连接
async fn handle_connection(
    connection: quinn::Connection,
    save_dir: PathBuf,
) -> unidrop_core::Result<()> {
    let remote = connection.remote_address();

    // 接受控制流
    let (mut send, mut recv) = connection
        .accept_bi()
        .await
        .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

    // 读取传输请求
    let request: Message = recv_message(&mut recv).await?;

    let (session_id, files) = match request {
        Message::TransferRequest { session_id, files } => (session_id, files),
        _ => {
            let error = Message::Error {
                message: "Expected TransferRequest".to_string(),
            };
            send_message(&mut send, &error).await?;
            return Err(unidrop_core::Error::Protocol(
                "Invalid request".to_string(),
            ));
        }
    };

    info!(
        "Transfer request from {}: {} files",
        remote,
        files.len()
    );

    // 自动接受，生成 tokens
    let tokens: Vec<String> = files
        .iter()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect();

    let response = Message::TransferResponse {
        session_id: session_id.clone(),
        accepted: true,
        tokens: tokens.clone(),
    };
    send_message(&mut send, &response).await?;

    // 创建保存目录
    tokio::fs::create_dir_all(&save_dir).await?;

    // 接收文件
    let mut received = 0;
    while received < files.len() {
        // 接受文件流
        let (_, mut file_recv) = connection
            .accept_bi()
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        // 读取文件头
        let header: Message = recv_message(&mut file_recv).await?;

        let (file_id, token, file_name, size) = match header {
            Message::FileHeader {
                file_id,
                token,
                file_name,
                size,
            } => (file_id, token, file_name, size),
            _ => {
                warn!("Expected FileHeader, got {:?}", header);
                continue;
            }
        };

        // 验证 token
        let expected_idx = files.iter().position(|f| f.id == file_id);
        if let Some(idx) = expected_idx {
            if tokens[idx] != token {
                warn!("Invalid token for file {}", file_id);
                continue;
            }
        } else {
            warn!("Unknown file_id: {}", file_id);
            continue;
        }

        // 接收文件内容
        let save_path = save_dir.join(&file_name);
        let mut file = File::create(&save_path).await?;
        let mut total = 0u64;

        while let Some(chunk) = file_recv
            .read_chunk(64 * 1024, true)
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?
        {
            file.write_all(&chunk.bytes).await?;
            total += chunk.bytes.len() as u64;
        }

        info!("Received file: {} ({} bytes) -> {:?}", file_name, total, save_path);
        received += 1;
    }

    // 等待完成消息
    let complete: Message = recv_message(&mut recv).await?;
    match complete {
        Message::TransferComplete { session_id: sid } if sid == session_id => {
            info!("Transfer session {} completed", session_id);
        }
        _ => {
            warn!("Unexpected message at end of transfer");
        }
    }

    Ok(())
}

/// 创建服务器 TLS 配置
fn create_server_config(cert_info: &CertInfo) -> unidrop_core::Result<ServerConfig> {
    let cert = CertificateDer::from(cert_info.cert_der.clone());
    let key = PrivateKeyDer::try_from(cert_info.key_der.clone())
        .map_err(|e| unidrop_core::Error::Protocol(format!("Invalid key: {}", e)))?;

    let _ = rustls::crypto::ring::default_provider().install_default();

    let server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)
        .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?;

    let server_config = ServerConfig::with_crypto(Arc::new(
        quinn::crypto::rustls::QuicServerConfig::try_from(server_crypto)
            .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?,
    ));

    Ok(server_config)
}

/// 创建客户端 TLS 配置（跳过证书验证，用于自签名证书）
fn create_client_config() -> unidrop_core::Result<ClientConfig> {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let client_crypto = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();

    let client_config = ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto)
            .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?,
    ));

    Ok(client_config)
}

/// 跳过服务器证书验证（用于自签名证书）
#[derive(Debug)]
struct SkipServerVerification;

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

/// 发送消息
async fn send_message(send: &mut SendStream, msg: &Message) -> unidrop_core::Result<()> {
    let data = serde_json::to_vec(msg)
        .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?;
    let len = (data.len() as u32).to_be_bytes();

    send.write_all(&len)
        .await
        .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;
    send.write_all(&data)
        .await
        .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

    Ok(())
}

/// 接收消息
async fn recv_message<T: for<'de> Deserialize<'de>>(
    recv: &mut RecvStream,
) -> unidrop_core::Result<T> {
    let mut len_buf = [0u8; 4];
    recv.read_exact(&mut len_buf)
        .await
        .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

    let len = u32::from_be_bytes(len_buf) as usize;
    let mut data = vec![0u8; len];

    recv.read_exact(&mut data)
        .await
        .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

    let msg = serde_json::from_slice(&data)
        .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?;
    Ok(msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quic_client_creation() {
        let client = QuicClient::new();
        assert!(client.is_ok());
    }
}
