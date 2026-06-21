//! Tauri command handlers and app entry point.
//!
//! Most functionality is now handled directly by the UI paint loop
//! via the GStreamer backend. This module retains the app bootstrap
//! and any remaining Tauri IPC commands.

use crate::platform_util;

/// Show file in file manager.
#[tauri::command]
fn show_in_file_manager(path: String) -> Result<(), String> {
    platform_util::show_in_file_manager(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![show_in_file_manager,])
        .setup(|app| {
            crate::init_tenchi_ui(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Player");
}
