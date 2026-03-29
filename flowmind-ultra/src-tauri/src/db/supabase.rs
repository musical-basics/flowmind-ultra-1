use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct SwarmRunUpdate {
    pub workspace_id: String,
    pub state: String,
    pub detail: Option<String>,
    pub is_commander_approved: bool,
    pub is_compiler_approved: bool,
    pub prompt: Option<String>,
}

pub struct SupabaseClient {
    url: String,
    key: String,
}

impl SupabaseClient {
    pub fn from_env() -> Result<Self, String> {
        // In a real Tauri app, these might be in process env or a config file.
        // We'll look for them in the standard env for now.
        let url = env::var("SUPABASE_URL").map_err(|_| "Missing SUPABASE_URL")?;
        let key = env::var("SUPABASE_SERVICE_ROLE_KEY").map_err(|_| "Missing SUPABASE_SERVICE_ROLE_KEY")?;
        
        Ok(Self { url, key })
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.key).unwrap());
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", self.key)).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        // --- CRITICAL: Target the 'flowmind' schema ---
        headers.insert("Accept-Profile", HeaderValue::from_static("flowmind"));
        headers.insert("Content-Profile", HeaderValue::from_static("flowmind"));
        headers
    }

    pub async fn upsert_run(&self, update: SwarmRunUpdate) -> Result<(), String> {
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&update).map_err(|e| e.to_string())?;
        
        // Upsert based on workspace_id (using PostgREST on_conflict)
        let url = format!("{}/rest/v1/swarm_runs?on_conflict=workspace_id", self.url);
        
        let res = client.post(&url)
            .headers(self.headers())
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(format!("Supabase Error: {} - {}", res.status(), err_text));
        }

        Ok(())
    }

    pub async fn get_run_status(&self, workspace_id: &str) -> Result<Option<SwarmRunUpdate>, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/rest/v1/swarm_runs?workspace_id=eq.{}&select=*", self.url, workspace_id);
        
        let res = client.get(&url)
            .headers(self.headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let items: Vec<SwarmRunUpdate> = res.json().await.map_err(|e| e.to_string())?;
            Ok(items.into_iter().next())
        } else {
            Err(res.text().await.unwrap_or_default())
        }
    }
}
