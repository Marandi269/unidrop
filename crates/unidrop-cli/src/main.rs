//! UniDrop CLI - 命令行工具

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use unidrop_core::{DeviceId, ProtocolId, TransferIntent};
use unidrop_engine::{Engine, EngineConfig};
use unidrop_protocol_localsend::LocalSendFactory;

#[derive(Parser)]
#[command(name = "drop")]
#[command(author, version, about = "UniDrop - Cross-platform file sharing")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List online devices
    Devices,

    /// Send files to a device
    Send {
        /// Files to send
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Target device (fingerprint or name)
        #[arg(short, long)]
        to: Option<String>,
    },

    /// Show registered protocols
    Protocols,

    /// Receive mode (wait for incoming transfers)
    Receive,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 初始化日志
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // 创建 Engine
    let engine = create_engine();

    match cli.command {
        Commands::Devices => list_devices(&engine).await?,
        Commands::Send { files, to } => send_files(&engine, files, to).await?,
        Commands::Protocols => list_protocols(&engine),
        Commands::Receive => receive_mode(&engine).await?,
    }

    Ok(())
}

fn create_engine() -> Engine {
    let config = EngineConfig {
        device_name: hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "UniDrop-CLI".to_string()),
        save_dir: dirs::download_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(std::env::temp_dir)
            .join("UniDrop"),
        encryption: true,
        pin: None,
    };

    Engine::builder()
        .config(config)
        .with_protocol(LocalSendFactory::new())
        .build()
}

async fn list_devices(engine: &Engine) -> Result<()> {
    engine.start().await?;

    println!("Scanning for devices...\n");

    // 等待一段时间让 mDNS 发现设备
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let devices = engine.devices().await;

    if devices.is_empty() {
        println!("No devices found.");
        println!("\nMake sure other devices are running LocalSend or UniDrop.");
    } else {
        println!("Found {} device(s):\n", devices.len());
        for device in devices {
            println!(
                "  {} ({})",
                device.name(),
                device.address()
            );
            println!("    Protocol: {}", device.protocol());
            println!("    ID: {}", device.id());
            println!();
        }
    }

    engine.stop().await?;
    Ok(())
}

async fn send_files(engine: &Engine, files: Vec<PathBuf>, to: Option<String>) -> Result<()> {
    // 检查文件是否存在
    for file in &files {
        if !file.exists() {
            anyhow::bail!("File not found: {:?}", file);
        }
    }

    engine.start().await?;

    println!("Scanning for devices...\n");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let devices = engine.devices().await;

    if devices.is_empty() {
        println!("No devices found.");
        engine.stop().await?;
        return Ok(());
    }

    // 选择目标设备
    let target = match to {
        Some(ref name) => {
            devices
                .iter()
                .find(|d| {
                    d.name().to_lowercase().contains(&name.to_lowercase())
                        || d.id().fingerprint.starts_with(name)
                })
                .ok_or_else(|| anyhow::anyhow!("Device not found: {}", name))?
        }
        None => {
            if devices.len() == 1 {
                &devices[0]
            } else {
                println!("Multiple devices found. Please specify target with --to:");
                for device in &devices {
                    println!("  {} ({})", device.name(), device.id().fingerprint);
                }
                engine.stop().await?;
                return Ok(());
            }
        }
    };

    println!("Sending {} file(s) to {}...\n", files.len(), target.name());

    let intent = TransferIntent::new(target.id().clone(), files);

    match engine.send(intent).await {
        Ok(session_id) => {
            println!("Transfer completed successfully!");
            println!("Session ID: {}", session_id);
        }
        Err(e) => {
            println!("Transfer failed: {}", e);
        }
    }

    engine.stop().await?;
    Ok(())
}

fn list_protocols(engine: &Engine) {
    let protocols = engine.protocols();

    println!("Registered protocols:\n");
    for proto in protocols {
        let status = if proto.supported {
            "supported"
        } else {
            "not supported"
        };
        println!("  {} v{} ({})", proto.name, proto.version, status);
        println!("    ID: {}", proto.id);
        println!("    {}", proto.description);
        println!();
    }
}

async fn receive_mode(engine: &Engine) -> Result<()> {
    engine.start().await?;

    let mut events = engine.subscribe();

    println!("UniDrop is now receiving...");
    println!("Press Ctrl+C to stop.\n");

    tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            match &event.kind {
                unidrop_core::EventKind::DeviceDiscovered(device) => {
                    println!("Device online: {} ({})", device.name(), device.address());
                }
                unidrop_core::EventKind::DeviceLost(id) => {
                    println!("Device offline: {}", id);
                }
                unidrop_core::EventKind::TransferRequested(request) => {
                    println!(
                        "\nIncoming transfer from {}: {} files ({} bytes)",
                        request.from.name(),
                        request.file_count(),
                        request.total_size
                    );
                    // 自动接受
                    println!("Auto-accepting...");
                }
                unidrop_core::EventKind::TransferCompleted { transfer_id } => {
                    println!("Transfer completed: {}", transfer_id);
                }
                unidrop_core::EventKind::TransferFailed { transfer_id, error } => {
                    println!("Transfer failed: {} - {}", transfer_id, error);
                }
                _ => {}
            }
        }
    });

    tokio::signal::ctrl_c().await?;

    println!("\nShutting down...");
    engine.stop().await?;

    Ok(())
}
