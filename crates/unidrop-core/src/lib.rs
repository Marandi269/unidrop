//! UniDrop Core - 协议无关的核心抽象层
//!
//! 本模块定义了所有协议实现必须遵循的 trait 和数据类型。
//! 上层业务代码只依赖这些抽象，不直接依赖具体协议实现。

pub mod device;
pub mod error;
pub mod event;
pub mod protocol;
pub mod transfer;

pub use device::{Device, DeviceId, DeviceType, Peer};
pub use error::{Error, Result};
pub use event::{Event, EventKind};
pub use protocol::{Protocol, ProtocolBuilder, ProtocolConfig, ProtocolFactory, ProtocolId, ProtocolInfo};
pub use transfer::{
    AcceptPolicy, FileInfo, TransferIntent, TransferProgress, TransferRequest, TransferState,
};
