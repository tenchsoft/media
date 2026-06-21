use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tench_image_core::{
    generate_thumbnail as core_generate_thumbnail,
    list_images_in_archive as core_list_images_in_archive,
    list_images_in_folder as core_list_images_in_folder, open_image as core_open_image,
    open_image_from_archive as core_open_image_from_archive, read_metadata as core_read_metadata,
    ImageDocument, ImageEntry, ImageMetadata, ThumbnailRequest, ThumbnailResult,
};
use tench_image_runtime::view::service as image_service;
use tench_image_runtime::{ImageRuntimeCapabilities, VIEW_RUNTIME};
use tench_storage_core::app_config_file;

use crate::platform_util;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizeParams {
    pub width: u32,
    pub height: u32,
    pub maintain_aspect: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditResult {
    pub path: String,
    pub data_url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiResult {
    pub feature: String,
    pub data_url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewSettings {
    pub theme: String,
    pub show_metadata: bool,
    pub show_thumbnails: bool,
    pub thumbnail_size: u32,
    pub fit_mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSummary {
    pub name: String,
    pub path: String,
    pub extension: String,
    pub size_bytes: u64,
    pub modified_unix: Option<u64>,
    pub is_file: bool,
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            show_metadata: false,
            show_thumbnails: true,
            thumbnail_size: 120,
            fit_mode: "fit".to_string(),
        }
    }
}

struct AppState {
    settings: Mutex<ViewSettings>,
}

#[tauri::command]
fn runtime_capabilities() -> ImageRuntimeCapabilities {
    VIEW_RUNTIME.clone()
}

#[tauri::command]
fn open_image(path: String) -> Result<ImageDocument, String> {
    core_open_image(path).map_err(error_message)
}

#[tauri::command]
fn list_images_in_folder(path: String) -> Result<Vec<ImageEntry>, String> {
    core_list_images_in_folder(path).map_err(error_message)
}

#[tauri::command]
fn list_images_in_archive(path: String) -> Result<Vec<ImageEntry>, String> {
    core_list_images_in_archive(path).map_err(error_message)
}

#[tauri::command]
fn open_image_from_archive(path: String, entry_index: usize) -> Result<ImageDocument, String> {
    core_open_image_from_archive(path, entry_index).map_err(error_message)
}

#[tauri::command]
fn read_metadata(path: String) -> Result<ImageMetadata, String> {
    core_read_metadata(path).map_err(error_message)
}

#[tauri::command]
fn generate_thumbnail(path: String, max_size: u32) -> Result<ThumbnailResult, String> {
    // Check disk cache first
    let cache_key = thumbnail_cache_key(&path, max_size);
    if let Ok(cached) = read_thumbnail_cache(&cache_key) {
        return Ok(ThumbnailResult {
            path: path.clone(),
            width: max_size,
            height: max_size,
            data_url: cached,
        });
    }

    let result = core_generate_thumbnail(&ThumbnailRequest {
        path: path.clone(),
        max_size,
    })
    .map_err(error_message)?;

    // Save to disk cache
    let _ = write_thumbnail_cache(&cache_key, &result.data_url);

    Ok(result)
}

#[tauri::command]
fn get_view_settings(state: tauri::State<'_, AppState>) -> Result<ViewSettings, String> {
    state
        .settings
        .lock()
        .map(|settings| settings.clone())
        .map_err(|_| "Settings lock is poisoned".to_string())
}

#[tauri::command]
fn save_view_settings(
    settings: ViewSettings,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    {
        let mut current = state
            .settings
            .lock()
            .map_err(|_| "Settings lock is poisoned".to_string())?;
        *current = settings.clone();
    }

    save_settings_to_disk(&settings)
}

#[tauri::command]
fn delete_image_file(path: String) -> Result<(), String> {
    fs::remove_file(&path).map_err(|error| format!("Failed to delete {}: {error}", path))
}

#[tauri::command]
pub fn show_in_file_manager(path: String) -> Result<(), String> {
    platform_util::show_in_file_manager(&path)
}

#[tauri::command]
fn rename_file(old_path: String, new_name: String) -> Result<String, String> {
    let path = std::path::Path::new(&old_path);
    if !path.exists() {
        return Err(format!("File not found: {old_path}"));
    }
    let new_path = path
        .parent()
        .map(|p| p.join(&new_name))
        .unwrap_or_else(|| std::path::PathBuf::from(&new_name));
    fs::rename(path, &new_path).map_err(|e| format!("Failed to rename: {e}"))?;
    Ok(new_path.to_string_lossy().to_string())
}

#[tauri::command]
fn copy_file(source: String, destination: String) -> Result<(), String> {
    fs::copy(&source, &destination).map_err(|e| format!("Failed to copy file: {e}"))?;
    Ok(())
}

#[tauri::command]
fn move_file(source: String, destination: String) -> Result<(), String> {
    fs::rename(&source, &destination).map_err(|e| format!("Failed to move file: {e}"))?;
    Ok(())
}

#[tauri::command]
fn get_file_info(path: String) -> Result<FileSummary, String> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }
    let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
    let name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext = p
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    let modified = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    Ok(FileSummary {
        name,
        path: path.clone(),
        extension: ext,
        size_bytes: meta.len(),
        modified_unix: modified,
        is_file: meta.is_file(),
    })
}

