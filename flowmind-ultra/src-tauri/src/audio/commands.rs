#[tauri::command]
pub async fn start_voice_dictation() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn stop_and_transcribe_audio() -> Result<String, String> {
    Ok(String::new())
}

#[tauri::command]
pub async fn cancel_voice_dictation() -> Result<(), String> {
    Ok(())
}
