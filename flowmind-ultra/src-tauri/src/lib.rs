mod db;
mod pty;

use tauri::Manager;

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
        
        let pty_manager = pty::manager::TerminalManager::new();
        app.manage(pty_manager);

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
        db::commands::outbox_enqueue,
        pty::commands::terminal_create,
        pty::commands::terminal_write,
        pty::commands::terminal_resize,
        pty::commands::terminal_close,
        pty::commands::snapshot_ansi,
        pty::commands::snapshot_lines
    ])
    .on_window_event(|window, event| {
        if let tauri::WindowEvent::Destroyed = event {
            let manager = window.state::<pty::manager::TerminalManager>();
            tauri::async_runtime::block_on(async {
                manager.kill_all().await;
            });
        }
    })
    .expect("error while running tauri application");
}
