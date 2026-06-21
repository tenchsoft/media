//! Player media scanning, playlist management, subtitle parsing, and capture export.
//!
//! Uses `tench_media_core` for folder scanning and `tench_subtitle_core`
//! for subtitle parsing.

use std::fs;
use std::path::{Path, PathBuf};

/// A playlist entry representing a media file.
#[derive(Debug, Clone)]
pub struct PlaylistEntry {
    pub id: String,
    pub title: String,
    pub duration: f64,
    pub path: String,
}

/// A recent file entry with a last-opened timestamp.
#[derive(Debug, Clone)]
pub struct RecentEntry {
    pub title: String,
    pub duration: f64,
    pub path: String,
    pub last_opened: u64,
}

/// Scan a folder for video files using tench_media_core.
pub fn scan_video_folder(folder: &str) -> Result<Vec<PlaylistEntry>, String> {
    let dir = Path::new(folder);
    if !dir.is_dir() {
        return Err(format!("Not a directory: {}", folder));
    }

    let options = tench_media_core::MediaScanOptions::videos();
    let entries = tench_media_core::scan_folder(folder, &options).map_err(|e| e.message)?;

    let playlist: Vec<PlaylistEntry> = entries
        .into_iter()
        .map(|e| PlaylistEntry {
            id: e.id,
            title: e.file_name,
            duration: 0.0, // Will be populated when media is loaded
            path: e.path,
        })
        .collect();

    Ok(playlist)
}

/// Guess basic metadata from a file extension (no FFmpeg).
pub fn guess_media_info(path: &str) -> MediaInfoGuess {
    let p = Path::new(path);
    let file_name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let file_size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);

    let ext = p
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let (video_codec, audio_codec) = match ext.as_str() {
        "mp4" | "m4v" => ("H.264", "AAC"),
        "webm" => ("VP9", "Opus"),
        "mkv" => ("H.264", "AAC"),
        "avi" => ("MPEG-4", "MP3"),
        "mov" => ("H.264", "AAC"),
        "wmv" => ("WMV", "WMA"),
        "flv" => ("H.264", "AAC"),
        "mpg" | "mpeg" => ("MPEG-2", "MP2"),
        _ => ("Unknown", "Unknown"),
    };

    MediaInfoGuess {
        file_name,
        file_size,
        video_codec: video_codec.to_string(),
        audio_codec: audio_codec.to_string(),
    }
}

/// Guessed media info from file extension.
pub struct MediaInfoGuess {
    pub file_name: String,
    pub file_size: u64,
    pub video_codec: String,
    pub audio_codec: String,
}

pub fn pictures_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("Pictures")
}

pub fn capture_output_path(prefix: &str, extension: &str) -> PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    pictures_dir().join(format!("{prefix}_{ts}.{extension}"))
}

pub fn save_screenshot_png(pixels: Vec<u8>, width: u32, height: u32) -> Result<String, String> {
    if pixels.len() != (width as usize * height as usize * 4) {
        return Err("Frame data mismatch".to_string());
    }
    let image = image::RgbaImage::from_raw(width, height, pixels)
        .ok_or_else(|| "Failed to create image".to_string())?;
    let dir = pictures_dir();
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let path = capture_output_path("tench_screenshot", "png");
    image.save(&path).map_err(|error| error.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

/// Downsample RGBA pixels to a smaller size using nearest-neighbor sampling.
pub fn downsample_rgba(pixels: &[u8], src_w: u32, src_h: u32, dst_w: u16, dst_h: u16) -> Vec<u8> {
    let dst_w = dst_w as u32;
    let dst_h = dst_h as u32;
    let mut out = Vec::with_capacity((dst_w * dst_h * 4) as usize);
    let x_ratio = src_w as f64 / dst_w as f64;
    let y_ratio = src_h as f64 / dst_h as f64;

    for y in 0..dst_h {
        let src_y = ((y as f64 * y_ratio) as u32).min(src_h - 1);
        for x in 0..dst_w {
            let src_x = ((x as f64 * x_ratio) as u32).min(src_w - 1);
            let idx = ((src_y * src_w + src_x) * 4) as usize;
            if idx + 3 < pixels.len() {
                out.extend_from_slice(&pixels[idx..idx + 4]);
            } else {
                out.extend_from_slice(&[0, 0, 0, 255]);
            }
        }
    }
    out
}

/// Encode RGBA frames into a GIF in the user's Pictures directory.
pub fn encode_gif(frames: &[Vec<u8>], dims: (u16, u16)) -> Result<String, String> {
    let dir = pictures_dir();
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let path = capture_output_path("tench_capture", "gif");
    encode_gif_to_path(frames, dims, &path)?;
    Ok(path.to_string_lossy().to_string())
}

pub fn encode_gif_to_path(frames: &[Vec<u8>], dims: (u16, u16), path: &Path) -> Result<(), String> {
    use gif::Repeat;
    let (w, h) = dims;
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;

    let mut encoder =
        gif::Encoder::new(&mut file, w, h, &[]).map_err(|e| format!("GIF encoder init: {e}"))?;
    encoder
        .set_repeat(Repeat::Infinite)
        .map_err(|e| format!("GIF repeat: {e}"))?;

    for frame_data in frames {
        let mut rgb_data = Vec::with_capacity((w as usize) * (h as usize) * 3);
        for chunk in frame_data.chunks_exact(4) {
            rgb_data.push(chunk[0]);
            rgb_data.push(chunk[1]);
            rgb_data.push(chunk[2]);
        }

        let frame = gif::Frame {
            width: w,
            height: h,
            buffer: std::borrow::Cow::Owned(rgb_data),
            delay: 3,
            ..Default::default()
        };
        encoder
            .write_frame(&frame)
            .map_err(|e| format!("GIF frame write: {e}"))?;
    }

    drop(encoder);
    drop(file);
    Ok(())
}

// Re-export subtitle types and parsing from the shared crate.
pub use tench_subtitle_core::parse_ass;
pub use tench_subtitle_core::parse_srt;
pub use tench_subtitle_core::parse_subtitle_file;
pub use tench_subtitle_core::parse_vtt;
pub use tench_subtitle_core::SubtitleCue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_media_info_mp4() {
        let dir = std::env::temp_dir().join("tench_player_test");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("test.mp4");
        fs::write(&path, b"fake").ok();
        let info = guess_media_info(&path.to_string_lossy());
        assert_eq!(info.video_codec, "H.264");
        assert_eq!(info.audio_codec, "AAC");
        assert_eq!(info.file_name, "test.mp4");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn downsample_rgba_uses_nearest_neighbor_pixels() {
        let pixels = [
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
        ];

        let downsampled = downsample_rgba(&pixels, 2, 2, 1, 1);

        assert_eq!(downsampled, vec![255, 0, 0, 255]);
    }
}