#[tauri::command]
fn copy_image_to_clipboard(path: String) -> Result<(), String> {
    let dynamic = image_service::load_image_dynamic(&path)?;
    let rgba = dynamic.to_rgba8();
    let (w, h) = rgba.dimensions();
    platform_util::copy_to_clipboard_image(w as usize, h as usize, rgba.into_raw())
}

#[tauri::command]
fn get_recent_files() -> Result<Vec<String>, String> {
    let path = recent_files_path();
    let Ok(contents) = fs::read_to_string(path) else {
        return Ok(Vec::new());
    };
    let entries: Vec<String> = serde_json::from_str(&contents).unwrap_or_default();
    // Filter to only existing files
    Ok(entries
        .into_iter()
        .filter(|p| fs::metadata(p).is_ok())
        .collect())
}

#[tauri::command]
fn add_recent_file(path: String) -> Result<(), String> {
    let file_path = recent_files_path();
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create recent files directory: {error}"))?;
    }

    let mut entries: Vec<String> = fs::read_to_string(&file_path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default();

    // Remove duplicates
    entries.retain(|p| p != &path);
    entries.insert(0, path);

    // Keep only last 50
    entries.truncate(50);

    let contents = serde_json::to_string_pretty(&entries)
        .map_err(|error| format!("Failed to serialize recent files: {error}"))?;
    fs::write(file_path, contents).map_err(|error| format!("Failed to save recent files: {error}"))
}

#[tauri::command]
fn clear_recent_files() -> Result<(), String> {
    let path = recent_files_path();
    if fs::metadata(&path).is_ok() {
        fs::write(path, "[]").map_err(|error| format!("Failed to clear recent files: {error}"))?;
    }
    Ok(())
}

fn encode_to_data_url(image: &DynamicImage, format: image::ImageFormat) -> Result<String, String> {
    image_service::encode_to_data_url(image, format)
}

fn format_from_name(name: &str) -> image::ImageFormat {
    image_service::format_from_name(name)
}

#[tauri::command]
fn crop_image(path: String, rect: CropRect) -> Result<EditResult, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let (img_w, img_h) = (image.width(), image.height());
    let x = rect.x.min(img_w.saturating_sub(1));
    let y = rect.y.min(img_h.saturating_sub(1));
    let w = rect.width.min(img_w - x);
    let h = rect.height.min(img_h - y);
    let cropped = image_service::crop_image(&image, x, y, w, h)?;
    let format = format_from_name(&path);
    let data_url = encode_to_data_url(&cropped, format)?;
    Ok(EditResult {
        path: path.clone(),
        data_url,
        width: w,
        height: h,
    })
}

