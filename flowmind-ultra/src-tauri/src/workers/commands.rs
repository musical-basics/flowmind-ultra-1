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
pub async fn exec_global_script(manager: State<'_, Arc<ClusterManager>>, script: String) -> Result<(), String> {
    manager.enqueue(vec![WorkerTask {
        id: "global-override".into(),
        title: format!("Global Execute: {}", script),
        files: vec![],
        status: "Pending".into()
    }]).await;
    Ok(())
}
