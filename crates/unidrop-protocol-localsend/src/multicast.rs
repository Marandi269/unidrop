//! LocalSend UDP Multicast 设备发现
//!
//! LocalSend 使用 UDP multicast 进行设备发现：
//! - 组播地址：224.0.0.167
//! - 端口：53317（与 HTTPS 端口相同）
//! - 消息格式：JSON (MulticastDto)

use parking_lot::RwLock;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use unidrop_core::{Device, DeviceType, Event, Peer, ProtocolId};

use crate::models::DeviceInfo;
use crate::{MULTICAST_ADDR, PROTOCOL_ID};

/// Multicast 消息结构（与 LocalSend 兼容）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MulticastDto {
    pub alias: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "deviceModel", skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    pub fingerprint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    /// v1 字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement: Option<bool>,
    /// v2 字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce: Option<bool>,
}

impl MulticastDto {
    /// 创建公告消息
    pub fn announcement(info: &DeviceInfo) -> Self {
        Self {
            alias: info.alias.clone(),
            version: Some(info.version.clone()),
            device_model: Some("UniDrop".to_string()),
            device_type: Some("desktop".to_string()),
            fingerprint: info.fingerprint.clone(),
            port: Some(info.port),
            protocol: Some(info.protocol.clone()),
            download: Some(false),
            announcement: Some(true),
            announce: Some(true),
        }
    }

    /// 创建响应消息（非公告）
    pub fn response(info: &DeviceInfo) -> Self {
        Self {
            alias: info.alias.clone(),
            version: Some(info.version.clone()),
            device_model: Some("UniDrop".to_string()),
            device_type: Some("desktop".to_string()),
            fingerprint: info.fingerprint.clone(),
            port: Some(info.port),
            protocol: Some(info.protocol.clone()),
            download: Some(false),
            announcement: Some(false),
            announce: Some(false),
        }
    }

    /// 判断是否为公告消息
    pub fn is_announcement(&self) -> bool {
        self.announcement.unwrap_or(false) || self.announce.unwrap_or(false)
    }
}

/// UDP Multicast 发现服务
pub struct MulticastDiscovery {
    local_info: DeviceInfo,
    devices: Arc<RwLock<HashMap<String, Device>>>,
    event_tx: mpsc::Sender<Event>,
    socket: Option<UdpSocket>,
}

impl MulticastDiscovery {
    pub fn new(local_info: DeviceInfo, event_tx: mpsc::Sender<Event>) -> Self {
        Self {
            local_info,
            devices: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            socket: None,
        }
    }

