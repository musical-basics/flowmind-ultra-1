use std::sync::Arc;
use tokio::sync::Mutex;
use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
use std::path::Path;

pub struct WhisperEngine {
    pub ctx: WhisperContext,
}

impl WhisperEngine {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(model_path, params)
            .map_err(|e| format!("Failed to create context: {}", e))?;
        Ok(Self { ctx })
    }

    pub fn transcribe(&self, pcm_data: &[f32]) -> Result<String, String> {
        let mut state = self.ctx.create_state().map_err(|e| e.to_string())?;
        
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_progress(false);
        params.set_print_special(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        state.full(params, pcm_data).map_err(|e| e.to_string())?;

        let num_segments = state.full_n_segments().map_err(|e| e.to_string())?;
        let mut transcript = String::new();
        
        for i in 0..num_segments {
            let segment = state.full_get_segment_text(i).map_err(|e| e.to_string())?;
            transcript.push_str(&segment);
            transcript.push(' ');
        }
        
        // Return raw transcription, ready for optional grammar fix
        Ok(transcript.trim().to_string())
    }
}

pub async fn download_model_if_missing(app: &tauri::AppHandle, cache_dir: &Path, filename: &str, url: &str) -> Result<String, String> {
    let model_path = cache_dir.join(filename);
    if model_path.exists() {
        return Ok(model_path.to_string_lossy().to_string());
    }
    
    log::info!("Downloading Whisper model from {}...", url);
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }
    
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();

    while let Some(item) = futures::StreamExt::next(&mut stream).await {
        let chunk = item.map_err(|e| e.to_string())?;
        buffer.extend_from_slice(&chunk);
        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let progress = (downloaded as f32 / total_size as f32) * 100.0;
            // Emit progress event
            #[derive(serde::Serialize, Clone)]
            struct DownloadProgress { filename: String, progress: f32, total: u64 }
            let _ = tauri::Emitter::emit(app, "whisper_model_progress", DownloadProgress {
                filename: filename.to_string(),
                progress,
                total: total_size,
            });
        }
    }
    
    std::fs::write(&model_path, &buffer).map_err(|e| e.to_string())?;
    log::info!("Model downloaded to {:?}", model_path);
    
    Ok(model_path.to_string_lossy().to_string())
}
