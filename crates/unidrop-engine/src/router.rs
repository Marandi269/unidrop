//! 传输路由器 - 根据设备自动选择协议

use std::sync::Arc;
use tracing::{debug, warn};

use unidrop_core::{Device, DeviceId, Protocol, ProtocolId, Result, TransferIntent};

use crate::ProtocolRegistry;

/// 传输路由器
///
/// 负责根据目标设备的协议类型，将传输请求路由到正确的协议实现。
pub struct TransferRouter {
    registry: Arc<ProtocolRegistry>,
}

impl TransferRouter {
    pub fn new(registry: Arc<ProtocolRegistry>) -> Self {
        Self { registry }
    }

    /// 发送文件 - 自动路由到正确的协议
    pub async fn send(&self, intent: TransferIntent) -> Result<String> {
        let protocol_id = &intent.target.protocol;

        debug!("Routing transfer to protocol: {}", protocol_id);

        let protocol = self
            .registry
            .get(protocol_id)
            .ok_or_else(|| unidrop_core::Error::ProtocolNotFound(protocol_id.to_string()))?;

        if !protocol.is_running() {
            return Err(unidrop_core::Error::Protocol(format!(
                "Protocol {} is not running",
                protocol_id
            )));
        }

        protocol.send(intent).await
    }

    /// 根据设备选择最佳协议
    ///
    /// 如果同一设备在多个协议中都可见，选择优先级最高的
    pub fn select_protocol(&self, device_id: &DeviceId) -> Option<Arc<dyn Protocol>> {
        self.registry.get(&device_id.protocol)
    }

    /// 获取设备可用的所有协议
    pub async fn available_protocols(&self, device_id: &DeviceId) -> Vec<ProtocolId> {
        // 目前一个设备只属于一个协议
        // 未来可以扩展为多协议支持（如同一设备同时支持 LocalSend 和 AirDrop）
        vec![device_id.protocol.clone()]
    }
}
