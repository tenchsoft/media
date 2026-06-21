use std::collections::HashMap;
use std::path::PathBuf;
use tauri::Manager;
use tench_media_runtime::composer::{self, MediaInfo, ProjectData};
use tench_media_runtime::{MediaRuntimeCapabilities, COMPOSER_RUNTIME};

// ── State ──

#[derive(Default)]
#[allow(dead_code)]
pub struct AppState {
    pub project: Option<ProjectData>,
    pub thumbnail_cache: HashMap<String, String>,
    pub recent_projects: Vec<String>,
}

fn project_dir(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("projects")
}

// ── Commands ──

#[tauri::command]
fn runtime_capabilities() -> MediaRuntimeCapabilities {
    COMPOSER_RUNTIME.clone()
}

#[tauri::command]
fn new_project(name: String) -> ProjectData {
    composer::new_project(name)
}

#[tauri::command]
fn save_project(app: tauri::AppHandle, project: ProjectData) -> Result<String, String> {
    composer::save_project(project_dir(&app), &project)
}

#[tauri::command]
fn load_project(app: tauri::AppHandle, name: String) -> Result<ProjectData, String> {
    composer::load_project(project_dir(&app), &name)
}

#[tauri::command]
fn list_projects(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    composer::list_projects(project_dir(&app))
}

#[tauri::command]
fn import_media_files(paths: Vec<String>) -> Result<Vec<MediaInfo>, String> {
    composer::import_media_files(&paths)
}

#[tauri::command]
fn scan_media_folder(folder_path: String) -> Result<Vec<MediaInfo>, String> {
    composer::scan_media_folder(folder_path)
}

#[tauri::command]
fn generate_media_thumbnail(
    _app: tauri::AppHandle,
    path: String,
    max_size: u32,
) -> Result<String, String> {
    composer::generate_media_thumbnail(&path, max_size)
}

#[tauri::command]
fn open_media_dialog(app: tauri::AppHandle) -> Result<Option<Vec<String>>, String> {
    use tauri_plugin_dialog::DialogExt;
    let result = app
        .dialog()
        .file()
        .add_filter(
            "Media",
            &[
                "mp4", "avi", "mov", "mkv", "webm", "mp3", "wav", "flac", "jpg", "png",
            ],
        )
        .blocking_pick_files();
    match result {
        Some(paths) => Ok(Some(paths.iter().map(|p| p.to_string()).collect())),
        None => Ok(None),
    }
}

#[tauri::command]
fn open_folder_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let result = app.dialog().file().blocking_pick_folder();
    match result {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None),
    }
}

#[tauri::command]
fn get_recent_projects(app: tauri::AppHandle) -> Vec<String> {
    composer::list_projects(project_dir(&app)).unwrap_or_default()
}

#[tauri::command]
fn show_in_file_manager(path: String) -> Result<(), String> {
    composer::show_in_file_manager(&path)
}

// ── Entry Point ──

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            runtime_capabilities,
            new_project,
            save_project,
            load_project,
            list_projects,
            import_media_files,
            scan_media_folder,
            generate_media_thumbnail,
            open_media_dialog,
            open_folder_dialog,
            get_recent_projects,
            show_in_file_manager,
        ])
        .setup(|app| {
            crate::init_tenchi_ui(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Composer");
}
