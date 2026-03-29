use tauri::{AppHandle, Emitter};
use crate::pty::session::TerminalSession;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Serialize, Clone, Debug)]
pub struct WorkerTask {
    pub id: String,
    pub title: String,
    pub files: Vec<String>,
    pub status: String,
    pub cwd: String,
}

#[derive(serde::Serialize, Clone)]
pub struct WorkerStatusEvent {
    pub worker_id: String,
    pub task: Option<WorkerTask>,
    pub state: String,
}

pub struct ExecutionWorker {
    pub id: String,
    pub app: AppHandle,
    pub pty_session: Option<Arc<Mutex<TerminalSession>>>,
    pub current_task: Arc<Mutex<Option<WorkerTask>>>,
}

impl ExecutionWorker {
    pub fn new(id: String, app: AppHandle) -> Self {
        Self {
            id,
            app,
            pty_session: None,
            current_task: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn run_task(&mut self, mut task: WorkerTask) -> Result<(), String> {
        task.status = "Sending".to_string();
        
        let mut t = self.current_task.lock().await;
        t.replace(task.clone());
        drop(t);

        let _ = self.app.emit("workers_status", WorkerStatusEvent {
            worker_id: self.id.clone(),
            task: Some(task.clone()),
            state: "Running".to_string(),
        });

        if self.pty_session.is_none() {
            let session = TerminalSession::spawn(format!("worker-{}", self.id), task.cwd.clone(), self.app.clone())?;
            self.pty_session = Some(session);
        }

        if let Some(pty) = &self.pty_session {
            // 75. Token limiting simulated delay
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let cmd = format!("echo 'Deploying Multi-Agent Cluster for -> {}'\nsleep 3\necho 'Agent File Access: {}'\nsleep 2\necho 'Agent Execution Completed.'\n", task.title, task.files.join(", "));
            pty.lock().await.write(cmd.as_bytes())?;
        }

        // Simulated deep execution duration
        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;

        let mut t = self.current_task.lock().await;
        *t = None;
        drop(t);

        let _ = self.app.emit("workers_status", WorkerStatusEvent {
            worker_id: self.id.clone(),
            task: None,
            state: "Idle".to_string(),
        });

        Ok(())
    }
}
