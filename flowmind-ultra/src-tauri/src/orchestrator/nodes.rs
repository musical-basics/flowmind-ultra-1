use std::time::SystemTime;
use crate::llm::client::{LlmClient, ChatRequest, ChatMessage};
use super::schemas::{OverseerOutput, TopologicalGraph, CommanderPlan, WizardCluster};
use crate::llm::sanitizer::sanitize_json;

use crate::llm::flattener::flatten_workspace;

pub fn run_origin(workspace_path: &str, prompt: &str, ignored_dirs: Option<Vec<String>>) -> std::io::Result<String> {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let dir = format!("{}/_swarm_artifacts/run_{}", workspace_path, ts);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(format!("{}/prompt.md", dir), prompt)?;
    
    // Flatten Workspace
    let context = flatten_workspace(workspace_path, ignored_dirs).unwrap_or_else(|e| format!("Failed to read context: {}", e));
    std::fs::write(format!("{}/context.md", dir), context)?;

    Ok(dir)
}

pub async fn run_spec_factory(client: &LlmClient, model: &str, prompt: &str, codebase_context: &str) -> Result<String, String> {
    let req = ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage { role: "system".into(), content: "You are the SpecFactory Node. Write a highly detailed Markdown PRD (Product Requirements Document) outlining exactly how to build what the user asked for. Include a 10-step concrete execution plan.\n\nHere is the current Repository Codebase:\n".to_string() + codebase_context },
            ChatMessage { role: "user".into(), content: prompt.to_string() }
        ],
        response_format: None,
        temperature: Some(0.4),
    };
    let (res, _) = client.complete(req).await?;
    Ok(res)
}

pub async fn run_overseer(client: &LlmClient, model: &str, prd: &str) -> Result<OverseerOutput, String> {
    let req = ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage { role: "system".into(), content: "You are the Overseer Node. Break down the PRD into logical Implementation Sprints. Output STRICT JSON in the shape: {\"sprints\": [{\"id\": 1, \"title\": \"...\", \"description\": \"...\"}]}".into() },
            ChatMessage { role: "user".into(), content: prd.to_string() }
        ],
        response_format: Some(serde_json::json!({ "type": "json_object" })),
        temperature: Some(0.1),
    };
    let (res, _) = client.complete(req).await?;
    let json_clean = sanitize_json(&res);
    serde_json::from_str(&json_clean).map_err(|e| e.to_string())
}

pub async fn run_planner(client: &LlmClient, model: &str, sprint_desc: &str, ledger_context: &str) -> Result<TopologicalGraph, String> {
    let req = ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage { role: "system".into(), content: format!("You are the Planner Node. Map out the files needed for this Sprint into a dependency graph. Output STRICT JSON: {{\n  \"files\": [\n    {{\"filepath\": \"src/main.rs\", \"description\": \"setup\", \"dependencies\": []}}\n  ]\n}}\n\nGlobal Ledger Context:\n{}", ledger_context) },
            ChatMessage { role: "user".into(), content: sprint_desc.to_string() }
        ],
        response_format: Some(serde_json::json!({ "type": "json_object" })),
        temperature: Some(0.1),
    };
    let (res, _) = client.complete(req).await?;
    let json_clean = sanitize_json(&res);
    serde_json::from_str(&json_clean).map_err(|e| e.to_string())
}

pub async fn run_commander(client: &LlmClient, model: &str, graph: &TopologicalGraph) -> CommanderPlan {
    let req = ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage { role: "system".into(), content: "You are the Commander Node. Route the dependency graph into Execution Clusters (wizard_clusters for broad logic, specialist_pairs for specific UI/Backend drops, swarm_files for config files). Output STRICT JSON matching CommanderPlan Schema.".into() },
            ChatMessage { role: "user".into(), content: serde_json::to_string(graph).unwrap() }
        ],
        response_format: Some(serde_json::json!({ "type": "json_object" })),
        temperature: Some(0.1),
    };
    
    let mut all_files = Vec::new();
    for f in &graph.files {
        all_files.push(f.filepath.clone());
    }

    if let Ok((res, _)) = client.complete(req).await {
        let json_clean = sanitize_json(&res);
        if let Ok(plan) = serde_json::from_str::<CommanderPlan>(&json_clean) {
            return plan;
        }
    }

    // Fallback logic
    CommanderPlan {
        wizard_clusters: vec![WizardCluster {
            title: "Fallback Cluster".into(),
            files: all_files,
        }],
        specialist_pairs: vec![],
        swarm_files: vec![],
    }
}
