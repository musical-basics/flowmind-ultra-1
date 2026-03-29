use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Notify;
use super::schemas::{CommanderPlan, TopologicalGraph};

pub struct SwarmOrchestratorState {
    pub commander_approval: Arc<Notify>,
}

impl SwarmOrchestratorState {
    pub fn new() -> Self {
        Self {
            commander_approval: Arc::new(Notify::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SwarmState {
    Idle,
    Origin,
    SpecFactory,
    Overseer,
    Planner,
    Commander,
    Executor,
    QaReviewer,
    Complete,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintChunk {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub dependency_graph: Option<TopologicalGraph>,
    pub execution_plan: Option<CommanderPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmContext {
    pub workspace_dir: String,
    pub artifact_dir: Option<String>,
    pub original_prompt: String,
    pub prd_markdown: Option<String>,
    pub sprint_chunks: Vec<SprintChunk>,
    pub current_chunk_idx: usize,
    pub active_state: SwarmState,
}

impl SwarmContext {
    pub fn new(workspace_dir: String, prompt: String) -> Self {
        Self {
            workspace_dir,
            artifact_dir: None,
            original_prompt: prompt,
            prd_markdown: None,
            sprint_chunks: vec![],
            current_chunk_idx: 0,
            active_state: SwarmState::Idle,
        }
    }
}