    /// 启动 multicast 发现服务
    pub fn start(&mut self) -> unidrop_core::Result<()> {
        info!("Starting LocalSend multicast discovery service");

        let port = self.local_info.port;
        let multicast_addr: Ipv4Addr = MULTICAST_ADDR.parse().unwrap();

        // 绑定到 multicast 端口
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
            .map_err(|e| unidrop_core::Error::Network(format!("Failed to bind UDP socket: {}", e)))?;

        // 设置为非阻塞模式用于接收
        socket.set_nonblocking(true)
            .map_err(|e| unidrop_core::Error::Network(format!("Failed to set non-blocking: {}", e)))?;

        // 加入组播组
        socket.join_multicast_v4(&multicast_addr, &Ipv4Addr::UNSPECIFIED)
            .map_err(|e| unidrop_core::Error::Network(format!("Failed to join multicast group: {}", e)))?;

        info!("Joined multicast group {}:{}", MULTICAST_ADDR, port);

        // 保存 socket 用于发送
        let send_socket = socket.try_clone()
            .map_err(|e| unidrop_core::Error::Network(format!("Failed to clone socket: {}", e)))?;
        self.socket = Some(send_socket);

        // 启动接收线程
        let local_fingerprint = self.local_info.fingerprint.clone();
        let local_info = self.local_info.clone();
        let devices = self.devices.clone();
        let event_tx = self.event_tx.clone();

        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((len, src)) => {
                        if let Ok(json_str) = std::str::from_utf8(&buf[..len]) {
                            debug!("Received multicast from {}: {}", src, json_str);
                            if let Ok(dto) = serde_json::from_str::<MulticastDto>(json_str) {
                                // 跳过自己
                                if dto.fingerprint == local_fingerprint {
                                    continue;
                                }

                                let ip = src.ip();
                                let port = dto.port.unwrap_or(53317);

                                // 解析设备类型
                                let device_type = dto.device_type
                                    .as_deref()
                                    .map(DeviceType::from_str)
                                    .unwrap_or(DeviceType::Desktop);

                                let peer = Peer::new(
                                    ProtocolId::new(PROTOCOL_ID),
                                    dto.fingerprint.clone(),
                                    dto.alias.clone(),
                                )
                                .with_device_type(device_type)
                                .with_version(dto.version.clone().unwrap_or_else(|| "2.0".to_string()));

                                let device = Device::new(peer, ip, port);

                                let fingerprint = dto.fingerprint.clone();
                                let is_new = !devices.read().contains_key(&fingerprint);

                                info!(
                                    "Discovered device via multicast: {} ({}:{})",
                                    dto.alias, ip, port
                                );
                                devices.write().insert(fingerprint.clone(), device.clone());

                                let evt = if is_new {
                                    Event::device_discovered(device)
                                } else {
                                    Event::new(unidrop_core::EventKind::DeviceUpdated(device))
                                };

                                let _ = event_tx.blocking_send(evt);

                                // 如果是公告消息，回复响应
                                if dto.is_announcement() {
                                    debug!("Responding to announcement from {}", dto.alias);
                                    let response = MulticastDto::response(&local_info);
                                    if let Ok(response_json) = serde_json::to_string(&response) {
                                        let target = SocketAddr::new(ip, port);
                                        // 发送到组播地址
                                        let multicast_target = SocketAddr::new(
                                            IpAddr::V4(MULTICAST_ADDR.parse().unwrap()),
                                            port,
                                        );
                                        if let Ok(sock) = UdpSocket::bind("0.0.0.0:0") {
                                            let _ = sock.send_to(response_json.as_bytes(), multicast_target);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // 没有数据，短暂休眠
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Err(e) => {
                        warn!("Multicast receive error: {}", e);
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                }
            }
        });

        // 发送公告
        self.send_announcement()?;

        Ok(())
    }

    /// 发送公告消息
    pub fn send_announcement(&self) -> unidrop_core::Result<()> {
        let socket = match &self.socket {
            Some(s) => s,
            None => return Err(unidrop_core::Error::Network("Socket not initialized".into())),
        };

        let dto = MulticastDto::announcement(&self.local_info);
        let json = serde_json::to_string(&dto)
            .map_err(|e| unidrop_core::Error::Protocol(e.to_string()))?;

        let target = SocketAddr::new(
            IpAddr::V4(MULTICAST_ADDR.parse().unwrap()),
            self.local_info.port,
        );

        // 发送多次公告（100ms, 500ms, 2000ms 后）
        let socket_clone = socket.try_clone()
            .map_err(|e| unidrop_core::Error::Network(e.to_string()))?;
        let json_clone = json.clone();

        std::thread::spawn(move || {
            for delay in [100, 500, 2000] {
                std::thread::sleep(std::time::Duration::from_millis(delay));
                info!("Sending multicast announcement");
                if let Err(e) = socket_clone.send_to(json_clone.as_bytes(), target) {
                    warn!("Failed to send multicast announcement: {}", e);
                }
            }
        });

        Ok(())
    }

    /// 获取设备列表
    pub fn devices(&self) -> Vec<Device> {
        self.devices.read().values().cloned().collect()
    }

    /// 根据指纹获取设备
    pub fn device(&self, fingerprint: &str) -> Option<Device> {
        self.devices.read().get(fingerprint).cloned()
    }
}
