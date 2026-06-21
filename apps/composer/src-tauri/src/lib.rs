mod commands;
pub mod ui;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    commands::run();
}

pub type BackendState = tench_ui::platform::TauriBackendState;

/// Initialize tench-ui rendering on a Tauri window.
pub fn init_tenchi_ui(app: &mut tauri::App) {
    tench_ui::platform::init_tauri_ui(
        app,
        tench_ui::platform::TauriUiOptions::default(),
        |backend, app| {
            let widget = ui::ComposerApp::new();
            widget.set_app_handle(app.handle().clone());
            backend.set_root(widget);
        },
    );
}
