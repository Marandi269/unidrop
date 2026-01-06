//! 协议注册表 - 管理所有已注册的协议

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

use unidrop_core::{Protocol, ProtocolFactory, ProtocolId, ProtocolInfo, Result};

/// 协议注册表
///
/// 维护所有已注册的协议及其工厂，支持动态添加/移除协议。
pub struct ProtocolRegistry {
    /// 协议工厂映射
    factories: RwLock<HashMap<ProtocolId, Arc<dyn ProtocolFactory>>>,
    /// 已创建的协议实例
    instances: RwLock<HashMap<ProtocolId, Arc<dyn Protocol>>>,
}

impl ProtocolRegistry {
    pub fn new() -> Self {
        Self {
            factories: RwLock::new(HashMap::new()),
            instances: RwLock::new(HashMap::new()),
        }
    }

    /// 注册协议工厂
    pub fn register<F: ProtocolFactory + 'static>(&self, factory: F) {
        self.register_arc(Arc::new(factory));
    }

    /// 注册协议工厂（Arc 版本）
    pub fn register_arc(&self, factory: Arc<dyn ProtocolFactory>) {
        let info = factory.info();
        let id = info.id.clone();

        if !info.supported {
            warn!(
                "Protocol {} is not supported on this platform, skipping",
                id
            );
            return;
        }

        info!("Registering protocol: {} v{}", info.name, info.version);
        self.factories.write().insert(id, factory);
    }

    /// 注销协议
    pub fn unregister(&self, id: &ProtocolId) -> bool {
        let removed = self.factories.write().remove(id).is_some();
        if removed {
            // 同时移除实例
            self.instances.write().remove(id);
            info!("Unregistered protocol: {}", id);
        }
        removed
    }

    /// 获取所有已注册的协议信息
    pub fn list(&self) -> Vec<ProtocolInfo> {
        self.factories
            .read()
            .values()
            .map(|f| f.info())
            .collect()
    }

    /// 检查协议是否已注册
    pub fn contains(&self, id: &ProtocolId) -> bool {
        self.factories.read().contains_key(id)
    }

    /// 获取或创建协议实例
    pub fn get_or_create(&self, id: &ProtocolId) -> Option<Arc<dyn Protocol>> {
        // 先检查是否已有实例
        if let Some(instance) = self.instances.read().get(id) {
            return Some(instance.clone());
        }

        // 创建新实例
        let factory = self.factories.read().get(id)?.clone();
        let instance = factory.create();

        debug!("Created protocol instance: {}", id);
        self.instances.write().insert(id.clone(), instance.clone());

        Some(instance)
    }

    /// 获取已创建的协议实例
    pub fn get(&self, id: &ProtocolId) -> Option<Arc<dyn Protocol>> {
        self.instances.read().get(id).cloned()
    }

    /// 获取所有已创建的协议实例
    pub fn instances(&self) -> Vec<Arc<dyn Protocol>> {
        self.instances.read().values().cloned().collect()
    }

    /// 按优先级排序的协议列表
    pub fn sorted_by_priority(&self) -> Vec<ProtocolInfo> {
        let mut list = self.list();
        list.sort_by(|a, b| b.priority.cmp(&a.priority));
        list
    }
}

impl Default for ProtocolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用的 Mock 协议实现会在后面添加
}
