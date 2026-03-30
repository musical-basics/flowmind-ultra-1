use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use lancedb::query::ExecutableQuery;
use crate::db::store::{DbState, FILE_SNAPSHOTS, SNAPSHOT_TIMELINE};
use crate::workers::history::generate_unified_diff;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use arrow_array::{RecordBatch, StringArray};
use futures::StreamExt;

#[derive(Serialize, Clone)]
pub struct CommitNode {
    pub timestamp: u64,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct SwarmRunUpdate {
    pub status: String,
    pub current_node: String,
    pub ledger_snapshot: String,
}

#[derive(Serialize, Clone)]
pub struct StationUpdate {
    pub station: String,
    pub status: String,
    pub detail: Option<String>,
}

pub async fn start_orchestration(
    app: AppHandle, 
    workspace_dir: String,
    _prompt: String,
    _overseer_model: String,
    _planner_model: String,
    _executor_model: String,
    _ignored_dirs: Option<Vec<String>>,
) -> Result<(), String> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        let state = app.state::<DbState>();
        let db = &state.db;

        // 1. Fetch Latest Snapshots from Redb
        let read_txn = match db.begin_read() {
            Ok(t) => t,
            _ => continue,
        };

        let snapshots_table = match read_txn.open_table(FILE_SNAPSHOTS) {
            Ok(t) => t,
            _ => continue,
        };

        let mut ledger_content = String::new();
        if let Ok(range) = snapshots_table.range::<(u64, &str)>(..) {
            for item in range {
                if let Ok((key, val)) = item {
                    let (_, path) = key.value();
                    let content = String::from_utf8_lossy(val.value());
                    ledger_content.push_str(&format!("File: {}\nContent: {}\n---\n", path, content));
                }
            }
        }

        // 2. Swarm "Intelligence" Pass: Embed & Search
        // Setup local embedding engine (BGE-Small-EN-v1.5)
        let cache_dir = app.path().app_cache_dir().unwrap_or_else(|_| std::path::PathBuf::from("/tmp"));
        let model_path = cache_dir.join("model.safetensors");
        let tokenizer_path = cache_dir.join("tokenizer.json");
        
        if let Ok(engine) = crate::llm::embeddings::EmbeddingEngine::new(&model_path, &tokenizer_path) {
            if let Ok(vdb) = crate::db::vector::VectorDB::init(&workspace_dir).await {
                if let Ok(table) = vdb.get_or_create_table("swarm_memory", 384).await {
                    let query_vec = engine.generate("significant architectural change").unwrap_or_default();
                    
                    let query_result = table.query()
                        .nearest_to(query_vec)
                        .map_err(|e| e.to_string());
                    
                    if let Ok(query) = query_result {
                        if let Ok(mut stream) = query.execute().await {
                            if let Some(Ok(batch)) = stream.next().await {
                                 let column = batch.column(2).clone();
                                 if let Some(text_array) = column.as_any().downcast_ref::<StringArray>() {
                                      let text_results: Vec<String> = text_array.iter()
                                        .flatten()
                                        .take(3)
                                        .map(|s| s.to_string())
                                        .collect();
                                      let memory_context = text_results.join("\n---\n");
                                      let _ = app.emit("memory_retrieved", Some(memory_context));
                                 }
                            }
                        }
                    }
                }
            }
        }

        // 3. Emit Swarm State Update
        let _ = app.emit("swarm_run_update", SwarmRunUpdate {
            status: "Idle".to_string(),
            current_node: "Commander".to_string(),
            ledger_snapshot: ledger_content,
        });
    }
}
