mod commands;
pub mod gst_backend;
#[cfg(target_os = "linux")]
mod mpris;
pub mod platform_util;
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod system_media;
pub mod ui;

use std::sync::{mpsc, OnceLock};

/// Result from a file/folder dialog.
pub enum DialogResult {
    /// User selected a single file path.
    File(String),
    /// User selected a folder path.
    Folder(String),
    /// User selected a subtitle file.
    Subtitle(String),
}

/// Global sender for dialog results.
static DIALOG_TX: OnceLock<mpsc::Sender<DialogResult>> = OnceLock::new();

/// Set the global dialog result sender (called once during init).
pub fn set_dialog_sender(tx: mpsc::Sender<DialogResult>) {
    let _ = DIALOG_TX.set(tx);
}

/// Get the global dialog result sender.
pub fn dialog_sender() -> Option<&'static mpsc::Sender<DialogResult>> {
    DIALOG_TX.get()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    commands::run();
}

pub type BackendState = tench_ui::platform::TauriBackendState;

/// Initialize tench-ui rendering on a Tauri window.
pub fn init_tenchi_ui(app: &mut tauri::App) {
    use tauri::Listener;

    let (dialog_tx, dialog_rx) = mpsc::channel();
    let dialog_tx_clone = dialog_tx.clone();
    set_dialog_sender(dialog_tx);

    let (gst_backend, gst_event_rx) = gst_backend::PlayerBackend::new();

    tench_ui::platform::init_tauri_ui(
        app,
        tench_ui::platform::TauriUiOptions::default(),
        move |backend, app| {
            let mut app_widget = ui::PlayerApp::new();
            app_widget.set_app_handle(app.handle().clone());
            app_widget.set_dialog_receiver(dialog_rx);
            app_widget.set_backend(gst_backend);
            app_widget.set_backend_event_receiver(gst_event_rx);
            backend.set_root(app_widget);
        },
    );

    let drop_tx = dialog_tx_clone.clone();
    app.listen("tauri://drag-drop", move |event| {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct DropPayload {
            paths: Vec<String>,
        }
        if let Ok(payload) = serde_json::from_str::<DropPayload>(event.payload()) {
            if let Some(path) = payload.paths.first() {
                let _ = drop_tx.send(DialogResult::File(path.clone()));
            }
        }
    });

    let file_open_tx = dialog_tx_clone;
    app.listen("tauri://file-open", move |event| {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct FileOpenPayload {
            paths: Vec<String>,
        }
        if let Ok(payload) = serde_json::from_str::<FileOpenPayload>(event.payload()) {
            if let Some(path) = payload.paths.first() {
                let _ = file_open_tx.send(DialogResult::File(path.clone()));
            }
        }
    });
}
