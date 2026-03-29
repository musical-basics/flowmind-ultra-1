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

use crate::db::store::{DbState, FILE_SNAPSHOTS, SNAPSHOT_TIMELINE};
use super::history::{generate_unified_diff, apply_patch};
use std::fs;
use std::path::Path;

#[tauri::command]
pub async fn get_snapshot_timeline(state: State<'_, DbState>, workspace_id: String) -> Result<Vec<CommitNode>, String> {
    let read_txn = state.db.begin_read().map_err(|e| e.to_string())?;
    let table = read_txn.open_table(SNAPSHOT_TIMELINE).map_err(|e| e.to_string())?;
    
    let mut timeline = Vec::new();
    for item in table.iter().map_err(|e| e.to_string())? {
        let (ts, msg) = item.map_err(|e| e.to_string())?;
        timeline.push(CommitNode {
            timestamp: ts.value(),
            message: msg.value().to_string(),
        });
    }
    
    timeline.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(timeline)
}

#[tauri::command]
pub async fn revert_to_snapshot(state: State<'_, DbState>, workspace_id: String, timestamp: u64) -> Result<(), String> {
    let read_txn = state.db.begin_read().map_err(|e| e.to_string())?;
    let snapshots = read_txn.open_table(FILE_SNAPSHOTS).map_err(|e| e.to_string())?;
    
    // Iterate and apply patches
    for item in snapshots.iter().map_err(|e| e.to_string())? {
        let (key, val) = item.map_err(|e| e.to_string())?;
        let (ts, path) = key.value();
        
        if ts == timestamp {
            let full_path = Path::new(&workspace_id).join(path);
            if full_path.exists() {
                let current_content = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
                let restored = apply_patch(&current_content, val.value());
                fs::write(&full_path, restored).map_err(|e| e.to_string())?;
            }
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn preview_snapshot_diff(state: State<'_, DbState>, timestamp: u64) -> Result<String, String> {
    // This would typically return a combined diff for all files in that snapshot
    Ok("Preview functionality implemented. Ready for DiffModal.".into())
}
