use tauri::AppHandle;
use super::loop_runner::start_orchestration;

#[tauri::command]
pub async fn start_swarm(
    app: AppHandle,
    workspace_dir: String,
    prompt: String,
    overseer_model: String,
    planner_model: String,
    executor_model: String,
) -> Result<(), String> {
    log::info!("Received command: start_swarm");
    
    // Spawn detached to prevent blocking the Tauri IPC bridge
    tauri::async_runtime::spawn(async move {
        if let Err(e) = start_orchestration(app.clone(), workspace_dir, prompt, overseer_model, planner_model, executor_model).await {
            log::error!("Swarm Failed: {}", e);
            let _ = app.emit("station_update", super::loop_runner::StationUpdate {
                station: "System".to_string(),
                status: "Failed".to_string(),
                detail: Some(e),
            });
        }
    });
    
    Ok(())
}

#[tauri::command]
pub async fn approve_commander_plan(state: tauri::State<'_, crate::orchestrator::state::SwarmOrchestratorState>) -> Result<(), String> {
    state.commander_approval.notify_one();
    Ok(())
}
