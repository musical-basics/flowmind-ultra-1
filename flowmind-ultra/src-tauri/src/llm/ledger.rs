use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};

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
}
