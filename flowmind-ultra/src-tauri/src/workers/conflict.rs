use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ConflictManager {
    active_files: Arc<Mutex<HashSet<String>>>,
}

impl ConflictManager {
    pub fn new() -> Self {
        Self { active_files: Arc::new(Mutex::new(HashSet::new())) }
    }

    pub async fn attempt_lock(&self, files: &[String]) -> bool {
        let mut lock = self.active_files.lock().await;
        // Check if any file is already locked by another worker
        for file in files {
            if lock.contains(file) {
                return false;
            }
        }
        for file in files {
            lock.insert(file.clone());
        }
        true
    }

    pub async fn release(&self, files: &[String]) {
        let mut lock = self.active_files.lock().await;
        for file in files {
            lock.remove(file);
        }
    }
}
