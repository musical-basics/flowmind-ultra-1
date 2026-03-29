use redb::{Database, TableDefinition};
use std::sync::Arc;
use tauri::{AppHandle, Manager};

pub const WORKSPACES: TableDefinition<&str, &str> = TableDefinition::new("workspaces");
pub const CONVERSATIONS: TableDefinition<&str, &str> = TableDefinition::new("conversations");
pub const MESSAGES: TableDefinition<&str, &str> = TableDefinition::new("messages");
pub const CHAT_OUTBOX_TASKS: TableDefinition<&str, &str> = TableDefinition::new("chat_outbox_tasks");

pub struct DbState {
    pub db: Arc<Database>,
}

pub fn init_db(app_handle: &AppHandle) -> Result<Arc<Database>, String> {
    let mut db_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Needs app data dir: {}", e))?;
    
    std::fs::create_dir_all(&db_path).map_err(|e| format!("Failed to create App config dir: {}", e))?;
    
    db_path.push("flowmind_data.redb");
    
    let db = Database::create(&db_path).map_err(|e| format!("Failed to create DB: {}", e))?;
    
    // Initialize tables
    let write_txn = db.begin_write().map_err(|e| e.to_string())?;
    {
        write_txn.open_table(WORKSPACES).map_err(|e| e.to_string())?;
        write_txn.open_table(CONVERSATIONS).map_err(|e| e.to_string())?;
        write_txn.open_table(MESSAGES).map_err(|e| e.to_string())?;
        write_txn.open_table(CHAT_OUTBOX_TASKS).map_err(|e| e.to_string())?;
    }
    write_txn.commit().map_err(|e| e.to_string())?;

    let db_arc = Arc::new(db);
    Ok(db_arc)
}