#[tauri::command]
fn resize_image(path: String, params: ResizeParams) -> Result<EditResult, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let resized =
        image_service::resize_image(&image, params.width, params.height, params.maintain_aspect)?;
    let (new_w, new_h) = (resized.width(), resized.height());
    let format = format_from_name(&path);
    let data_url = encode_to_data_url(&resized, format)?;
    Ok(EditResult {
        path: path.clone(),
        data_url,
        width: new_w,
        height: new_h,
    })
}

#[tauri::command]
fn rotate_image(path: String, degrees: f64) -> Result<EditResult, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let rotated = image_service::rotate_image(&image, degrees)?;
    let (out_w, out_h) = (rotated.width(), rotated.height());
    let format = format_from_name(&path);
    let data_url = encode_to_data_url(&rotated, format)?;
    Ok(EditResult {
        path: path.clone(),
        data_url,
        width: out_w,
        height: out_h,
    })
}

#[tauri::command]
fn convert_image(path: String, target_format: String) -> Result<EditResult, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let format = format_from_name(&target_format);
    let (w, h) = (image.width(), image.height());
    let data_url = encode_to_data_url(&image, format)?;
    Ok(EditResult {
        path: path.clone(),
        data_url,
        width: w,
        height: h,
    })
}

#[tauri::command]
fn save_edited_image(data_url: String, save_path: String) -> Result<(), String> {
    let bytes = image_service::decode_data_url_bytes(&data_url)?;
    fs::write(&save_path, bytes).map_err(|e| format!("Failed to save image: {e}"))
}

#[tauri::command]
fn get_image_pixel_data(path: String) -> Result<Vec<u8>, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let rgba = image.to_rgba8();
    Ok(rgba.as_raw().clone())
}

// ── AI commands ──
//
// These commands require Tench Engine to be configured and running.
// When Engine is not available, they return a structured error indicating
// the feature requires Engine configuration.

fn engine_not_configured(feature: &str) -> AiResult {
    AiResult {
        feature: feature.to_string(),
        data_url: None,
        width: None,
        height: None,
        tags: None,
        description: None,
        message: format!(
            "{} requires Tench Engine. Configure Engine in Tench One to enable AI features.",
            feature
        ),
    }
}

#[tauri::command]
fn ai_enhance(_path: String) -> Result<AiResult, String> {
    Ok(engine_not_configured("Enhance"))
}

#[tauri::command]
fn ai_upscale(path: String, scale: u32) -> Result<AiResult, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let (w, h) = (image.width() * scale, image.height() * scale);
    let upscaled = image_service::resize_image(&image, w, h, false)?;
    let format = format_from_name(&path);
    let data_url = encode_to_data_url(&upscaled, format)?;
    Ok(AiResult {
        feature: "upscale".to_string(),
        data_url: Some(data_url),
        width: Some(w),
        height: Some(h),
        tags: None,
        description: None,
        message: format!("Upscaled {scale}x with Lanczos3 (AI super-resolution requires Engine)"),
    })
}

#[tauri::command]
fn ai_background_removal(_path: String) -> Result<AiResult, String> {
    Ok(engine_not_configured("Background removal"))
}

#[tauri::command]
fn ai_smart_crop(path: String) -> Result<AiResult, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let (w, h) = (image.width(), image.height());
    let margin_x = w / 10;
    let margin_y = h / 10;
    let cropped = image_service::crop_image(
        &image,
        margin_x,
        margin_y,
        w - 2 * margin_x,
        h - 2 * margin_y,
    )?;
    let format = format_from_name(&path);
    let data_url = encode_to_data_url(&cropped, format)?;
    Ok(AiResult {
        feature: "smart_crop".to_string(),
        data_url: Some(data_url),
        width: Some(cropped.width()),
        height: Some(cropped.height()),
        tags: None,
        description: None,
        message: "Center-cropped to 80% (AI smart crop requires Engine)".to_string(),
    })
}

#[tauri::command]
fn ai_tag_image(_path: String) -> Result<AiResult, String> {
    Ok(engine_not_configured("Image tagging"))
}

