use std::fs;
use crate::llm::client::{LlmClient, ChatRequest, ChatMessage};

pub async fn generate_patch(failing_file: &str, error_msg: &str, workspace_dir: &str) -> Result<String, String> {
    let full_path = format!("{}/{}", workspace_dir, failing_file);
    let original_code = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;

    let sys_prompt = r#"You are the Swarm Auto-Healing Sniper Node.
Your sole job is to fix syntax and compiler errors provided in the context.
You must analyze the existing code and the compiler error map.
OUTPUT STRICTLY A VALID JSON RESPONSE. NO MARKDOWN WRAPPERS!
Format: {"code": "new_full_file_contents_here"}
Do not return anything else."#;

    let user_prompt = format!("Code File:\n```\n{}\n```\n\nCompiler Errors:\n{}\n\nFix the code and output the JSON patch.", original_code, error_msg);

    let client = LlmClient::new(
        std::env::var("OPENROUTER_API_KEY").ok(),
        std::env::var("ANTHROPIC_API_KEY").ok()
    );

    let req = ChatRequest {
        model: "anthropic/claude-3-5-sonnet-20241022".to_string(), // Default executor profile
        messages: vec![
            ChatMessage { role: "system".into(), content: sys_prompt.into() },
            ChatMessage { role: "user".into(), content: user_prompt }
        ],
        response_format: None,
        temperature: Some(0.1),
    };

    let (res, _) = client.complete(req).await.map_err(|e| e.to_string())?;
    
    // Quick sanitization
    let clean = crate::llm::sanitizer::sanitize_json(&res);
    Ok(clean)
}
