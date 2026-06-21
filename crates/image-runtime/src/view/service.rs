//! Reusable image service operations.
//!
//! Provides core image manipulation functions that can be used by both
//! the Widget (direct call) and Tauri commands (IPC wrapper).

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use image::{DynamicImage, ImageFormat as ImageCrateFormat, ImageReader};
use serde::{Deserialize, Serialize};
use tench_image_core::{generate_thumbnail as core_generate_thumbnail, ThumbnailRequest};
use tench_ui::peniko::ImageData;

use super::util;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ViewImageDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ViewImageFileMetadata {
    pub file_name: String,
    pub format: String,
    pub dimensions: Option<ViewImageDimensions>,
    pub file_size: u64,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ViewBatchReport {
    pub attempted: usize,
    pub completed: usize,
    pub failed: usize,
}

impl ViewBatchReport {
    fn new(attempted: usize) -> Self {
        Self {
            attempted,
            completed: 0,
            failed: 0,
        }
    }

    fn record(&mut self, result: Result<(), String>) {
        if result.is_ok() {
            self.completed += 1;
        } else {
            self.failed += 1;
        }
    }
}

pub fn image_file_metadata(path: &str) -> ViewImageFileMetadata {
    let file_name = Path::new(path)
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "image".to_string());
    let format = Path::new(path)
        .extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
        .unwrap_or_else(|| "png".to_string());
    let file_size = fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    let dimensions = image::image_dimensions(path)
        .ok()
        .map(|(width, height)| ViewImageDimensions { width, height });

    ViewImageFileMetadata {
        file_name,
        format,
        dimensions,
        file_size,
        path: path.to_string(),
    }
}

pub fn view_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        PathBuf::from(format!("{}/Library/Application Support", home))
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
                PathBuf::from(format!("{}/.config", home))
            })
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        PathBuf::from(".")
    }
}

pub fn recent_files_path() -> PathBuf {
    view_config_dir()
        .join("Tench")
        .join("View")
        .join("recent.json")
}

pub fn load_recent_files(path: &Path) -> Result<Vec<String>, String> {
    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str::<Vec<String>>(&contents).map_err(|error| error.to_string())
}

pub fn save_recent_files(path: &Path, files: &[String]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let contents = serde_json::to_string_pretty(files).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

pub fn delete_image_file(path: &str) -> Result<(), String> {
    fs::remove_file(path).map_err(|error| format!("Failed to delete {path}: {error}"))
}

pub fn rename_image_file(old_path: &str, new_name: &str) -> Result<String, String> {
    let path = Path::new(old_path);
    let new_path = path
        .parent()
        .map(|parent| parent.join(new_name))
        .unwrap_or_else(|| PathBuf::from(new_name));
    fs::rename(path, &new_path).map_err(|error| format!("Rename failed: {error}"))?;
    Ok(new_path.to_string_lossy().to_string())
}

pub fn save_rgba_pixels_to_path(
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    path: &str,
) -> Result<(), String> {
    let rgba = image::RgbaImage::from_raw(width, height, pixels)
        .ok_or_else(|| "Failed to create RGBA image from pixel data".to_string())?;
    let dynamic = DynamicImage::ImageRgba8(rgba);
    let format = Path::new(path)
        .extension()
        .map(|extension| extension.to_string_lossy().to_string())
        .unwrap_or_else(|| "png".to_string());
    save_image(&dynamic, path, format_from_name(&format))
}

pub fn convert_rgba_pixels_to_path(
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    output_path: &str,
    format_name: &str,
) -> Result<(), String> {
    let rgba = image::RgbaImage::from_raw(width, height, pixels)
        .ok_or_else(|| "Failed to create RGBA image from pixel data".to_string())?;
    let dynamic = DynamicImage::ImageRgba8(rgba);
    save_image(&dynamic, output_path, format_from_name(format_name))
}

pub fn batch_resize_images(paths: &[String], width: u32, height: u32) -> ViewBatchReport {
    let mut report = ViewBatchReport::new(paths.len());
    let width = width.max(1);
    let height = height.max(1);

    for (idx, path) in paths.iter().enumerate() {
        let result = resize_image_to_batch_output(path, width, height, idx);
        report.record(result);
    }

    report
}

fn resize_image_to_batch_output(
    path: &str,
    width: u32,
    height: u32,
    fallback_index: usize,
) -> Result<(), String> {
    let dynamic_img = load_image_dynamic(path)?;
    let resized = dynamic_img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let source = Path::new(path);
    let parent = source
        .parent()
        .ok_or_else(|| format!("Missing parent directory for {path}"))?;
    let output_dir = parent.join("tench_batch_resized");
    fs::create_dir_all(&output_dir).map_err(|error| error.to_string())?;
    let file_name = source
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("image_{fallback_index}.png"));
    let output_path = output_dir.join(&file_name);
    let format = Path::new(&file_name)
        .extension()
        .map(|extension| extension.to_string_lossy().to_string())
        .unwrap_or_else(|| "png".to_string());
    save_image(
        &resized,
        &output_path.to_string_lossy(),
        format_from_name(&format),
    )
}

