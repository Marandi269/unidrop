//! P2P 网络行为定义

use std::time::Duration;
use libp2p::{
    dcutr, identify, ping, relay,
    request_response::{self, ProtocolSupport, cbor::Behaviour as CborBehaviour},
    swarm::NetworkBehaviour,
    StreamProtocol,
};
use serde::{Deserialize, Serialize};

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2pFileInfo {
    /// 文件 ID
    pub id: String,
    /// 文件名
    pub name: String,
    /// 文件大小
    pub size: u64,
    /// MIME 类型
    pub mime_type: Option<String>,
}

/// 文件传输请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRequest {
    /// 传输 ID
    pub transfer_id: String,
    /// 文件列表
    pub files: Vec<P2pFileInfo>,
}

/// 文件传输响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResponse {
    /// 传输 ID
    pub transfer_id: String,
    /// 是否接受
    pub accepted: bool,
    /// 消息
    pub message: Option<String>,
}

/// 文件数据块（支持分块传输）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    /// 传输 ID
    pub transfer_id: String,
    /// 文件 ID
    pub file_id: String,
    /// 文件名
    pub file_name: String,
    /// 块索引
    pub chunk_index: u64,
    /// 总块数
    pub total_chunks: u64,
    /// 数据内容（最大 64KB 以确保在 128KB 中继限制内）
    pub data: Vec<u8>,
}

/// 文件数据确认
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunkAck {
    /// 传输 ID
    pub transfer_id: String,
    /// 文件 ID
    pub file_id: String,
    /// 块索引
    pub chunk_index: u64,
    /// 是否成功
    pub success: bool,
}

/// 默认块大小 (64KB)
pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

/// P2P 客户端行为
#[derive(NetworkBehaviour)]
pub struct P2pClientBehaviour {
    /// 中转客户端
    pub relay_client: relay::client::Behaviour,
    /// 打洞协议
    pub dcutr: dcutr::Behaviour,
    /// 节点识别
    pub identify: identify::Behaviour,
    /// 心跳
    pub ping: ping::Behaviour,
    /// 文件传输请求/响应
    pub file_transfer: CborBehaviour<FileRequest, FileResponse>,
    /// 文件数据传输（分块）
    pub file_data: CborBehaviour<FileChunk, FileChunkAck>,
}

impl P2pClientBehaviour {
    pub fn new(
        relay_client: relay::client::Behaviour,
        keypair: &libp2p::identity::Keypair,
    ) -> Self {
        Self {
            relay_client,
            dcutr: dcutr::Behaviour::new(keypair.public().to_peer_id()),
            identify: identify::Behaviour::new(identify::Config::new(
                "/unidrop/1.0.0".to_string(),
                keypair.public(),
            )),
            ping: ping::Behaviour::new(
                ping::Config::default().with_interval(Duration::from_secs(15))
            ),
            file_transfer: CborBehaviour::new(
                [(
                    StreamProtocol::new("/unidrop/file/1.0.0"),
                    ProtocolSupport::Full,
                )],
                request_response::Config::default(),
            ),
            file_data: CborBehaviour::new(
                [(
                    StreamProtocol::new("/unidrop/data/1.0.0"),
                    ProtocolSupport::Full,
                )],
                request_response::Config::default(),
            ),
        }
    }
}
