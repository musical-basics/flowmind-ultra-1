use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use serde::Serialize;
use std::io::Read;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

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

        let id_clone = id.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; 8192];
            let _ = app.emit("pty-state", TerminalStatePayload { id: id_clone.clone(), state: "Online".to_string() });
            
            loop {
                match reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let data = buf[..n].to_vec();
                        // 26. Broadcast terminal output chunk at 60fps (thottling is handled on frontend or in advance iterators)
                        let _ = app.emit("pty-output", PTYPayload {
                            id: id_clone.clone(),
                            data,
                        });
                    }
                    _ => {
                        let _ = app.emit("pty-state", TerminalStatePayload { id: id_clone.clone(), state: "Offline".to_string() });
                        break;
                    }
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
