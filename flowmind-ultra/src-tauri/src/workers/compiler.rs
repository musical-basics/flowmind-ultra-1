use std::path::Path;
use std::process::{Command, Stdio};

pub struct CompilerSession {
    // Scaffold: Manage headless portable-pty session (if needed for PTY compatibility, but process::Command is safer for headless compilation buffering)
}

pub fn detect_compiler_env(workspace_dir: &str) -> Option<String> {
    let p = Path::new(workspace_dir);
    if p.join("Cargo.toml").exists() {
        return Some("cargo check".to_string());
    }
    if p.join("tsconfig.json").exists() {
        return Some("npx tsc --noEmit".to_string());
    }
    if p.join("requirements.txt").exists() {
        return Some("flake8".to_string());
    }
    None
}

pub async fn run_compiler_check(workspace_dir: &str, app: tauri::AppHandle) -> Result<(), String> {
    let cmd_str = detect_compiler_env(workspace_dir).ok_or("No supported compiler found")?;
    let mut parts = cmd_str.split_whitespace();
    let program = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    let output = Command::new(program)
        .args(args)
        .current_dir(workspace_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run compiler: {}", e))?;

    let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

    let combined = format!("{}\n{}", stdout_str, stderr_str);

    // 46. Live Telemetry
    #[derive(serde::Serialize, Clone)]
    struct CompilerStream { data: String }
    let _ = tauri::Emitter::emit(&app, "compiler_diagnostics_stream", CompilerStream { data: combined.clone() });

    if output.status.success() {
        Ok(())
    } else {
        Err(combined)
    }
}

pub struct CompilerDiagnostics {
    pub file_path: String,
    pub line_number: u32,
    pub error_message: String,
}

pub fn parse_compiler_errors(stderr: &str) -> Vec<CompilerDiagnostics> {
    let mut diags = Vec::new();
    let re_rust = regex::Regex::new(r"(?m)-->\s*([^:]+):(\d+):\d+").unwrap();
    let re_ts = regex::Regex::new(r"(?m)^([^:]+)\((\d+),\d+\):\s*error").unwrap();

    for cap in re_rust.captures_iter(stderr).chain(re_ts.captures_iter(stderr)) {
        if let (Some(f), Some(l)) = (cap.get(1), cap.get(2)) {
            diags.push(CompilerDiagnostics {
                file_path: f.as_str().trim().to_string(),
                line_number: l.as_str().parse::<u32>().unwrap_or(0),
                error_message: stderr.to_string()
            });
            break; // Grab the first critical error for sniper
        }
    }
    diags
}
