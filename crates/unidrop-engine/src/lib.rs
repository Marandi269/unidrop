//! UniDrop Engine - 协议路由与管理
//!
//! Engine 是上层业务与协议实现之间的中间层。
//! 它负责：
//! - 协议注册与生命周期管理
//! - 设备聚合（合并多个协议发现的设备）
//! - 传输路由（根据设备自动选择协议）
//! - 事件聚合（统一分发各协议事件）

mod engine;
mod registry;
mod router;

pub use engine::{Engine, EngineBuilder, EngineConfig};
pub use registry::ProtocolRegistry;
pub use router::TransferRouter;
