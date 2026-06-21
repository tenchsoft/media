use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
pub use tench_fs_core::{is_hidden_path, normalize_extension};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKind {
    Image,
    Video,
    Audio,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MediaEntry {
    pub id: String,
    pub path: String,
    pub file_name: String,
    pub extension: Option<String>,
    pub size_bytes: u64,
    pub modified_at_unix: Option<u64>,
    pub kind: MediaKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MediaScanOptions {
    pub recursive: bool,
    pub allowed_extensions: Vec<String>,
    pub include_hidden: bool,
}

impl MediaScanOptions {
    pub fn images() -> Self {
        Self {
            recursive: false,
            allowed_extensions: image_extensions().into_iter().map(str::to_string).collect(),
            include_hidden: false,
        }
    }

    pub fn videos() -> Self {
        Self {
            recursive: false,
            allowed_extensions: video_extensions().into_iter().map(str::to_string).collect(),
            include_hidden: false,
        }
    }

    pub fn audio() -> Self {
        Self {
            recursive: false,
            allowed_extensions: audio_extensions().into_iter().map(str::to_string).collect(),
            include_hidden: false,
        }
    }

    pub fn all_media() -> Self {
        let mut exts: Vec<String> = Vec::new();
        exts.extend(image_extensions().iter().map(|s| s.to_string()));
        exts.extend(video_extensions().iter().map(|s| s.to_string()));
        exts.extend(audio_extensions().iter().map(|s| s.to_string()));
        Self {
            recursive: false,
            allowed_extensions: exts,
            include_hidden: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MediaError {
    pub code: String,
    pub message: String,
}

impl MediaError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MediaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for MediaError {}

pub fn image_extensions() -> Vec<&'static str> {
    vec![
        "avif", "bmp", "gif", "heic", "heif", "ico", "jpeg", "jpg", "jxl", "png", "pnm", "qoi",
        "svg", "tga", "tif", "tiff", "webp",
    ]
}

pub fn video_extensions() -> Vec<&'static str> {
    vec![
        "mp4", "webm", "mkv", "avi", "mov", "wmv", "flv", "m4v", "mpg", "mpeg", "ogv", "3gp", "ts",
        "mts", "m2ts", "vob", "divx", "asf", "mp4v",
    ]
}

pub fn audio_extensions() -> Vec<&'static str> {
    vec![
        "mp3", "wav", "flac", "aac", "ogg", "opus", "wma", "m4a", "aiff", "aif", "pcm", "wv",
        "ape", "ac3", "dts", "amr", "mid", "midi", "au", "mka",
    ]
}

/// Classify a file extension into the appropriate media kind.
pub fn kind_for_extension(ext: &str) -> MediaKind {
    let normalized = normalize_extension(ext);
    if image_extensions().contains(&normalized.as_str()) {
        MediaKind::Image
    } else if video_extensions().contains(&normalized.as_str()) {
        MediaKind::Video
    } else if audio_extensions().contains(&normalized.as_str()) {
        MediaKind::Audio
    } else {
        MediaKind::Other
    }
}

pub fn scan_folder(
    path: impl AsRef<Path>,
    options: &MediaScanOptions,
) -> Result<Vec<MediaEntry>, MediaError> {
    let path = path.as_ref();
    if !path.is_dir() {
        return Err(MediaError::new(
            "not_a_directory",
            format!("Not a directory: {}", path.display()),
        ));
    }

    let allowed_extensions = options
        .allowed_extensions
        .iter()
        .map(normalize_extension)
        .collect::<HashSet<_>>();

    let mut entries = Vec::new();
    scan_folder_inner(path, options, &allowed_extensions, &mut entries)?;
    entries.sort_by_key(|entry| entry.file_name.to_lowercase());
    Ok(entries)
}

pub fn entry_for_path(path: impl AsRef<Path>, kind: MediaKind) -> Result<MediaEntry, MediaError> {
    let path = path.as_ref();
    let metadata = fs::metadata(path).map_err(|error| {
        MediaError::new(
            "metadata_failed",
            format!("Failed to read metadata for {}: {error}", path.display()),
        )
    })?;

    if !metadata.is_file() {
        return Err(MediaError::new(
            "not_a_file",
            format!("Not a file: {}", path.display()),
        ));
    }

    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_string();
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(normalize_extension);
    let modified_at_unix = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs());
    let path_string = path.display().to_string();
    let id = stable_media_id(&path_string, metadata.len(), modified_at_unix);

    Ok(MediaEntry {
        id,
        path: path_string,
        file_name,
        extension,
        size_bytes: metadata.len(),
        modified_at_unix,
        kind,
    })
}

fn scan_folder_inner(
    path: &Path,
    options: &MediaScanOptions,
    allowed_extensions: &HashSet<String>,
    entries: &mut Vec<MediaEntry>,
) -> Result<(), MediaError> {
    let read_dir = fs::read_dir(path).map_err(|error| {
        MediaError::new(
            "read_directory_failed",
            format!("Failed to read directory {}: {error}", path.display()),
        )
    })?;

    for item in read_dir {
        let item = item.map_err(|error| {
            MediaError::new(
                "read_directory_entry_failed",
                format!(
                    "Failed to read directory entry in {}: {error}",
                    path.display()
                ),
            )
        })?;
        let item_path = item.path();
        if !options.include_hidden && is_hidden_path(&item_path) {
            continue;
        }

        if item_path.is_dir() && options.recursive {
            scan_folder_inner(&item_path, options, allowed_extensions, entries)?;
            continue;
        }

        if !item_path.is_file() {
            continue;
        }

        let extension = item_path
            .extension()
            .and_then(|value| value.to_str())
            .map(normalize_extension);
        let Some(extension) = extension else {
            continue;
        };
        if !allowed_extensions.is_empty() && !allowed_extensions.contains(&extension) {
            continue;
        }

        let kind = kind_for_extension(&extension);
        entries.push(entry_for_path(&item_path, kind)?);
    }

    Ok(())
}

fn stable_media_id(path: &str, size_bytes: u64, modified_at_unix: Option<u64>) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in path.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash ^= size_bytes;
    hash = hash.wrapping_mul(0x100000001b3);
    if let Some(modified_at_unix) = modified_at_unix {
        hash ^= modified_at_unix;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn parent_folder(path: impl AsRef<Path>) -> Option<PathBuf> {
    path.as_ref().parent().map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_extensions() {
        assert_eq!(normalize_extension(".JPG"), "jpg");
    }

    #[test]
    fn detects_hidden_paths() {
        assert!(is_hidden_path(Path::new(".hidden.png")));
        assert!(!is_hidden_path(Path::new("visible.png")));
    }

    #[test]
    fn kind_for_extension_classifies_correctly() {
        assert_eq!(kind_for_extension("jpg"), MediaKind::Image);
        assert_eq!(kind_for_extension(".PNG"), MediaKind::Image);
        assert_eq!(kind_for_extension("mp4"), MediaKind::Video);
        assert_eq!(kind_for_extension(".MKV"), MediaKind::Video);
        assert_eq!(kind_for_extension("mp3"), MediaKind::Audio);
        assert_eq!(kind_for_extension(".FLAC"), MediaKind::Audio);
        assert_eq!(kind_for_extension("xyz"), MediaKind::Other);
    }

    #[test]
    fn scan_options_videos_returns_video_extensions() {
        let opts = MediaScanOptions::videos();
        assert!(opts.allowed_extensions.contains(&"mp4".to_string()));
        assert!(opts.allowed_extensions.contains(&"mkv".to_string()));
        assert!(!opts.allowed_extensions.contains(&"jpg".to_string()));
    }

    #[test]
    fn scan_options_all_media_includes_all_types() {
        let opts = MediaScanOptions::all_media();
        assert!(opts.allowed_extensions.contains(&"jpg".to_string()));
        assert!(opts.allowed_extensions.contains(&"mp4".to_string()));
        assert!(opts.allowed_extensions.contains(&"mp3".to_string()));
    }
}