pub fn batch_convert_images(paths: &[String], target_format: &str) -> ViewBatchReport {
    let mut report = ViewBatchReport::new(paths.len());
    let img_format = format_from_name(target_format);

    for path in paths {
        let result = load_image_dynamic(path).and_then(|dynamic_img| {
            let new_path = path
                .rsplit_once('.')
                .map(|(base, _)| format!("{base}.{target_format}"))
                .unwrap_or_else(|| format!("{path}.{target_format}"));
            save_image(&dynamic_img, &new_path, img_format)
        });
        report.record(result);
    }

    report
}

/// Loads and decodes an image from disk, returning both the decoded image
/// and its metadata as an `ImageDocument`.
pub fn load_and_decode(
    path: &str,
) -> Result<(DynamicImage, tench_image_core::ImageDocument), String> {
    let doc = tench_image_core::open_image(path).map_err(|e| e.message)?;
    let dynamic = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {e}"))?
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess format: {e}"))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {e}"))?;
    Ok((dynamic, doc))
}

/// Crops an image to the specified rectangle.
pub fn crop_image(
    img: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<DynamicImage, String> {
    let (img_w, img_h) = (img.width(), img.height());
    let x = x.min(img_w.saturating_sub(1));
    let y = y.min(img_h.saturating_sub(1));
    let w = width.min(img_w - x);
    let h = height.min(img_h - y);
    let mut cloned = img.clone();
    Ok(cloned.crop(x, y, w, h))
}

/// Resizes an image to the specified dimensions.
///
/// When `maintain_aspect` is true, the image is scaled to fit within
/// the given dimensions while preserving the aspect ratio.
pub fn resize_image(
    img: &DynamicImage,
    width: u32,
    height: u32,
    maintain_aspect: bool,
) -> Result<DynamicImage, String> {
    let (new_w, new_h) = if maintain_aspect {
        let (orig_w, orig_h) = (img.width() as f64, img.height() as f64);
        let ratio = orig_w / orig_h;
        if width > 0 && height > 0 {
            let scale = (width as f64 / orig_w).min(height as f64 / orig_h);
            ((orig_w * scale) as u32, (orig_h * scale) as u32)
        } else if width > 0 {
            (width, (width as f64 / ratio) as u32)
        } else if height > 0 {
            ((height as f64 * ratio) as u32, height)
        } else {
            (img.width(), img.height())
        }
    } else {
        (width.max(1), height.max(1))
    };
    Ok(img.resize_exact(new_w, new_h, image::imageops::FilterType::Lanczos3))
}

/// Rotates an image by the given degrees.
///
/// Supports 90-degree increments for lossless rotation.
/// Arbitrary angles use a bounding-box resize approximation.
pub fn rotate_image(img: &DynamicImage, degrees: f64) -> Result<DynamicImage, String> {
    let rotated = match degrees as u32 % 360 {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => {
            let rad = degrees.to_radians();
            let cos_a = rad.cos().abs();
            let sin_a = rad.sin().abs();
            let (w, h) = (img.width() as f64, img.height() as f64);
            let new_w = (w * cos_a + h * sin_a).ceil() as u32;
            let new_h = (w * sin_a + h * cos_a).ceil() as u32;
            img.resize_exact(new_w, new_h, image::imageops::FilterType::Lanczos3)
        }
    };
    Ok(rotated)
}

/// Converts an image to the specified format, returning the encoded bytes.
pub fn convert_image(img: &DynamicImage, format: ImageCrateFormat) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|e| format!("Failed to encode image: {e}"))?;
    Ok(buf)
}