#[tauri::command]
fn ai_describe_image(path: String) -> Result<AiResult, String> {
    let image = image_service::load_image_dynamic(&path)?;
    let (w, h) = (image.width(), image.height());
    Ok(AiResult {
        feature: "describe".to_string(),
        data_url: None,
        width: Some(w),
        height: Some(h),
        tags: None,
        description: Some(format!("Image: {w}x{h} pixels")),
        message: "Basic description generated (AI description requires Engine)".to_string(),
    })
}

#[tauri::command]
fn print_image(
    path: String,
    _paper_size: String,
    _orientation: String,
    _scaling: String,
) -> Result<(), String> {
    let image = image_service::load_image_dynamic(&path)?;
    let temp_dir = std::env::temp_dir();
    let print_file = temp_dir.join("tench_view_print.png");
    image
        .save(&print_file)
        .map_err(|e| format!("Failed to prepare print file: {e}"))?;
    platform_util::open_file(&print_file.to_string_lossy())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            settings: Mutex::new(load_settings_from_disk()),
        })
        .invoke_handler(tauri::generate_handler![
            runtime_capabilities,
            open_image,
            list_images_in_folder,
            list_images_in_archive,
            open_image_from_archive,
            read_metadata,
            generate_thumbnail,
            get_view_settings,
            save_view_settings,
            delete_image_file,
            show_in_file_manager,
            rename_file,
            copy_file,
            move_file,
            get_file_info,
            copy_image_to_clipboard,
            get_recent_files,
            add_recent_file,
            clear_recent_files,
            clear_thumbnail_cache,
            crop_image,
            resize_image,
            rotate_image,
            convert_image,
            save_edited_image,
            get_image_pixel_data,
            ai_enhance,
            ai_upscale,
            ai_background_removal,
            ai_smart_crop,
            ai_tag_image,
            ai_describe_image,
            print_image,
        ])
        .setup(|app| {
            // Initialize tench-ui rendering on the main window
            crate::init_tenchi_ui(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run View");
}

fn error_message(error: impl std::fmt::Display) -> String {
    error.to_string()
}

fn load_settings_from_disk() -> ViewSettings {
    let path = settings_path();
    let Ok(contents) = fs::read_to_string(path) else {
        return ViewSettings::default();
    };

    serde_json::from_str(&contents).unwrap_or_else(|_| ViewSettings::default())
}

fn save_settings_to_disk(settings: &ViewSettings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create settings directory: {error}"))?;
    }

    let contents = serde_json::to_string_pretty(settings)
        .map_err(|error| format!("Failed to serialize settings: {error}"))?;
    fs::write(path, contents).map_err(|error| format!("Failed to save settings: {error}"))
}

fn recent_files_path() -> std::path::PathBuf {
    app_config_file("Tench", "View", "recent.json")
}

fn settings_path() -> std::path::PathBuf {
    app_config_file("Tench", "View", "settings.json")
}

fn thumbnail_cache_dir() -> std::path::PathBuf {
    app_config_file("Tench", "View", "thumbnails")
}

fn thumbnail_cache_key(path: &str, max_size: u32) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut hasher);
    max_size.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn read_thumbnail_cache(cache_key: &str) -> Result<String, ()> {
    let dir = thumbnail_cache_dir();
    let path = dir.join(format!("{cache_key}.txt"));
    fs::read_to_string(path).map_err(|_| ())
}

fn write_thumbnail_cache(cache_key: &str, data_url: &str) -> Result<(), String> {
    let dir = thumbnail_cache_dir();
    fs::create_dir_all(&dir)
        .map_err(|error| format!("Failed to create thumbnail cache dir: {error}"))?;
    let path = dir.join(format!("{cache_key}.txt"));
    fs::write(path, data_url).map_err(|error| format!("Failed to write thumbnail cache: {error}"))
}

#[tauri::command]
fn clear_thumbnail_cache() -> Result<(), String> {
    let dir = thumbnail_cache_dir();
    if dir.exists() {
        fs::remove_dir_all(&dir)
            .map_err(|error| format!("Failed to clear thumbnail cache: {error}"))?;
    }
    Ok(())
}
