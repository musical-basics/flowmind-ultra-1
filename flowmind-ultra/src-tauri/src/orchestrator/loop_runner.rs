use tauri::{AppHandle, Emitter};
use crate::llm::client::LlmClient;
use crate::llm::ledger::LedgerManager;
use super::state::{SwarmContext, SwarmState, SprintChunk};
use super::nodes::*;

#[derive(serde::Serialize, Clone)]
pub struct StationUpdate {
    pub station: String,
    pub status: String,
    pub detail: Option<String>,
}

#[derive(serde::Serialize, Clone)]
pub struct ChunkStart {
    pub chunk_id: usize,
    pub chunk_title: String,
    pub total_chunks: usize,
}

pub async fn start_orchestration(
    app: AppHandle,
    workspace_dir: String,
    prompt: String,
    overseer_model: String,
    planner_model: String,
    executor_model: String,
) -> Result<(), String> {
    log::info!("Starting Swarm Orchestration in {}", workspace_dir);

    // 1. Initialize Context
    let mut ctx = SwarmContext::new(workspace_dir.clone(), prompt.clone());
    
    // Fallback to Env if store keys aren't explicitly passed (or implement key passing via Bridge later)
    let client = LlmClient::new(
        std::env::var("OPENROUTER_API_KEY").ok(),
        std::env::var("ANTHROPIC_API_KEY").ok()
    );

    let emit_station = |station: &str, status: &str, detail: Option<String>| {
        let _ = app.emit("station_update", StationUpdate {
            station: station.to_string(),
            status: status.to_string(),
            detail,
        });
    };

    // Node 1: Origin
    ctx.active_state = SwarmState::Origin;
    emit_station("Origin", "Active", None);
    let artifact_dir = run_origin(&workspace_dir, &prompt).map_err(|e| e.to_string())?;
    ctx.artifact_dir = Some(artifact_dir.clone());
    emit_station("Origin", "Complete", None);

    // Node 2: SpecFactory
    ctx.active_state = SwarmState::SpecFactory;
    emit_station("SpecFactory", "Active", None);
    let prd = run_spec_factory(&client, &overseer_model, &prompt).await?;
    std::fs::write(format!("{}/prd.md", artifact_dir), &prd).unwrap();
    ctx.prd_markdown = Some(prd.clone());
    emit_station("SpecFactory", "Complete", None);

    // Node 3: Overseer
    ctx.active_state = SwarmState::Overseer;
    emit_station("Overseer", "Active", None);
    let overseer_output = run_overseer(&client, &overseer_model, &prd).await?;
    ctx.sprint_chunks = overseer_output.sprints.into_iter().map(|s| SprintChunk {
        id: s.id,
        title: s.title,
        description: s.description,
        dependency_graph: None,
        execution_plan: None,
    }).collect();
    emit_station("Overseer", "Complete", Some(format!("Generated {} sprints", ctx.sprint_chunks.len())));

    let ledger = LedgerManager::new(&workspace_dir);

    // 55. Master Async Orchestration Loop
    for (i, chunk) in ctx.sprint_chunks.iter_mut().enumerate() {
        ctx.current_chunk_idx = i;
        
        // 60. Granular status emits
        let _ = app.emit("chunk_start", ChunkStart {
            chunk_id: chunk.id,
            chunk_title: chunk.title.clone(),
            total_chunks: ctx.sprint_chunks.len(),
        });

        // Node 4: Planner
        ctx.active_state = SwarmState::Planner;
        emit_station("Planner", "Active", Some(format!("Planning Chunk {}", chunk.id)));
        let current_ledger = ledger.read().unwrap_or_default();
        let graph = run_planner(&client, &planner_model, &chunk.description, &current_ledger).await?;
        chunk.dependency_graph = Some(graph.clone());
        emit_station("Planner", "Complete", None);

        // Node 5: Commander
        ctx.active_state = SwarmState::Commander;
        emit_station("Commander", "Active", Some("Routing dependencies...".into()));
        let plan = run_commander(&client, &executor_model, &graph).await;
        chunk.execution_plan = Some(plan.clone());
        emit_station("Commander", "Complete", None);

        /* 
          // Future Phase 8 integration
          ctx.active_state = SwarmState::Executor;
          emit_station("Executor", "Active", Some("Deploying agents...".into()));
          run_executor_clusters(...).await;
          emit_station("Executor", "Complete", None);
        */

        // Simulate execution delay for UI testing
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        ledger.append(&format!("Sprint {} Completed: {}", chunk.id, chunk.title)).unwrap();
    }

    ctx.active_state = SwarmState::Complete;
    emit_station("System", "Complete", Some("Swarm Orchestration Finished".into()));

    Ok(())
}
