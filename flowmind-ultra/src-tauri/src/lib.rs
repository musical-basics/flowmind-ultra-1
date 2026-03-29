mod db;

#[tauri::command]
fn log_event(message: String) {
    log::info!("Frontend Telemetry: {}", message);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
        // Initialize DB
        log::info!("Initializing Database...");
        let db_arc = db::store::init_db(app.handle())
            .expect("Failed to initialize database");

        // Repair any lingering processing messages on boot
        let _ = db::outbox::chat_repair_messages(&db_arc);

        // Spawn Detached Tokio Worker Thread (Polling Outbox)
        let db_worker_clone = db_arc.clone();
        tauri::async_runtime::spawn(async move {
            db::outbox::start_worker_loop(db_worker_clone).await;
        });

        app.manage(db::store::DbState { db: db_arc });
        Ok(())
    })
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())
    .invoke_handler(tauri::generate_handler![
        log_event,
        db::commands::workspace_save,
        db::commands::workspace_read,
        db::commands::chat_save_message,
        db::commands::outbox_enqueue
    ])
    .expect("error while running tauri application");
}
