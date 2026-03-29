use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponseUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponseMessage {
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponseChoice {
    pub message: ChatResponseMessage,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<ChatResponseChoice>,
    pub usage: Option<ChatResponseUsage>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LlmCostInterceptor {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub cost_usd: f64,
}

pub struct LlmClient {
    client: Client,
    openrouter_key: Option<String>,
    anthropic_key: Option<String>,
}

impl LlmClient {
    pub fn new(openrouter_key: Option<String>, anthropic_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            openrouter_key,
            anthropic_key,
        }
    }

    pub async fn fetch_models(&self) -> Result<serde_json::Value, String> {
        let res = self
            .client
            .get("https://openrouter.ai/api/v1/models")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        Ok(json)
    }

    pub async fn complete(&self, req: ChatRequest) -> Result<(String, LlmCostInterceptor), String> {
        let key = self.openrouter_key.as_deref().unwrap_or("");

        let endpoint = if req.model.starts_with("anthropic/") && !key.starts_with("sk-or-v1-") {
            // Anthropic direct mock
            "https://api.anthropic.com/v1/messages"
        } else {
            "https://openrouter.ai/api/v1/chat/completions"
        };

        let res = self
            .client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", key))
            .header("HTTP-Referer", "https://flowmind.local")
            .header("X-Title", "Flowmind Ultra")
            .json(&req)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: ChatResponse = res.json().await.map_err(|e| e.to_string())?;

        let content = json
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let mut interceptor = LlmCostInterceptor {
            prompt_tokens: 0,
            completion_tokens: 0,
            cost_usd: 0.0,
        };

        if let Some(u) = json.usage {
            interceptor.prompt_tokens = u.prompt_tokens;
            interceptor.completion_tokens = u.completion_tokens;
            interceptor.cost_usd =
                (u.prompt_tokens as f64 * 0.000001) + (u.completion_tokens as f64 * 0.000002);
        }

        Ok((content, interceptor))
    }
}
