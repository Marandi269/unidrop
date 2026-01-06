//! LocalSend HTTP 服务器 - 接收文件

use axum::{
    body::Bytes,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

use unidrop_core::{Device, Event};

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

/// HTTP 服务器
pub struct HttpServer {
    state: Arc<ServerState>,
    port: u16,
}

impl HttpServer {
    pub fn new(
        local_info: DeviceInfo,
        save_dir: PathBuf,
        pin: Option<String>,
        event_tx: mpsc::Sender<Event>,
    ) -> Self {
        let port = local_info.port;
        let state = Arc::new(ServerState {
            local_info,
            sessions: RwLock::new(HashMap::new()),
            save_dir,
            pin,
            event_tx,
        });

        Self { state, port }
    }

    /// 获取状态引用
    pub fn state(&self) -> Arc<ServerState> {
        self.state.clone()
    }

    /// 启动服务器
    pub async fn start(&self) -> unidrop_core::Result<()> {
        let app = Router::new()
            .route("/api/localsend/v2/register", post(register))
            .route("/api/localsend/v2/prepare-upload", post(prepare_upload))
            .route("/api/localsend/v2/upload", post(upload_simple))
            .route("/api/localsend/v2/cancel", post(cancel))
            .route("/api/localsend/v2/info", get(info))
            .with_state(self.state.clone());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("LocalSend server listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app)
            .await
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;

        Ok(())
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
    body: axum::body::Body,
) -> StatusCode {
    use http_body_util::BodyExt;

    // 读取 body
    let body_bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let sessions = state.sessions.read();
    let session = match sessions.get(&query.session_id) {
        Some(s) => s,
        None => return StatusCode::NOT_FOUND,
    };

    // 验证 token
    let expected_token = match session.tokens.get(&query.file_id) {
        Some(t) => t,
        None => return StatusCode::NOT_FOUND,
    };

    if &query.token != expected_token {
        return StatusCode::UNAUTHORIZED;
    }

    // 获取文件信息
    let file_info = match session.files.get(&query.file_id) {
        Some(f) => f,
        None => return StatusCode::NOT_FOUND,
    };

    // 确保保存目录存在
    let _ = std::fs::create_dir_all(&state.save_dir);
    let save_path = state.save_dir.join(&file_info.file_name);

    drop(sessions); // 释放锁

    // 保存文件
    if let Err(e) = tokio::fs::write(&save_path, &body_bytes).await {
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
