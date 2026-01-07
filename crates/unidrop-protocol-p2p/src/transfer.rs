//! 文件传输管理

use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// 传输状态
#[derive(Debug, Clone)]
pub enum TransferState {
    /// 等待接受
    Pending,
    /// 传输中
    Transferring { progress: f64 },
    /// 完成
    Completed,
    /// 失败
    Failed { error: String },
    /// 取消
    Cancelled,
}

/// 传输会话
#[derive(Debug, Clone)]
pub struct TransferSession {
    pub id: String,
    pub peer_id: String,
    pub file_name: String,
    pub file_size: u64,
    pub state: TransferState,
    pub save_path: Option<PathBuf>,
    pub bytes_transferred: u64,
}

impl TransferSession {
    pub fn new(id: String, peer_id: String, file_name: String, file_size: u64) -> Self {
        Self {
            id,
            peer_id,
            file_name,
            file_size,
            state: TransferState::Pending,
            save_path: None,
            bytes_transferred: 0,
        }
    }

    pub fn progress(&self) -> f64 {
        if self.file_size == 0 {
            return 1.0;
        }
        self.bytes_transferred as f64 / self.file_size as f64
    }
}

/// 传输管理器
pub struct TransferManager {
    sessions: Arc<RwLock<HashMap<String, TransferSession>>>,
}

impl TransferManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_session(&self, session: TransferSession) {
        self.sessions.write().insert(session.id.clone(), session);
    }

    pub fn get_session(&self, id: &str) -> Option<TransferSession> {
        self.sessions.read().get(id).cloned()
    }

    pub fn update_progress(&self, id: &str, bytes: u64) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.bytes_transferred = bytes;
            session.state = TransferState::Transferring {
                progress: session.progress(),
            };
        }
    }

    pub fn complete(&self, id: &str) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.state = TransferState::Completed;
        }
    }

    pub fn fail(&self, id: &str, error: String) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.state = TransferState::Failed { error };
        }
    }

    pub fn cancel(&self, id: &str) {
        if let Some(session) = self.sessions.write().get_mut(id) {
            session.state = TransferState::Cancelled;
        }
    }

    pub fn remove(&self, id: &str) -> Option<TransferSession> {
        self.sessions.write().remove(id)
    }

    pub fn all_sessions(&self) -> Vec<TransferSession> {
        self.sessions.read().values().cloned().collect()
    }
}

impl Default for TransferManager {
    fn default() -> Self {
        Self::new()
    }
}
