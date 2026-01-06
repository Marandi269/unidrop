//! LocalSend mDNS 设备发现

use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use unidrop_core::{Device, DeviceId, DeviceType, Event, Peer, ProtocolId};

use crate::models::DeviceInfo;
use crate::{PROTOCOL_ID, SERVICE_TYPE};

/// 发现服务
pub struct DiscoveryService {
    local_info: DeviceInfo,
    devices: Arc<RwLock<HashMap<String, Device>>>,
    mdns: Option<ServiceDaemon>,
    event_tx: mpsc::Sender<Event>,
}

impl DiscoveryService {
    pub fn new(local_info: DeviceInfo, event_tx: mpsc::Sender<Event>) -> Self {
        Self {
            local_info,
            devices: Arc::new(RwLock::new(HashMap::new())),
            mdns: None,
            event_tx,
        }
    }

    /// 启动发现服务
    pub fn start(&mut self) -> unidrop_core::Result<()> {
        info!("Starting LocalSend discovery service");

        let mdns =
            ServiceDaemon::new().map_err(|e| unidrop_core::Error::Discovery(e.to_string()))?;

        // 注册自己的服务
        let service_name = format!("{}-{}", self.local_info.alias, &self.local_info.fingerprint[..8]);

        let mut properties = HashMap::new();
        properties.insert("alias".to_string(), self.local_info.alias.clone());
        properties.insert("fingerprint".to_string(), self.local_info.fingerprint.clone());
        properties.insert("version".to_string(), self.local_info.version.clone());
        properties.insert("protocol".to_string(), self.local_info.protocol.clone());
        properties.insert("deviceType".to_string(), "desktop".to_string());

        let host = format!("{}.local.", service_name.replace(' ', "-"));

        let service_info = ServiceInfo::new(SERVICE_TYPE, &service_name, &host, "", self.local_info.port, properties)
            .map_err(|e| unidrop_core::Error::Discovery(e.to_string()))?;

        mdns.register(service_info)
            .map_err(|e| unidrop_core::Error::Discovery(e.to_string()))?;

        info!("Registered mDNS service: {}", service_name);

        // 浏览其他设备
        let receiver = mdns
            .browse(SERVICE_TYPE)
            .map_err(|e| unidrop_core::Error::Discovery(e.to_string()))?;

        let devices = self.devices.clone();
        let event_tx = self.event_tx.clone();
        let local_fingerprint = self.local_info.fingerprint.clone();

        // 启动事件处理
        std::thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        if let Some(device) = parse_service_info(&info, &local_fingerprint) {
                            let fingerprint = device.peer.id.fingerprint.clone();
                            let is_new = !devices.read().contains_key(&fingerprint);

                            devices.write().insert(fingerprint.clone(), device.clone());

                            let evt = if is_new {
                                Event::device_discovered(device)
                            } else {
                                Event::new(unidrop_core::EventKind::DeviceUpdated(device))
                            };

                            let _ = event_tx.blocking_send(evt);
                        }
                    }
                    ServiceEvent::ServiceRemoved(_, fullname) => {
                        debug!("Service removed: {}", fullname);
                        // 尝试从 fullname 提取 fingerprint 并发送离线事件
                    }
                    _ => {}
                }
            }
        });

        self.mdns = Some(mdns);
        Ok(())
    }

    /// 停止发现服务
    pub fn stop(&mut self) {
        self.mdns = None;
        info!("LocalSend discovery service stopped");
    }

    /// 获取当前设备列表
    pub fn devices(&self) -> Vec<Device> {
        self.devices.read().values().cloned().collect()
    }

    /// 根据指纹获取设备
    pub fn device(&self, fingerprint: &str) -> Option<Device> {
        self.devices.read().get(fingerprint).cloned()
    }
}

/// 解析 mDNS 服务信息
fn parse_service_info(info: &ServiceInfo, local_fingerprint: &str) -> Option<Device> {
    let properties = info.get_properties();

    let fingerprint = properties.get("fingerprint")?.val_str();

    // 跳过自己
    if fingerprint == local_fingerprint {
        return None;
    }

    let alias = properties
        .get("alias")
        .map(|v| v.val_str().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let version = properties
        .get("version")
        .map(|v| v.val_str().to_string())
        .unwrap_or_else(|| "2.0".to_string());

    let device_type = properties
        .get("deviceType")
        .map(|v| DeviceType::from_str(v.val_str()))
        .unwrap_or(DeviceType::Desktop);

    // 获取 IP 地址
    let ip: IpAddr = info
        .get_addresses()
        .iter()
        .next()
        .copied()
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));

    let port = info.get_port();

    let peer = Peer::new(
        ProtocolId::new(PROTOCOL_ID),
        fingerprint.to_string(),
        alias,
    )
    .with_device_type(device_type)
    .with_version(version);

    Some(Device::new(peer, ip, port))
}
