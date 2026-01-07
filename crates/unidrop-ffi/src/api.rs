//! FFI API 定义
//!
//! 这些函数和类型会被 flutter_rust_bridge 自动生成 Dart 绑定

use std::path::PathBuf;
use std::sync::Arc;

use flutter_rust_bridge::{frb, DartFnFuture};
use parking_lot::RwLock;

use unidrop_core::{Event, EventKind};
use unidrop_engine::{Engine, EngineConfig};
use unidrop_protocol_localsend::LocalSendFactory;
use unidrop_protocol_p2p::P2pFactory;

// ============================================================================
// 数据模型 - 暴露给 Flutter
// ============================================================================

/// 设备信息
#[frb(dart_metadata=("freezed"))]
pub struct FfiDevice {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub address: String,
    pub protocol: String,
}

/// 传输状态
#[frb(dart_metadata=("freezed"), unignore)]
pub struct FfiTransferProgress {
    pub transfer_id: String,
    pub file_name: String,
    pub bytes_sent: u64,
    pub total_bytes: u64,
    pub progress: f64,
}

/// 传输请求
#[frb(dart_metadata=("freezed"), unignore)]
pub struct FfiTransferRequest {
    pub request_id: String,
    pub from_device: FfiDevice,
    pub files: Vec<FfiFileInfo>,
    pub total_size: u64,
}

/// 文件信息
#[frb(dart_metadata=("freezed"), unignore)]
pub struct FfiFileInfo {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub mime_type: Option<String>,
}

/// 事件类型
#[frb(dart_metadata=("freezed"), unignore)]
pub enum FfiEvent {
    /// 发现设备
    DeviceDiscovered { device: FfiDevice },
    /// 设备离线
    DeviceLost { device_id: String },
    /// 收到传输请求
    TransferRequested { request: FfiTransferRequest },
    /// 传输进度
    TransferProgress { progress: FfiTransferProgress },
    /// 传输完成
    TransferCompleted { transfer_id: String },
    /// 传输失败
    TransferFailed { transfer_id: String, error: String },
    /// 错误
    Error { message: String },
}

/// 本机信息
#[frb(dart_metadata=("freezed"))]
pub struct FfiLocalInfo {
    pub device_name: String,
    pub save_dir: String,
    pub protocols: Vec<String>,
}

// ============================================================================
// 全局状态
// ============================================================================

lazy_static::lazy_static! {
    static ref ENGINE: RwLock<Option<Arc<Engine>>> = RwLock::new(None);
    static ref RUNTIME: RwLock<Option<tokio::runtime::Runtime>> = RwLock::new(None);
}

fn get_runtime() -> &'static tokio::runtime::Runtime {
    static RUNTIME_STATIC: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();
    RUNTIME_STATIC.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime")
    })
}

fn get_engine() -> Option<Arc<Engine>> {
    ENGINE.read().clone()
}

// ============================================================================
// API 函数
// ============================================================================

/// 初始化 UniDrop 引擎
#[frb(sync)]
pub fn init_engine(device_name: Option<String>, save_dir: Option<String>) -> Result<FfiLocalInfo, String> {
    // 初始化 tracing
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init();

    let name = device_name.unwrap_or_else(|| {
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "UniDrop".to_string())
    });

    let dir = save_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::download_dir()
                .or_else(dirs::home_dir)
                .unwrap_or_else(std::env::temp_dir)
                .join("UniDrop")
        });

    // 确保目录存在
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let config = EngineConfig {
        device_name: name.clone(),
        port: 0,
        save_dir: dir.clone(),
        encryption: true,
        pin: None,
    };

    let engine = Engine::builder()
        .config(config)
        .with_protocol(LocalSendFactory::new())
        .with_protocol(P2pFactory::new())
        .build();

    let protocols: Vec<String> = engine.protocols().iter().map(|p| p.name.clone()).collect();

    *ENGINE.write() = Some(Arc::new(engine));

    Ok(FfiLocalInfo {
        device_name: name,
        save_dir: dir.to_string_lossy().to_string(),
        protocols,
    })
}

/// 启动引擎
pub async fn start_engine() -> Result<(), String> {
    let engine = get_engine().ok_or("Engine not initialized")?;
    engine.start().await.map_err(|e| e.to_string())
}

/// 停止引擎
pub async fn stop_engine() -> Result<(), String> {
    let engine = get_engine().ok_or("Engine not initialized")?;
    engine.stop().await.map_err(|e| e.to_string())
}

