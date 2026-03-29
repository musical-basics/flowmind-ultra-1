use tauri::{AppHandle, Emitter};
use crate::pty::session::TerminalSession;
use crate::llm::client::{LlmClient, ChatRequest, ChatMessage};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Serialize, Clone, Debug)]
pub struct WorkerTask {
    pub id: String,
    pub title: String,
    pub files: Vec<String>,
    pub status: String,
    pub cwd: String,
    pub model: String,
}

#[derive(serde::Serialize, Clone)]
pub struct WorkerStatusEvent {
    pub worker_id: String,
    pub task: Option<WorkerTask>,
    pub state: String,
}

pub struct ExecutionWorker {
    pub id: String,
    pub app: AppHandle,
    pub pty_session: Option<Arc<Mutex<TerminalSession>>>,
    pub current_task: Arc<Mutex<Option<WorkerTask>>>,
}

impl ExecutionWorker {
    pub fn new(id: String, app: AppHandle) -> Self {
        Self {
            id,
            app,
            pty_session: None,
            current_task: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn run_task(&mut self, mut task: WorkerTask) -> Result<(), String> {
        task.status = "Sending".to_string();
        
        let mut t = self.current_task.lock().await;
        t.replace(task.clone());
        drop(t);

        let _ = self.app.emit("workers_status", WorkerStatusEvent {
            worker_id: self.id.clone(),
            task: Some(task.clone()),
            state: "Running".to_string(),
        });

        if self.pty_session.is_none() {
            let session = TerminalSession::spawn(format!("worker-{}", self.id), task.cwd.clone(), self.app.clone())?;
            self.pty_session = Some(session);
        }

        if let Some(pty) = &self.pty_session {
            // Clear PTY visually for a fresh task
            let _ = pty.lock().await.write(b"\x1b[2J\x1b[3J\x1b[H");

            // Token limiting simulated delay
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            let client = LlmClient::new(
                std::env::var("OPENROUTER_API_KEY").ok(),
                std::env::var("ANTHROPIC_API_KEY").ok()
            );

            let mut file_context = String::new();
            for file in &task.files {
                let path = format!("{}/{}", task.cwd, file);
                if let Ok(content) = std::fs::read_to_string(&path) {
                    file_context.push_str(&format!("--- {} ---\n{}\n", file, content));
                }
            }

            let sys_prompt = "You are the Agent Worker Node. You must generate a single bash execution script to fulfill the requested task on the provided context files. Generate ONLY a valid bash script that patches, executes, or modifies the files using standard bash commands (cat << 'EOF' > file.txt, sed, et al). Start directly with the commands. Your entire output goes straight into a bash terminal. Do not use code blocks if possible. If you must use markdown codeblocks to wrap your bash, limit it to one block.";
            let user_prompt = format!("Task: {}\nFiles Context:\n{}", task.title, file_context);

            let req = ChatRequest {
                model: task.model.clone(),
                messages: vec![
                    ChatMessage { role: "system".into(), content: sys_prompt.into() },
                    ChatMessage { role: "user".into(), content: user_prompt }
                ],
                response_format: None,
                temperature: Some(0.2),
            };

            let mut script = String::new();
            if let Ok((res, _)) = client.complete(req).await {
                script = res;
            } else {
                script = format!("echo 'Failed to generate code payload for {}'", task.title);
            }

            // Cleanup potential markdown blocks
            let clean_script = script.replace("```bash\n", "").replace("```sh\n", "").replace("```\n", "").replace("```", "");
            
            let cmd = format!("echo 'Executing LLM Payload for {}'\n{}\n", task.title, clean_script);
            pty.lock().await.write(cmd.as_bytes())?;
        }

        // Wait for script execution
        tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;

        // --- QA Loop ---
        if let Some(pty) = &self.pty_session {
            let pty_lock = pty.lock().await;
            let output = pty_lock.output_buffer.lock().await.clone();
            drop(pty_lock);

            let re_port = regex::Regex::new(r"(?i)(eaddrinuse|port .* already in use|address already in use)").unwrap();
            let re_oom = regex::Regex::new(r"(?i)(heap out of memory|oom|out of memory)").unwrap();
            let re_dep = regex::Regex::new(r"(?i)(cannot find module|module not found|no matching package)").unwrap();
            let re_rust = regex::Regex::new(r"(?i)(error\[e[0-9]+\]|build failed)").unwrap();
            let re_general = regex::Regex::new(r"(?i)(error:|command not found|failed|exception|traceback|panic)").unwrap();

            let mut specific_healing_prompt = "";

            if re_port.is_match(&output) {
                specific_healing_prompt = "Port Collision Detected. Generate a bash script that either kills the process occupying the port (using lsof/kill) or configures the service to use an alternate port, then restarts the execution.";
            } else if re_oom.is_match(&output) {
                specific_healing_prompt = "Out of Memory Detected. Generate a script that configures Node max_old_space_size or reduces parallelism flag, then retries the process.";
            } else if re_dep.is_match(&output) {
                specific_healing_prompt = "Missing Dependency Detected. Generate a script that invokes `npm install`, `pnpm add`, or `cargo add` for the exact missing package before continuing.";
            } else if re_rust.is_match(&output) {
                specific_healing_prompt = "Rust Compiler Error Detected. Generate a bash patch (e.g., using sed or cat to rewrite lines) to fix the Rust code syntax/type/lifetime issue.";
            } else if re_general.is_match(&output) {
                specific_healing_prompt = "Execution Error Detected. Evaluate the error traceback and generate a bash script to fix the bug directly.";
            }

            if !specific_healing_prompt.is_empty() {
                let _ = self.app.emit("station_update", crate::orchestrator::loop_runner::StationUpdate {
                    station: "QA".to_string(),
                    status: "Active".to_string(),
                    detail: Some("Errors detected in xterm. Identifying specific error class & Auto-Healing...".to_string()),
                });

                let tail: String = output.lines().rev().take(30).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n");
                
                let sys_prompt = format!("You are the QA Fixer Node. The previous execution failed. {}\n\nStart your fix immediately with bash commands. Avoid code blocks. Output must be raw valid bash.", specific_healing_prompt);
                let user_prompt = format!("Task: {}\nPrevious Terminal Output / Error Tail:\n{}", task.title, tail);

                let req = ChatRequest {
                    model: task.model.clone(),
                    messages: vec![
                        ChatMessage { role: "system".into(), content: sys_prompt.into() },
                        ChatMessage { role: "user".into(), content: user_prompt }
                    ],
                    response_format: None,
                    temperature: Some(0.2),
                };

                let client = LlmClient::new(std::env::var("OPENROUTER_API_KEY").ok(), std::env::var("ANTHROPIC_API_KEY").ok());
                if let Ok((res, _)) = client.complete(req).await {
                    let fix_script = res.replace("```bash\n", "").replace("```sh\n", "").replace("```\n", "").replace("```", "");
                    
                    let pty_lock = pty.lock().await;
                    pty_lock.output_buffer.lock().await.clear();
                    let cmd = format!("echo 'Executing Auto-Heal Fix'\n{}\n", fix_script);
                    pty_lock.write(cmd.as_bytes())?;
                    drop(pty_lock);

                    tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
                    
                    let _ = self.app.emit("station_update", crate::orchestrator::loop_runner::StationUpdate {
                        station: "QA".to_string(),
                        status: "Complete".to_string(),
                        detail: Some("Auto-heal complete".to_string()),
                    });
                }
            } else {
                let _ = self.app.emit("station_update", crate::orchestrator::loop_runner::StationUpdate {
                    station: "QA".to_string(),
                    status: "Complete".to_string(),
                    detail: Some("No errors detected. Passed.".to_string()),
                });
            }
        }
        // --- End QA Loop ---

        let mut t = self.current_task.lock().await;
        *t = None;
        drop(t);

        let _ = self.app.emit("workers_status", WorkerStatusEvent {
            worker_id: self.id.clone(),
            task: None,
            state: "Idle".to_string(),
        });

        Ok(())
    }
}
