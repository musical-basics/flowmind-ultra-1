use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::db::supabase::{SupabaseClient, SwarmRunUpdate};
use std::sync::Arc;

pub struct LedgerManager {
    path: PathBuf,
}

impl LedgerManager {
    pub fn new<P: AsRef<Path>>(workspace_dir: P) -> Self {
        Self {
            path: workspace_dir.as_ref().join("global_architecture_ledger.md"),
        }
    }

    pub fn append(&self, content: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(file, "\n{}\n", content)?;
        Ok(())
    }

    pub fn read(&self) -> std::io::Result<String> {
        read_to_string(&self.path)
    }

    pub async fn uplink_to_supabase(&self, sb: &Arc<SupabaseClient>, workspace_id: String) -> Result<(), String> {
        let content = self.read().map_err(|e| e.to_string())?;
        sb.upsert_run(SwarmRunUpdate {
            workspace_id,
            state: "ArchSync".to_string(),
            detail: Some("Mirroring Architectural Ledger to Cloud...".into()),
            is_commander_approved: true,
            is_compiler_approved: true,
            prompt: None,
            ledger_snapshot: Some(content),
        }).await
    }
}
