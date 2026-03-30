use crate::db::store::{CHAT_OUTBOX_TASKS, DbState};
use redb::{Database, ReadableTable};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use ulid::Ulid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaskStatus {
    Pending,
    Processing,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutboxTask {
    pub id: String,
    pub payload: String, // Stringified JSON
    pub status: TaskStatus,
    pub retries: u32,
    pub next_attempt_at: u64, // Unix timestamp ms
}

pub fn enqueue_task(db: &Arc<Database>, payload: String) -> Result<String, String> {
    let id = Ulid::new().to_string();
    let task = OutboxTask {
        id: id.clone(),
        payload,
        status: TaskStatus::Pending,
        retries: 0,
        next_attempt_at: 0,
    };
    let json = serde_json::to_string(&task).unwrap();
    
    let txn = db.begin_write().map_err(|e| e.to_string())?;
    {
        let mut table = txn.open_table(CHAT_OUTBOX_TASKS).map_err(|e| e.to_string())?;
        table.insert(id.as_str(), json.as_str()).map_err(|e| e.to_string())?;
    }
    txn.commit().map_err(|e| e.to_string())?;
    
    Ok(id)
}

pub fn claim_due_tasks(db: &Arc<Database>) -> Result<Vec<OutboxTask>, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let txn = db.begin_write().map_err(|e| e.to_string())?;
    let mut claimed = Vec::new();

    {
        let mut table = txn.open_table(CHAT_OUTBOX_TASKS).map_err(|e| e.to_string())?;
        let items: Vec<(String, String)> = {
            let mut res = Vec::new();
            if let Ok(iter) = table.range::<&str>(..) {
                for item in iter {
                    if let Ok((k, v)) = item {
                        res.push((k.value().to_string(), v.value().to_string()));
                    }
                }
            }
            res
        };

        for (id, val) in items {
            let mut task: OutboxTask = serde_json::from_str(&val).map_err(|e| e.to_string())?;
            if let TaskStatus::Pending | TaskStatus::Failed = task.status {
                if task.next_attempt_at <= now {
                    task.status = TaskStatus::Processing;
                    let json = serde_json::to_string(&task).unwrap();
                    let _ = table.insert(id.as_str(), json.as_str());
                    claimed.push(task);
                }
            }
        }
    }
    txn.commit().map_err(|e| e.to_string())?;
    Ok(claimed)
}

pub fn mark_sent(db: &Arc<Database>, task_id: &str) -> Result<(), String> {
    let txn = db.begin_write().map_err(|e| e.to_string())?;
    {
        let mut table = txn.open_table(CHAT_OUTBOX_TASKS).map_err(|e| e.to_string())?;
        let _ = table.remove(task_id); // Remove successfully dispatched task
    }
    txn.commit().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn mark_failed(db: &Arc<Database>, task_id: &str) -> Result<(), String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let txn = db.begin_write().map_err(|e| e.to_string())?;
    {
        let mut table = txn.open_table(CHAT_OUTBOX_TASKS).map_err(|e| e.to_string())?;
        
        let val_str = {
            let existing = table.get(task_id).map_err(|e| e.to_string())?;
            if let Some(val) = existing {
                val.value().to_string()
            } else {
                return Ok(());
            }
        }; // Read guard dropped here
        
        let mut task: OutboxTask = serde_json::from_str(&val_str).map_err(|e| e.to_string())?;
        task.retries += 1;
        task.status = TaskStatus::Failed;
        // Exponential backoff
        let delay_ms = 1000 * 2u64.pow(task.retries.min(5));
        task.next_attempt_at = now + delay_ms;
        
        let json = serde_json::to_string(&task).unwrap();
        let _ = table.insert(task_id, json.as_str());
    }
    txn.commit().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn chat_repair_messages(db: &Arc<Database>) -> Result<(), String> {
    let txn = db.begin_write().map_err(|e| e.to_string())?;
    {
        let mut table = txn.open_table(CHAT_OUTBOX_TASKS).map_err(|e| e.to_string())?;
        let items: Vec<(String, String)> = {
            let mut res = Vec::new();
            if let Ok(iter) = table.range::<&str>(..) {
                for item in iter {
                    if let Ok((k, v)) = item {
                        res.push((k.value().to_string(), v.value().to_string()));
                    }
                }
            }
            res
        };

        for (id, val) in items {
            if let Ok(mut task) = serde_json::from_str::<OutboxTask>(&val) {
                if matches!(task.status, TaskStatus::Processing) {
                    task.status = TaskStatus::Pending;
                    let json = serde_json::to_string(&task).unwrap();
                    let _ = table.insert(id.as_str(), json.as_str());
                }
            }
        }
    }
    txn.commit().map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn start_worker_loop(db: Arc<Database>) {
    loop {
        if let Ok(due_tasks) = claim_due_tasks(&db) {
            for task in due_tasks {
                log::info!("Processing Outbox Task: {}", task.id);
                let db_clone = db.clone();
                tokio::spawn(async move {
                    let _ = mark_sent(&db_clone, &task.id);
                });
            }
        }
        sleep(Duration::from_millis(300)).await;
    }
}