/// 获取在线设备列表
pub async fn get_devices() -> Result<Vec<FfiDevice>, String> {
    let engine = get_engine().ok_or("Engine not initialized")?;
    let devices = engine.devices().await;

    Ok(devices
        .into_iter()
        .map(|d| FfiDevice {
            id: d.id().to_string(),
            name: d.name().to_string(),
            device_type: format!("{:?}", d.peer.device_type),
            address: d.address(),
            protocol: d.protocol().to_string(),
        })
        .collect())
}

/// 扫描设备
pub async fn scan_devices() -> Result<(), String> {
    let engine = get_engine().ok_or("Engine not initialized")?;
    engine.scan().await.map_err(|e| e.to_string())
}

/// 发送文件
pub async fn send_files(device_id: String, file_paths: Vec<String>) -> Result<String, String> {
    let engine = get_engine().ok_or("Engine not initialized")?;

    // 查找设备
    let devices = engine.devices().await;
    let device = devices
        .iter()
        .find(|d| d.id().to_string() == device_id)
        .ok_or("Device not found")?;

    let files: Vec<PathBuf> = file_paths.into_iter().map(PathBuf::from).collect();
    let intent = unidrop_core::TransferIntent::new(device.id().clone(), files);

    engine.send(intent).await.map_err(|e| e.to_string())
}

/// 接受传输请求
pub async fn accept_transfer(request_id: String, protocol: String) -> Result<(), String> {
    let engine = get_engine().ok_or("Engine not initialized")?;

    // 这里需要找到对应的 TransferRequest
    // 简化处理，实际需要维护 pending requests
    Err("Not implemented yet".to_string())
}

/// 拒绝传输请求
pub async fn reject_transfer(request_id: String, protocol: String) -> Result<(), String> {
    let engine = get_engine().ok_or("Engine not initialized")?;
    Err("Not implemented yet".to_string())
}

/// 订阅事件流 - 通过回调函数接收事件
pub async fn subscribe_events(callback: impl Fn(FfiEvent) -> DartFnFuture<()> + Send + Sync + 'static) -> Result<(), String> {
    let engine = get_engine().ok_or("Engine not initialized")?;
    let mut rx = engine.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            if let Some(ffi_event) = convert_event(event) {
                let _ = callback(ffi_event).await;
            }
        }
    });

    Ok(())
}

fn convert_event(event: Event) -> Option<FfiEvent> {
    match event.kind {
        EventKind::DeviceDiscovered(device) => Some(FfiEvent::DeviceDiscovered {
            device: FfiDevice {
                id: device.id().to_string(),
                name: device.name().to_string(),
                device_type: format!("{:?}", device.peer.device_type),
                address: device.address(),
                protocol: device.protocol().to_string(),
            },
        }),
        EventKind::DeviceLost(id) => Some(FfiEvent::DeviceLost {
            device_id: id.to_string(),
        }),
        EventKind::TransferRequested(request) => Some(FfiEvent::TransferRequested {
            request: FfiTransferRequest {
                request_id: request.id.clone(),
                from_device: FfiDevice {
                    id: request.from.id().to_string(),
                    name: request.from.name().to_string(),
                    device_type: format!("{:?}", request.from.peer.device_type),
                    address: request.from.address(),
                    protocol: request.from.protocol().to_string(),
                },
                files: request
                    .files
                    .iter()
                    .map(|f| FfiFileInfo {
                        id: f.id.clone(),
                        name: f.name.clone(),
                        size: f.size,
                        mime_type: Some(f.mime_type.clone()),
                    })
                    .collect(),
                total_size: request.total_size,
            },
        }),
        EventKind::TransferProgress(progress) => Some(FfiEvent::TransferProgress {
            progress: FfiTransferProgress {
                transfer_id: progress.transfer_id.clone(),
                file_name: progress.current_file.clone().unwrap_or_default(),
                bytes_sent: progress.bytes_transferred,
                total_bytes: progress.bytes_total,
                progress: progress.progress_percent() / 100.0,
            },
        }),
        EventKind::TransferCompleted { transfer_id } => {
            Some(FfiEvent::TransferCompleted { transfer_id })
        }
        EventKind::TransferFailed { transfer_id, error } => {
            Some(FfiEvent::TransferFailed { transfer_id, error })
        }
        EventKind::Error { message, .. } => Some(FfiEvent::Error { message }),
        _ => None,
    }
}

/// 获取协议列表
#[frb(sync)]
pub fn get_protocols() -> Result<Vec<String>, String> {
    let engine = get_engine().ok_or("Engine not initialized")?;
    Ok(engine.protocols().iter().map(|p| p.name.clone()).collect())
}

/// 检查引擎是否正在运行
#[frb(sync)]
pub fn is_engine_running() -> bool {
    get_engine().map(|e| e.is_running()).unwrap_or(false)
}
