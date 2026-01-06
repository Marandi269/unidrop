//! UniDrop Daemon - 后台服务

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use unidrop_engine::{Engine, EngineConfig};
use unidrop_protocol_localsend::LocalSendFactory;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("UniDrop Daemon starting...");

    // 配置
    let config = EngineConfig {
        device_name: hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "UniDrop".to_string()),
        save_dir: dirs::download_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(std::env::temp_dir)
            .join("UniDrop"),
        encryption: true,
        pin: None,
    };

    info!("Device name: {}", config.device_name);
    info!("Save directory: {:?}", config.save_dir);

    // 确保保存目录存在
    std::fs::create_dir_all(&config.save_dir)?;

    // 创建 Engine
    let engine = Engine::builder()
        .config(config)
        .with_protocol(LocalSendFactory::new())
        .build();

    // 订阅事件
    let mut events = engine.subscribe();

    // 启动 Engine
    engine.start().await?;

    info!("UniDrop Daemon started. Press Ctrl+C to stop.");

    // 事件处理循环
    tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            match &event.kind {
                unidrop_core::EventKind::DeviceDiscovered(device) => {
                    info!("Device discovered: {} ({})", device.name(), device.address());
                }
                unidrop_core::EventKind::DeviceLost(id) => {
                    info!("Device lost: {}", id);
                }
                unidrop_core::EventKind::TransferRequested(request) => {
                    info!(
                        "Transfer request from {}: {} files",
                        request.from.name(),
                        request.file_count()
                    );
                }
                unidrop_core::EventKind::TransferCompleted { transfer_id } => {
                    info!("Transfer completed: {}", transfer_id);
                }
                _ => {}
            }
        }
    });

    // 等待 Ctrl+C
    tokio::signal::ctrl_c().await?;

    info!("Shutting down...");
    engine.stop().await?;

    Ok(())
}
