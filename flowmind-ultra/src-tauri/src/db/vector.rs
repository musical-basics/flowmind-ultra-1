use std::sync::Arc;
use lancedb::connection::Connection;
use lancedb::Table;
use arrow_array::{RecordBatch, RecordBatchIterator};
use arrow_schema::{DataType, Field, Schema};
use std::path::PathBuf;

pub struct VectorDB {
    pub conn: Arc<dyn Connection>,
}

impl VectorDB {
    pub async fn init(workspace_dir: &str) -> Result<Self, String> {
        let db_path = PathBuf::from(workspace_dir).join(".golutra").join("vectors");
        std::fs::create_dir_all(&db_path).map_err(|e| e.to_string())?;

        let uri = format!("file://{}", db_path.to_string_lossy());
        let conn = lancedb::connect(&uri).execute().await.map_err(|e| e.to_string())?;

        Ok(Self { conn: Arc::new(conn) })
    }

    pub async fn get_or_create_table(&self, table_name: &str, dim: usize) -> Result<Table, String> {
        let table_names = self.conn.table_names().execute().await.map_err(|e| e.to_string())?;
        
        if table_names.contains(&table_name.to_string()) {
            return self.conn.open_table(table_name).execute().await.map_err(|e| e.to_string());
        }

        let schema = Arc::new(Schema::new(vec![
            Field::new("chunk_id", DataType::Utf8, false),
            Field::new("file_path", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new("vector", DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), dim as i32), false),
        ]));

        // Create an empty batch to initialize the table
        let batches = RecordBatchIterator::new(vec![Ok(RecordBatch::new_empty(schema.clone()))], schema);
        
        self.conn.create_table(table_name, Box::new(batches)).execute().await.map_err(|e| e.to_string())
    }

    pub async fn clear_table(&self, table_name: &str) -> Result<(), String> {
        self.conn.drop_table(table_name).await.map_err(|e| e.to_string())
    }
}
