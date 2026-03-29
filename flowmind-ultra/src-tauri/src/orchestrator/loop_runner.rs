use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use crate::db::store::{DbState, FILE_SNAPSHOTS, SNAPSHOT_TIMELINE};
use crate::workers::history::generate_unified_diff;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::llm::client::LlmClient;
use crate::llm::ledger::LedgerManager;
use crate::workers::manager::ClusterManager;
use crate::workers::executor::WorkerTask;
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
    ignored_dirs: Option<Vec<String>>,
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
    let artifact_dir = run_origin(&workspace_dir, &prompt, ignored_dirs.clone()).map_err(|e| e.to_string())?;
    ctx.artifact_dir = Some(artifact_dir.clone());
    emit_station("Origin", "Complete", None);

    let codebase_context = std::fs::read_to_string(format!("{}/context.md", artifact_dir)).unwrap_or_default();

    // Node 2: SpecFactory
    ctx.active_state = SwarmState::SpecFactory;
    emit_station("SpecFactory", "Active", None);
    let prd = run_spec_factory(&client, &overseer_model, &prompt, &codebase_context).await?;
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
        
        // RAG Retrieval
        let mut memory_context = None;
        let cache_dir = app.path().app_cache_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let model_path = cache_dir.join("model.safetensors"); // Assume pre-downloaded or handled by init
        let tokenizer_path = cache_dir.join("tokenizer.json");
        
        if let Ok(engine) = crate::llm::embeddings::EmbeddingEngine::new(&model_path, &tokenizer_path) {
            if let Ok(vdb) = crate::db::vector::VectorDB::init(&workspace_dir).await {
                if let Ok(table) = vdb.get_or_create_table("swarm_memory", 384).await {
                    let query_vec = engine.generate(&chunk.description).unwrap_or_default();
                    if let Ok(results) = table.query().nearest_to(query_vec).limit(3).execute().await {
                         let text_results: Vec<String> = results.columns()[2] // "text" column
                            .as_any().downcast_ref::<arrow_array::StringArray>().unwrap()
                            .iter().flatten().map(|s| s.to_string()).collect();
                         memory_context = Some(text_results.join("\n---\n"));
                         let _ = app.emit("memory_retrieved", memory_context.clone());
                    }
                }
            }
        }

        let current_ledger = ledger.read().unwrap_or_default();
        let graph = run_planner(&client, &planner_model, &chunk.description, &current_ledger, memory_context).await?;
        chunk.dependency_graph = Some(graph.clone());
        emit_station("Planner", "Complete", None);

        // Node 5: Commander
        ctx.active_state = SwarmState::Commander;
        emit_station("Commander", "Active", Some("Routing dependencies...".into()));
        let plan = run_commander(&client, &executor_model, &graph).await;
        chunk.execution_plan = Some(plan.clone());
        emit_station("Commander", "AwaitingApproval", Some("Awaiting your authorization".into()));

        // 90. Suspend execution until the user manually triggers Tauri command: approve_commander_plan
        let approval = app.state::<crate::orchestrator::state::SwarmOrchestratorState>();
        approval.commander_approval.notified().await;
        emit_station("Commander", "Complete", None);

        // Node 6: Worker Cluster (Execution)
        ctx.active_state = SwarmState::Executor;
        emit_station("Executor", "Active", Some(format!("Executing Cluster Actions for Chunk {}", chunk.id)));

        // --- EPIC 4: CHECKPOINTING ---
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if let Ok(db_state) = app.try_state::<DbState>() {
            if let Ok(write_txn) = db_state.db.begin_write() {
                {
                    let _ = write_txn.open_table(SNAPSHOT_TIMELINE).map(|mut timeline| {
                        let _ = timeline.insert(ts, chunk.title.as_str());
                    });
                    
                    let _ = write_txn.open_table(FILE_SNAPSHOTS).map(|mut snapshots| {
                        if let Some(graph) = &chunk.dependency_graph {
                            for file_node in &graph.files {
                                let full_path = std::path::Path::new(&workspace_dir).join(&file_node.filepath);
                                if full_path.exists() {
                                    if let Ok(content) = std::fs::read_to_string(&full_path) {
                                        let compressed = zstd::encode_all(content.as_bytes(), 3).unwrap();
                                        let _ = snapshots.insert((ts, file_node.filepath.as_str()), compressed.as_slice());
                                    }
                                }
                            }
                        }
                    });
                }
                let _ = write_txn.commit();
            }
        }
        // -----------------------------

        let cluster_mgr = app.state::<Arc<ClusterManager>>();
        let mut tasks = Vec::new();
        
        if let Some(plan) = &chunk.execution_plan {
            for wc in &plan.wizard_clusters {
                tasks.push(WorkerTask {
                    id: format!("wizard-{}-{}", chunk.id, wc.title),
                    title: wc.title.clone(),
                    files: wc.files.clone(),
                    status: "Pending".into(),
                    cwd: workspace_dir.clone(),
                    model: executor_model.clone(),
                });
            }
            for sp in &plan.specialist_pairs {
                tasks.push(WorkerTask {
                    id: format!("specialist-{}-{}", chunk.id, sp.producer_file),
                    title: format!("Pair: {} -> {}", sp.producer_file, sp.consumer_file),
                    files: vec![sp.producer_file.clone(), sp.consumer_file.clone()],
                    status: "Pending".into(),
                    cwd: workspace_dir.clone(),
                    model: executor_model.clone(),
                });
            }
            for sf in &plan.swarm_files {
                tasks.push(WorkerTask {
                    id: format!("swarm-file-{}-{}", chunk.id, sf.filepath),
                    title: format!("Config: {}", sf.filepath),
                    files: vec![sf.filepath.clone()],
                    status: "Pending".into(),
                    cwd: workspace_dir.clone(),
                    model: executor_model.clone(),
                });
            }
        }

        cluster_mgr.enqueue(tasks).await;

        // Block progression until cluster queue is fully drained
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            if cluster_mgr.is_idle().await {
                break;
            }
        }
        
        emit_station("Executor", "Complete", None);

        // 65. QA Station: Headless Compiler Loop
        ctx.active_state = SwarmState::QaReviewer;
        emit_station("QA", "Active", Some("Running Headless Compiler Check".into()));

        let mut attempts = 0;
        let max_attempts = 3;

        while let Err(stderr_dump) = crate::workers::compiler::run_compiler_check(&workspace_dir, app.clone()).await {
            attempts += 1;
            if attempts > max_attempts {
                // Fallback Escalation
                emit_station("QA", "AwaitingHumanFix", Some("Max healing attempts reached. Awaiting manual override.".into()));
                let approval = app.state::<crate::orchestrator::state::SwarmOrchestratorState>();
                approval.compiler_approval.notified().await;
                break; // Proceed after user unblocks
            }

            ctx.active_state = SwarmState::SelfHealing { attempt: attempts, max_attempts };
            emit_station("QA", "Healing", Some(format!("Compiler Error Detected. Initiating Sniper Node (Attempt {}/{})", attempts, max_attempts)));
            
            let diags = crate::workers::compiler::parse_compiler_errors(&stderr_dump);
            
            if let Some(target) = diags.first() {
                emit_station("QA", "Healing", Some(format!("Sniper targeting: {}", target.file_path)));
                
                if let Ok(fixed_code) = crate::llm::sniper::generate_patch(&target.file_path, &target.error_message, &workspace_dir).await {
                    
                    // Lock Bypass
                    cluster_mgr.conflict_mgr.force_acquire_lock(&target.file_path).await;
                    
                    // File Overwrite (apply patch to disk)
                    let target_full_path = format!("{}/{}", workspace_dir, target.file_path);
                    let _ = std::fs::write(&target_full_path, fixed_code);
                    
                    // Immediately Release Lock
                    cluster_mgr.conflict_mgr.release(&[target.file_path.clone()]).await;
                    
                    // Ledger Annotation
                    ledger.append(&format!("\n[Auto-Heal]: Sniper repaired compilation error via JSON diff in `{}`", target.file_path)).unwrap();
                }
            } else {
                // If it couldn't parse the exact file causing the issue, just skip sniper and break to human
                emit_station("QA", "AwaitingHumanFix", Some("Unparseable Compiler Error. Awaiting explicit manual override.".into()));
                let approval = app.state::<crate::orchestrator::state::SwarmOrchestratorState>();
                approval.compiler_approval.notified().await;
                break;
            }
        }

        ledger.append(&format!("\nSprint {} Completed: {}", chunk.id, chunk.title)).unwrap();
        
        // Memory Ingestion Phase (Step 158)
        emit_station("QA", "Complete", Some("✅ Compilation successful. Codebase stabilized.".into()));
        
        let workspace_dir_clone = workspace_dir.clone();
        let app_handle_clone = app.clone();
        tauri::async_runtime::spawn(async move {
             let cache_dir = app_handle_clone.path().app_cache_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
             let m_path = cache_dir.join("model.safetensors");
             let t_path = cache_dir.join("tokenizer.json");
             if let Ok(engine) = crate::llm::embeddings::EmbeddingEngine::new(&m_path, &t_path) {
                 if let Ok(vdb) = crate::db::vector::VectorDB::init(&workspace_dir_clone).await {
                     let _ = crate::llm::memory_indexer::index_workspace(&workspace_dir_clone, &engine, &vdb).await;
                 }
             }
        });
    }

    ctx.active_state = SwarmState::Complete;
    emit_station("System", "Complete", Some("Swarm Orchestration Finished".into()));

    Ok(())
}
