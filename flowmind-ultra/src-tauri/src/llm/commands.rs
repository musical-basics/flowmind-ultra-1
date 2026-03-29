use crate::llm::client::LlmClient;

#[tauri::command]
pub async fn fetch_models(api_key: Option<String>) -> Result<serde_json::Value, String> {
    let client = LlmClient::new(api_key, None);
    client.fetch_models().await
}

#[tauri::command]
pub fn sanitize_llm_json(raw: String) -> String {
    crate::llm::sanitizer::sanitize_json(&raw)
}