/// Saves an image to disk in the specified format.
pub fn save_image(img: &DynamicImage, path: &str, format: ImageCrateFormat) -> Result<(), String> {
    img.save_with_format(path, format)
        .map_err(|e| format!("Failed to save image: {e}"))?;
    Ok(())
}

/// Generates a thumbnail `ImageData` suitable for GPU rendering.
///
/// Decodes the image, creates a thumbnail at `max_size`, and converts
/// to `peniko::ImageData` for direct use in the rendering pipeline.
pub fn generate_thumbnail(path: &str, max_size: u32) -> Result<ImageData, String> {
    let reader = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {e}"))?
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess format: {e}"))?;
    let dynamic = reader
        .decode()
        .map_err(|e| format!("Failed to decode image: {e}"))?;
    let thumb = dynamic.thumbnail(max_size, max_size);
    Ok(util::dynamic_to_image_data(&thumb))
}

/// Generates a thumbnail data URL via tench_image_core.
pub fn generate_thumbnail_data_url(path: &str, max_size: u32) -> Result<String, String> {
    let result = core_generate_thumbnail(&ThumbnailRequest {
        path: path.to_string(),
        max_size,
    })
    .map_err(|e| e.message)?;
    Ok(result.data_url)
}

// --- Helper functions used by commands.rs ---

/// Loads a `DynamicImage` from a file path.
pub fn load_image_dynamic(path: &str) -> Result<DynamicImage, String> {
    ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {e}"))?
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess format: {e}"))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {e}"))
}

/// Encodes a `DynamicImage` to a base64 data URL.
pub fn encode_to_data_url(
    image: &DynamicImage,
    format: ImageCrateFormat,
) -> Result<String, String> {
    let mut buf = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|e| format!("Failed to encode image: {e}"))?;
    let mime = match format {
        ImageCrateFormat::Png => "image/png",
        ImageCrateFormat::Jpeg => "image/jpeg",
        ImageCrateFormat::WebP => "image/webp",
        ImageCrateFormat::Bmp => "image/bmp",
        ImageCrateFormat::Tiff => "image/tiff",
        _ => "image/png",
    };
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(buf)))
}

/// Decodes a base64 data URL into raw image bytes.
pub fn decode_data_url_bytes(data_url: &str) -> Result<Vec<u8>, String> {
    let encoded = data_url
        .strip_prefix("data:")
        .and_then(|s| s.split_once(';'))
        .and_then(|(_, rest)| rest.strip_prefix("base64,"))
        .ok_or_else(|| "Invalid data URL format".to_string())?;
    STANDARD
        .decode(encoded)
        .map_err(|e| format!("Failed to decode base64: {e}"))
}

/// Determines the image format from a file extension string.
pub fn format_from_name(name: &str) -> ImageCrateFormat {
    match name.to_ascii_lowercase().as_str() {
        "png" => ImageCrateFormat::Png,
        "jpg" | "jpeg" => ImageCrateFormat::Jpeg,
        "webp" => ImageCrateFormat::WebP,
        "bmp" => ImageCrateFormat::Bmp,
        "tiff" | "tif" => ImageCrateFormat::Tiff,
        _ => ImageCrateFormat::Png,
    }
}

/// Encodes RGBA pixel data to a DynamicImage and then to PNG bytes.
pub fn encode_rgba_as_png(width: u32, height: u32, pixels: &[u8]) -> Result<Vec<u8>, String> {
    let rgba = image::RgbaImage::from_raw(width, height, pixels.to_vec())
        .ok_or_else(|| "Failed to create RGBA image from pixel data".to_string())?;
    let dynamic = DynamicImage::ImageRgba8(rgba);
    convert_image(&dynamic, ImageCrateFormat::Png)
}
