use crate::db::store::{DbState, WORKSPACES, CONVERSATIONS, MESSAGES};
use redb::ReadableTable;
use tauri::State;

#[tauri::command]
pub fn workspace_save(db_state: State<'_, DbState>, id: String, config: String) -> Result<(), String> {
    let db = &db_state.db;
    let txn = db.begin_write().map_err(|e| e.to_string())?;
    {
        let mut table = txn.open_table(WORKSPACES).map_err(|e| e.to_string())?;
        table.insert(id.as_str(), config.as_str()).map_err(|e| e.to_string())?;
    }
    txn.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn workspace_read(db_state: State<'_, DbState>, id: String) -> Result<Option<String>, String> {
    let db = &db_state.db;
    let txn = db.begin_read().map_err(|e| e.to_string())?;
    let table = txn.open_table(WORKSPACES).map_err(|e| e.to_string())?;
    if let Ok(Some(val)) = table.get(id.as_str()) {
        Ok(Some(val.value().to_string()))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn chat_save_message(db_state: State<'_, DbState>, id: String, payload: String) -> Result<(), String> {
    let db = &db_state.db;
    let txn = db.begin_write().map_err(|e| e.to_string())?;
    {
        let mut table = txn.open_table(MESSAGES).map_err(|e| e.to_string())?;
        table.insert(id.as_str(), payload.as_str()).map_err(|e| e.to_string())?;
    }
    txn.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn outbox_enqueue(db_state: State<'_, DbState>, payload: String) -> Result<String, String> {
    crate::db::outbox::enqueue_task(&db_state.db, payload)
}
