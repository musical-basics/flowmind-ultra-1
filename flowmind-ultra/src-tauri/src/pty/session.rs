use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use serde::Serialize;
use std::io::Read;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::Duration;
use crate::pty::stability::StabilityMonitor;

#[derive(Serialize, Clone)]
pub struct PTYPayload {
    id: String,
    data: Vec<u8>, // Send raw bytes to frontend arraybuffer
}

#[derive(Serialize, Clone)]
pub struct TerminalStatePayload {
    id: String,
    state: String,
}

pub enum TerminalState {
    Connecting,
    Online,
    Working,
    Offline,
}

pub struct TerminalSession {
    pub id: String,
    pub master: Box<dyn MasterPty + Send>,
    pub child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
    pub state: Arc<Mutex<TerminalState>>,
}

impl TerminalSession {
    pub fn spawn(id: String, app: AppHandle) -> Result<Arc<Mutex<Self>>, String> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;

        let shell = if cfg!(windows) { "powershell.exe" } else { "zsh" };
        let cmd = CommandBuilder::new(shell);
        let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;

        let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
        let child_arc = Arc::new(Mutex::new(child));
        let state = Arc::new(Mutex::new(TerminalState::Connecting));

        let session = Arc::new(Mutex::new(TerminalSession {
            id: id.clone(),
            master: pair.master,
            child: child_arc.clone(),
            state: state.clone(),
        }));

        let _ = app.emit("pty-state", TerminalStatePayload { id: id.clone(), state: "Connecting".to_string() });

        let stability = Arc::new(StabilityMonitor::new());

        let id_clone = id.clone();
        let app_clone = app.clone();
        let stability_clone = stability.clone();

        tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 8192];
            let _ = app_clone.emit("pty-state", TerminalStatePayload { id: id_clone.clone(), state: "Online".to_string() });
            
            loop {
                match reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let data = buf[..n].to_vec();
                        tauri::async_runtime::block_on(async {
                            stability_clone.notify_activity(n).await;
                        });
                        // 26. Broadcast terminal output chunk at 60fps
                        let _ = app_clone.emit("pty-output", PTYPayload {
                            id: id_clone.clone(),
                            data,
                        });
                    }
                    _ => {
                        let _ = app_clone.emit("pty-state", TerminalStatePayload { id: id_clone.clone(), state: "Offline".to_string() });
                        break;
                    }
                }
            }
        });

        // 34. Poller task for stability and semantic flush
        let id_poller = id.clone();
        let app_poller = app.clone();
        let stability_poller = stability.clone();
        tokio::spawn(async move {
            let mut flushed = false;
            loop {
                tokio::time::sleep(Duration::from_millis(300)).await;
                let last = *stability_poller.last_read_time.lock().await;
                let elapsed = last.elapsed().as_millis() as u64;

                let mut idle = stability_poller.is_idle.lock().await;
                
                if elapsed < 1000 {
                    if *idle {
                        *idle = false;
                        flushed = false;
                        let _ = app_poller.emit("pty-state", TerminalStatePayload { id: id_poller.clone(), state: "Working".to_string() });
                    }
                } else if elapsed >= 1000 && !*idle {
                    *idle = true;
                    let _ = app_poller.emit("pty-state", TerminalStatePayload { id: id_poller.clone(), state: "Online".to_string() });
                }

                if elapsed >= 3000 && !flushed {
                    flushed = true;
                    // 35. Trigger Semantic Flush
                    let _ = app_poller.emit("pty-semantic-flush", TerminalStatePayload { id: id_poller.clone(), state: "IdleFlush".to_string() });
                }
            }
        });

        Ok(session)
    }

    pub fn write(&self, data: &[u8]) -> Result<(), String> {
        let mut writer = self.master.try_clone_writer().map_err(|e| e.to_string())?;
        std::io::Write::write_all(&mut writer, data).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn resize(&self, rows: u16, cols: u16) -> Result<(), String> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn kill(&self) {
        let mut child = self.child.lock().await;
        let _ = child.kill();
    }
}
