pub struct CompilerSession {
    // Scaffold: Manage headless portable-pty session
}

pub fn detect_compiler_env(workspace_dir: &str) -> Option<String> {
    // Scaffold: Return cargo check, tsc --noEmit, etc.
    None
}

pub async fn run_compiler_check(workspace_dir: &str) -> Result<(), String> {
    // Scaffold: Execute compiler, collect stderr
    Ok(())
}

pub struct CompilerDiagnostics {
    pub file_path: String,
    pub line_number: u32,
    pub error_message: String,
}

pub fn parse_compiler_errors(stderr: &str) -> Vec<CompilerDiagnostics> {
    // Scaffold: Parse errors via regex
    vec![]
}
