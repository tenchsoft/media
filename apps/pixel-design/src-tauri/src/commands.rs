use tench_image_runtime::pixel;
use tench_image_runtime::{ImageRuntimeCapabilities, PIXEL_DESIGN_RUNTIME};

#[tauri::command]
fn runtime_capabilities() -> ImageRuntimeCapabilities {
    PIXEL_DESIGN_RUNTIME.clone()
}

#[tauri::command]
fn open_image_file(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let path = app.dialog().file().blocking_pick_file();
    match path {
        Some(file_path) => {
            let path_str = file_path.to_string();
            pixel::read_image_file_as_data_url(&path_str).map(Some)
        }
        None => Ok(None),
    }
}

#[tauri::command]
fn save_canvas_image(data_url: String, path: String) -> Result<(), String> {
    pixel::save_data_url_to_path(&data_url, &path)
}

#[tauri::command]
fn save_canvas_as(app: tauri::AppHandle, data_url: String) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let path = app
        .dialog()
        .file()
        .add_filter("Image", &["png", "jpg", "jpeg", "webp", "bmp"])
        .set_file_name("untitled.png")
        .blocking_save_file();
    match path {
        Some(file_path) => {
            let path_str = file_path.to_string();
            save_canvas_image(data_url, path_str.clone())?;
            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

#[tauri::command]
fn get_image_dimensions(data_url: String) -> Result<(u32, u32), String> {
    pixel::data_url_dimensions(&data_url)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            runtime_capabilities,
            open_image_file,
            save_canvas_image,
            save_canvas_as,
            get_image_dimensions,
        ])
        .setup(|app| {
            crate::init_tenchi_ui(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Pixel Design");
}
