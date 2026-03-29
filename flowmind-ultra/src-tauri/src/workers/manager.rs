use std::sync::Arc;
use tokio::sync::Mutex;
use super::executor::{ExecutionWorker, WorkerTask};
use super::conflict::ConflictManager;
use tauri::{AppHandle, Emitter};

pub struct ClusterManager {
    queue: Arc<Mutex<Vec<WorkerTask>>>,
    pub conflict_mgr: Arc<ConflictManager>,
    pub workers: Arc<Mutex<Vec<Arc<Mutex<ExecutionWorker>>>>>,
    pub is_paused: Arc<Mutex<bool>>,
    app: AppHandle,
}

impl ClusterManager {
    pub async fn init(app: AppHandle) -> Arc<Self> {
        let mgr = Arc::new(Self {
            queue: Arc::new(Mutex::new(Vec::new())),
            conflict_mgr: Arc::new(ConflictManager::new()),
            workers: Arc::new(Mutex::new(Vec::new())),
            is_paused: Arc::new(Mutex::new(false)),
            app: app.clone(),
        });

        let mut lock = mgr.workers.lock().await;
        for i in 1..=3 {
            let worker = Arc::new(Mutex::new(ExecutionWorker::new(format!("W{}", i), app.clone())));
            lock.push(worker);
        }
        drop(lock);

        let mgr_clone = mgr.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                mgr_clone.try_dispatch().await;
            }
        });

        mgr
    }

    pub async fn enqueue(&self, mut tasks: Vec<WorkerTask>) {
        let mut q = self.queue.lock().await;
        for t in &mut tasks {
            t.status = "Pending".to_string();
        }
        q.extend(tasks);
        self.notify_queue_depth(q.len());
    }

    pub async fn try_dispatch(&self) {
        if *self.is_paused.lock().await { return; }
        
        let mut q = self.queue.lock().await;
        if q.is_empty() { return; }

        let workers = self.workers.lock().await;
        let mut i = 0;
        
        while i < q.len() {
            let task = &q[i];

            if !self.conflict_mgr.attempt_lock(&task.files).await {
                i += 1;
                continue;
            }

            let mut assigned = false;
            for worker_arc in workers.iter() {
                let worker = worker_arc.lock().await;
                if worker.current_task.lock().await.is_none() {
                    assigned = true;
                    drop(worker);
                    
                    let t = q.remove(i);
                    self.notify_queue_depth(q.len());
                    
                    let cmgr = self.conflict_mgr.clone();
                    let w_arc = worker_arc.clone();
                    
                    tokio::spawn(async move {
                        let mut w = w_arc.lock().await;
                        let _ = w.run_task(t.clone()).await;
                        cmgr.release(&t.files).await;
                    });
                    break;
                }
            }

            if !assigned {
                self.conflict_mgr.release(&task.files).await;
                break;
            }
        }
    }

    fn notify_queue_depth(&self, depth: usize) {
        #[derive(serde::Serialize, Clone)]
        struct QueueDepthEvent { depth: usize }
        let _ = self.app.emit("queue_depth", QueueDepthEvent { depth });
    }

    pub async fn is_idle(&self) -> bool {
        let q = self.queue.lock().await;
        if !q.is_empty() { return false; }
        
        let workers = self.workers.lock().await;
        for w in workers.iter() {
            let t = w.lock().await.current_task.lock().await.clone();
            if t.is_some() { return false; }
        }
        true
    }

    pub async fn cleanup(&self) {
        let workers = self.workers.lock().await;
        for w_arc in workers.iter() {
            let worker = w_arc.lock().await;
            if let Some(pty) = &worker.pty_session {
                pty.lock().await.kill().await;
            }
        }
    }
}
