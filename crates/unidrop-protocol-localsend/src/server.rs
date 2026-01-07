//! LocalSend HTTP 服务器 - 接收文件

use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_rustls::TlsAcceptor;
use tower::ServiceExt;
use tracing::{error, info};

use unidrop_core::{Device, Event};

use crate::cert::CertInfo;
use crate::models::*;

/// 传输会话
pub struct TransferSession {
    pub id: String,
    pub from: Device,
    pub files: HashMap<String, FileInfo>,
    pub tokens: HashMap<String, String>,
}

/// 服务器状态
pub struct ServerState {
    pub local_info: DeviceInfo,
    pub sessions: RwLock<HashMap<String, TransferSession>>,
    pub save_dir: PathBuf,
    pub pin: Option<String>,
    pub event_tx: mpsc::Sender<Event>,
}

/// HTTPS 服务器
pub struct HttpServer {
    state: Arc<ServerState>,
    port: u16,
    tls_acceptor: TlsAcceptor,
}

impl HttpServer {
    pub fn new(
        local_info: DeviceInfo,
        save_dir: PathBuf,
        pin: Option<String>,
        event_tx: mpsc::Sender<Event>,
        cert_info: &CertInfo,
    ) -> unidrop_core::Result<Self> {
        let port = local_info.port;
        let state = Arc::new(ServerState {
            local_info,
            sessions: RwLock::new(HashMap::new()),
            save_dir,
            pin,
            event_tx,
        });

        // 创建 TLS 配置
        let cert = CertificateDer::from(cert_info.cert_der.clone());
        let key = PrivateKeyDer::try_from(cert_info.key_der.clone())
            .map_err(|e| unidrop_core::Error::Protocol(format!("Invalid key: {}", e)))?;

        // 确保 crypto provider 已安装
        let _ = rustls::crypto::ring::default_provider().install_default();

        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .map_err(|e| unidrop_core::Error::Protocol(format!("TLS config error: {}", e)))?;

        let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));

        Ok(Self {
            state,
            port,
            tls_acceptor,
        })
    }

    /// 启动 HTTPS 服务器
    pub async fn start(&self) -> unidrop_core::Result<()> {
        let app = Router::new()
            .route("/api/localsend/v2/register", post(register))
            .route("/api/localsend/v2/prepare-upload", post(prepare_upload))
            .route("/api/localsend/v2/upload", post(upload_simple))
            .route("/api/localsend/v2/cancel", post(cancel))
            .route("/api/localsend/v2/info", get(info))
            .with_state(self.state.clone());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("LocalSend HTTPS server listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        let tls_acceptor = self.tls_acceptor.clone();

        loop {
            let (stream, peer_addr) = match listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Accept error: {}", e);
                    continue;
                }
            };

            let tls_acceptor = tls_acceptor.clone();
            let app = app.clone();

            tokio::spawn(async move {
                match tls_acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        let io = hyper_util::rt::TokioIo::new(tls_stream);
                        let service = hyper::service::service_fn(move |req| {
                            let app = app.clone();
                            async move { app.oneshot(req).await }
                        });

                        if let Err(e) = hyper_util::server::conn::auto::Builder::new(
                            hyper_util::rt::TokioExecutor::new(),
                        )
                        .serve_connection(io, service)
                        .await
                        {
                            // 忽略连接关闭错误
                            if !e.to_string().contains("connection closed") {
                                error!("Connection error from {}: {}", peer_addr, e);
                            }
                        }
                    }
                    Err(e) => {
                        // TLS 握手失败通常是正常的（比如客户端探测）
                        tracing::debug!("TLS handshake failed from {}: {}", peer_addr, e);
                    }
                }
            });
        }
    }
}

// === HTTP Handlers ===

/// POST /register - 设备注册
async fn register(
    State(state): State<Arc<ServerState>>,
    Json(_device): Json<DeviceInfo>,
) -> impl IntoResponse {
    Json(state.local_info.clone())
}

#[derive(Debug, Deserialize)]
pub struct PrepareUploadQuery {
    pub pin: Option<String>,
}

