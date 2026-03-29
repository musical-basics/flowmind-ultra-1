use std::path::Path;
use std::fs;
use std::sync::Arc;
use arrow_array::{RecordBatch, StringArray, FixedSizeListArray, Float32Array};
use arrow_schema::{Schema, Field, DataType};
use lancedb::Table;
use crate::db::vector::VectorDB;
use crate::llm::embeddings::EmbeddingEngine;

pub fn chunk_text(content: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;
    
    while start < content.len() {
        let end = (start + chunk_size).min(content.len());
        chunks.push(content[start..end].to_string());
        if end == content.len() { break; }
        start += chunk_size - overlap;
    }
    chunks
}

pub async fn index_file(
    file_path: &str,
    content: &str,
    engine: &EmbeddingEngine,
    table: &Table,
) -> Result<(), String> {
    let chunks = chunk_text(content, 1000, 100);
    
    let mut ids = Vec::new();
    let mut paths = Vec::new();
    let mut texts = Vec::new();
    let mut vectors = Vec::new();

    for (i, chunk) in chunks.into_iter().enumerate() {
        let vector = engine.generate(&chunk)?;
        ids.push(format!("{}-{}", file_path, i));
        paths.push(file_path.to_string());
        texts.push(chunk);
        vectors.push(Some(vector));
    }

    if ids.is_empty() { return Ok(()); }

    let schema = table.schema().await.map_err(|e| e.to_string())?;
    
    // Convert Vec<Option<Vec<f32>>> to FixedSizeListArray
    let flat_vectors: Vec<f32> = vectors.into_iter().flatten().flatten().collect();
    let vector_array = Float32Array::from(flat_vectors);
    let list_array = FixedSizeListArray::try_new_from_values(vector_array, 384).map_err(|e| e.to_string())?;

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(ids)),
            Arc::new(StringArray::from(paths)),
            Arc::new(StringArray::from(texts)),
            Arc::new(list_array),
        ],
    ).map_err(|e| e.to_string())?;

    table.add(Box::new(vec![Ok(batch)].into_iter())).execute().await.map_err(|e| e.to_string())
}

pub async fn index_workspace(
    workspace_dir: &str,
    engine: &EmbeddingEngine,
    db: &VectorDB,
) -> Result<(), String> {
    let table = db.get_or_create_table("swarm_memory", 384).await?;
    
    // Use walkdir or manual recursion (ignoring .git, node_modules etc.)
    // For now, simplify and process a few key files if they exist
    let files_to_index = vec!["global_architecture_ledger.md", "flowmind-ultra-prd2.md"];
    
    for f in files_to_index {
        let full_path = Path::new(workspace_dir).join(f);
        if full_path.exists() {
            let content = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
            index_file(f, &content, engine, &table).await?;
        }
    }

    Ok(())
}
