mod commands;
pub mod platform_util;
pub mod ui;

use std::sync::{mpsc, OnceLock};

/// Result from a file/folder dialog.
pub enum DialogResult {
    /// User selected a single file path.
    File(String),
    /// User selected a folder path.
    Folder(String),
    /// User selected a save path for image conversion.
    ConvertOutputPath(String),
    /// User selected a batch output folder.
    BatchOutputFolder(String),
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
///
/// This function is called from `commands::run()` during Tauri setup.
/// It creates product-specific channels and delegates rendering setup to tench-ui.
pub fn init_tenchi_ui(app: &mut tauri::App) {
    use tauri::Listener;
    use tauri::Manager;

    let (dialog_tx, dialog_rx) = mpsc::channel();
    set_dialog_sender(dialog_tx);

    tench_ui::platform::init_tauri_ui(
        app,
        tench_ui::platform::TauriUiOptions::default(),
        move |backend, app| {
            let mut app_widget = ui::ViewApp::new();
            app_widget.set_app_handle(app.handle().clone());
            app_widget.set_dialog_receiver(dialog_rx);
            app_widget.load_persisted_state();
            backend.set_root(app_widget);
        },
    );

    let app_handle = app.handle().clone();
    let window_label = "main".to_string();

    let ah = app_handle.clone();
    if let Some(wvw) = app.get_webview_window(&window_label) {
        wvw.on_window_event(move |event| {
            let Some(state) = ah.try_state::<BackendState>() else {
                return;
            };
            state.with_backend(|backend| match event {
                tauri::WindowEvent::Resized(size) => {
                    backend.resize(size.width, size.height);
                    backend.render();
                }
                tauri::WindowEvent::Focused(focused) => {
                    backend.on_window_event(tench_ui::core::events::WindowEvent::Focused(*focused));
                }
                tauri::WindowEvent::Destroyed => {
                    backend.on_window_event(tench_ui::core::events::WindowEvent::Destroyed);
                }
                _ => {}
            });
        });
    }

    let ah = app_handle.clone();
    app.listen("tauri://drag-drop", move |event| {
        if let Some(paths) = parse_dropped_paths(event.payload()) {
            if let Some(tx) = dialog_sender() {
                for path in paths {
                    let _ = tx.send(if std::path::Path::new(&path).is_dir() {
                        DialogResult::Folder(path)
                    } else {
                        DialogResult::File(path)
                    });
                }
            }
            let Some(state) = ah.try_state::<BackendState>() else {
                return;
            };
            state.with_backend(tench_ui::platform::TauriBackend::render);
        }
    });
}

/// Parse dropped file paths from a Tauri drag-drop event payload.
fn parse_dropped_paths(payload: &str) -> Option<Vec<String>> {
    // Tauri drag-drop payload is JSON: {"paths":["/path/to/file",...],"position":{"x":...,"y":...}}
    let v: serde_json::Value = serde_json::from_str(payload).ok()?;
    let paths = v.get("paths")?.as_array()?;
    Some(
        paths
            .iter()
            .filter_map(|p| p.as_str().map(|s| s.to_string()))
            .collect(),
    )
}