/// POST /prepare-upload - 准备上传
async fn prepare_upload(
    State(state): State<Arc<ServerState>>,
    Query(query): Query<PrepareUploadQuery>,
    Json(request): Json<PrepareUploadRequest>,
) -> Result<Json<PrepareUploadResponse>, StatusCode> {
    // 验证 PIN
    if let Some(required_pin) = &state.pin {
        match &query.pin {
            Some(pin) if pin == required_pin => {}
            _ => return Err(StatusCode::UNAUTHORIZED),
        }
    }

    // 创建会话
    let session_id = uuid::Uuid::new_v4().to_string();
    let mut file_tokens = HashMap::new();

    for (file_id, _) in &request.files {
        file_tokens.insert(file_id.clone(), uuid::Uuid::new_v4().to_string());
    }

    // TODO: 构造 Device 从 request.info
    // 暂时跳过事件发送，后续完善

    let session = TransferSession {
        id: session_id.clone(),
        from: create_temp_device(&request.info),
        files: request.files.clone(),
        tokens: file_tokens.clone(),
    };

    state.sessions.write().insert(session_id.clone(), session);

    info!("Created upload session: {}", session_id);

    Ok(Json(PrepareUploadResponse {
        session_id,
        files: file_tokens,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
    pub token: String,
}

/// POST /upload - 简化版上传处理
async fn upload_simple(
    State(state): State<Arc<ServerState>>,
    Query(query): Query<UploadQuery>,
    mut multipart: Multipart,
) -> StatusCode {
    // 提取所需数据，尽快释放锁
    let (file_name, save_dir, valid) = {
        let sessions = state.sessions.read();
        let Some(session) = sessions.get(&query.session_id) else {
            return StatusCode::NOT_FOUND;
        };

        let Some(expected_token) = session.tokens.get(&query.file_id) else {
            return StatusCode::NOT_FOUND;
        };

        if &query.token != expected_token {
            return StatusCode::UNAUTHORIZED;
        }

        let Some(file_info) = session.files.get(&query.file_id) else {
            return StatusCode::NOT_FOUND;
        };

        (file_info.file_name.clone(), state.save_dir.clone(), true)
    };

    if !valid {
        return StatusCode::BAD_REQUEST;
    }

    // 解析 multipart 表单，提取文件内容
    let file_data = match multipart.next_field().await {
        Ok(Some(field)) => {
            match field.bytes().await {
                Ok(data) => data,
                Err(e) => {
                    error!("Failed to read field bytes: {}", e);
                    return StatusCode::BAD_REQUEST;
                }
            }
        }
        Ok(None) => {
            error!("No file field in multipart form");
            return StatusCode::BAD_REQUEST;
        }
        Err(e) => {
            error!("Failed to parse multipart form: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    // 确保保存目录存在
    let _ = std::fs::create_dir_all(&save_dir);
    let save_path = save_dir.join(&file_name);

    // 保存文件
    if let Err(e) = tokio::fs::write(&save_path, &file_data).await {
        error!("Write error: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    info!("Saved file: {:?}", save_path);
    StatusCode::OK
}

/// POST /cancel - 取消传输
async fn cancel(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<CancelRequest>,
) -> StatusCode {
    state.sessions.write().remove(&request.session_id);
    info!("Cancelled session: {}", request.session_id);
    StatusCode::OK
}

/// GET /info - 设备信息
async fn info(State(state): State<Arc<ServerState>>) -> Json<DeviceInfo> {
    Json(state.local_info.clone())
}

/// 解析上传查询参数
fn parse_upload_query(query: &str) -> Option<UploadQuery> {
    let mut session_id = None;
    let mut file_id = None;
    let mut token = None;

    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next()?;
        let value = parts.next().unwrap_or("");

        match key {
            "sessionId" => session_id = Some(value.to_string()),
            "fileId" => file_id = Some(value.to_string()),
            "token" => token = Some(value.to_string()),
            _ => {}
        }
    }

    Some(UploadQuery {
        session_id: session_id?,
        file_id: file_id?,
        token: token?,
    })
}

/// 临时创建 Device（后续需要完善）
fn create_temp_device(info: &DeviceInfo) -> Device {
    use std::net::Ipv4Addr;
    use unidrop_core::{DeviceType, Peer, ProtocolId};

    let peer = Peer::new(
        ProtocolId::new(crate::PROTOCOL_ID),
        info.fingerprint.clone(),
        info.alias.clone(),
    )
    .with_device_type(
        info.device_type
            .as_ref()
            .map(|t| DeviceType::from_str(t))
            .unwrap_or(DeviceType::Desktop),
    )
    .with_version(&info.version);

    Device::new(
        peer,
        std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        info.port,
    )
}
