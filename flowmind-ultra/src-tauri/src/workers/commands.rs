use tauri::State;
use std::sync::Arc;
use super::manager::ClusterManager;
use super::executor::WorkerTask;

#[tauri::command]
pub async fn set_worker_override(manager: State<'_, Arc<ClusterManager>>, paused: bool) -> Result<(), String> {
    let mut lock = manager.is_paused.lock().await;
    *lock = paused;
    Ok(())
}

#[tauri::command]
pub async fn exec_global_script(manager: State<'_, Arc<ClusterManager>>, script: String, cwd: String) -> Result<(), String> {
    manager.enqueue(vec![WorkerTask {
        id: "global-override".into(),
        title: format!("Global Execute: {}", script),
        files: vec![],
        status: "Pending".into(),
        cwd,
    }]).await;
    Ok(())
}

#[derive(serde::Serialize)]
pub struct CommitNode {
    pub timestamp: u64,
    pub message: String,
}

#[tauri::command]
pub async fn get_snapshot_timeline(workspace_id: String) -> Result<Vec<CommitNode>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn revert_to_snapshot(workspace_id: String, timestamp: u64) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn preview_snapshot_diff(timestamp: u64) -> Result<String, String> {
    Ok(String::new())
}
