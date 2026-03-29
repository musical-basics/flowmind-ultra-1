use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use super::session::TerminalSession;

pub struct TerminalManager {
    // Session ID -> Active TerminalSession
    pub sessions: Arc<Mutex<HashMap<String, Arc<Mutex<TerminalSession>>>>>,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_session(&self, id: String, session: Arc<Mutex<TerminalSession>>) {
        let mut lock = self.sessions.lock().await;
        lock.insert(id, session);
    }

    pub async fn get_session(&self, id: &str) -> Option<Arc<Mutex<TerminalSession>>> {
        let lock = self.sessions.lock().await;
        lock.get(id).cloned()
    }

    pub async fn remove_session(&self, id: &str) -> Option<Arc<Mutex<TerminalSession>>> {
        let mut lock = self.sessions.lock().await;
        lock.remove(id)
    }

    pub async fn kill_all(&self) {
        let mut lock = self.sessions.lock().await;
        for (_, session) in lock.drain() {
            session.lock().await.kill().await;
        }
    }
}
