use tauri::{State, AppHandle};
use crate::pty::manager::TerminalManager;
use crate::pty::session::TerminalSession;

#[tauri::command]
pub async fn terminal_create(app: AppHandle, manager: State<'_, TerminalManager>, id: String, cwd: String) -> Result<(), String> {
    let session = TerminalSession::spawn(id.clone(), cwd, app)?;
    manager.add_session(id, session).await;
    Ok(())
}

#[tauri::command]
pub async fn terminal_write(manager: State<'_, TerminalManager>, id: String, data: Vec<u8>) -> Result<(), String> {
    if let Some(session) = manager.get_session(&id).await {
        session.lock().await.write(&data)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn terminal_resize(manager: State<'_, TerminalManager>, id: String, rows: u16, cols: u16) -> Result<(), String> {
    if let Some(session) = manager.get_session(&id).await {
        session.lock().await.resize(rows, cols)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn terminal_close(manager: State<'_, TerminalManager>, id: String) -> Result<(), String> {
    if let Some(session) = manager.remove_session(&id).await {
        tauri::async_runtime::spawn(async move {
            let session_lock = session.lock().await;
            session_lock.kill().await;
        });
    }
    Ok(())
}

// Stubs for wezterm-term internal virtual screen buffers

#[tauri::command]
pub async fn snapshot_ansi(_manager: State<'_, TerminalManager>, _id: String) -> Result<String, String> {
    // 28. TODO: Hook into WeztermEmulator
    Ok("".to_string())
}

#[tauri::command]
pub async fn snapshot_lines(_manager: State<'_, TerminalManager>, _id: String) -> Result<Vec<String>, String> {
    // 28. TODO: Hook into WeztermEmulator
    Ok(vec![])
}
