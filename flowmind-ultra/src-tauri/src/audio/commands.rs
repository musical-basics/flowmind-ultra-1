use std::sync::{Arc, Mutex, OnceLock};
use super::capture::AudioSystem;
use super::transcribe::{WhisperEngine, download_model_if_missing};
use tauri::AppHandle;

static AUDIO_SYS: OnceLock<Arc<Mutex<AudioSystem>>> = OnceLock::new();
static WHISPER_ENG: OnceLock<Arc<WhisperEngine>> = OnceLock::new();

#[tauri::command]
pub async fn start_voice_dictation(app: AppHandle) -> Result<(), String> {
    let sys = AUDIO_SYS.get_or_init(|| Arc::new(Mutex::new(AudioSystem::new())));
    
    // Asynchronously init whisper engine if not done yet
    if WHISPER_ENG.get().is_none() {
        let cache_dir = app.path().app_cache_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        std::fs::create_dir_all(&cache_dir).unwrap_or_default();
        
        // Use the updated function with app handle
        let model_path = download_model_if_missing(
            &app,
            &cache_dir, 
            "ggml-base.en.bin", 
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin"
        ).await?;
        
        if let Ok(engine) = WhisperEngine::new(&model_path) {
            let _ = WHISPER_ENG.set(Arc::new(engine));
        }
    }

    let mut lock = sys.lock().unwrap();
    lock.start_capture(app)?;
    Ok(())
}

#[tauri::command]
pub async fn stop_and_transcribe_audio() -> Result<String, String> {
    let sys = AUDIO_SYS.get().ok_or("Audio system not initialized")?;
    let pcm_data = sys.lock().unwrap().stop_capture();
    
    let engine = WHISPER_ENG.get().ok_or("Whisper engine not loaded")?;
    let raw_transcript = engine.transcribe(&pcm_data)?;
    
    if raw_transcript.is_empty() {
        return Ok(String::new());
    }

    // Step 146: Grammar Correction Pipeline (API-based)
    let client = crate::llm::client::LlmClient::new(
        std::env::var("OPENROUTER_API_KEY").ok(),
        std::env::var("ANTHROPIC_API_KEY").ok()
    );

    let sys_prompt = "You are a specialized developer assistant. Your task is to take a raw voice dictation and format it into a clean, concise developer prompt or code snippet. Correct grammar, expand technical abbreviations, and maintain professional coding intent. Output ONLY the corrected text.";
    
    let req = crate::llm::client::ChatRequest {
        model: "anthropic/claude-3-5-sonnet-20241022".to_string(), // High precision
        messages: vec![
            crate::llm::client::ChatMessage { role: "system".into(), content: sys_prompt.into() },
            crate::llm::client::ChatMessage { role: "user".into(), content: format!("Raw Dictation: \"{}\"", raw_transcript) }
        ],
        response_format: None,
        temperature: Some(0.1),
    };

    match client.complete(req).await {
        Ok((corrected, _)) => Ok(corrected.trim().replace("\"", "")),
        Err(e) => {
            log::warn!("Grammar correction failed, falling back to raw transcript: {}", e);
            Ok(raw_transcript)
        }
    }
}

#[tauri::command]
pub async fn cancel_voice_dictation() -> Result<(), String> {
    if let Some(sys) = AUDIO_SYS.get() {
        sys.lock().unwrap().stop_capture();
    }
    Ok(())
}
