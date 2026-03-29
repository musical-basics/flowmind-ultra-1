use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;

pub struct StabilityMonitor {
    pub last_read_time: Arc<Mutex<Instant>>,
    pub is_idle: Arc<Mutex<bool>>,
}

impl StabilityMonitor {
    pub fn new() -> Self {
        Self {
            last_read_time: Arc::new(Mutex::new(Instant::now())),
            is_idle: Arc::new(Mutex::new(true)),
        }
    }

    pub async fn notify_activity(&self, _byte_count: usize) {
        let mut last = self.last_read_time.lock().await;
        *last = Instant::now();
        
        let mut idle = self.is_idle.lock().await;
        if *idle {
            *idle = false;
        }
    }
}
