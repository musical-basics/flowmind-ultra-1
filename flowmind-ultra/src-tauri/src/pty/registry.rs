use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CliConfig {
    pub name: String,
    pub command: String,
}

pub fn get_default_registry() -> Vec<CliConfig> {
    vec![
        CliConfig { name: "Claude Code".into(), command: "claude-code".into() },
        CliConfig { name: "Aider".into(), command: "aider".into() },
        CliConfig { name: "Gemini CLI".into(), command: "gemini-cli".into() },
        CliConfig { name: "Shell".into(), command: "zsh".into() },
    ]
}
