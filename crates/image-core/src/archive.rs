use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::time::UNIX_EPOCH;

use tench_media_core::{image_extensions, MediaEntry, MediaKind};

use crate::{detect_format, ImageCoreError, ImageEntry};

pub(crate) enum ArchiveKind {
    Zip,
    Rar,
    SevenZip,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ArchiveImageInfo {
    pub(crate) entry_index: usize,
    pub(crate) name: String,
    pub(crate) source_name: String,
    pub(crate) size_bytes: u64,
    pub(crate) encrypted: bool,
}

fn is_supported_archive_image(name: &str) -> bool {
    let Some(extension) = Path::new(name)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
    else {
        return false;
    };

    image_extensions().contains(&extension.as_str())
}

pub(crate) fn list_archive_image_infos(
    path: &Path,
) -> Result<Vec<ArchiveImageInfo>, ImageCoreError> {
    match archive_kind(path)? {
        ArchiveKind::Zip => list_zip_images(path),
        ArchiveKind::Rar => list_rar_images(path),
        ArchiveKind::SevenZip => list_seven_zip_images(path),
    }
}

pub(crate) fn read_archive_entry_bytes(
    path: &Path,
    image_info: &ArchiveImageInfo,
) -> Result<Vec<u8>, ImageCoreError> {
    match archive_kind(path)? {
        ArchiveKind::Zip => read_zip_entry_bytes(path, image_info),
        ArchiveKind::Rar => read_rar_entry_bytes(path, image_info),
        ArchiveKind::SevenZip => read_seven_zip_entry_bytes(path, image_info),
    }
}

fn archive_kind(path: &Path) -> Result<ArchiveKind, ImageCoreError> {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .as_deref()
    {
        Some("zip") => Ok(ArchiveKind::Zip),
        Some("rar") => Ok(ArchiveKind::Rar),
        Some("7z") => Ok(ArchiveKind::SevenZip),
        _ => Err(ImageCoreError::new(
            "unsupported_archive_format",
            "Supported archive formats are ZIP, RAR, and 7z.",
        )),
    }
}

fn list_zip_images(path: &Path) -> Result<Vec<ArchiveImageInfo>, ImageCoreError> {
    let file = File::open(path).map_err(|error| {
        ImageCoreError::new(
            "archive_open_failed",
            format!("Failed to open ZIP archive {}: {error}", path.display()),
        )
    })?;
    let mut archive = zip::ZipArchive::new(file).map_err(|error| {
        ImageCoreError::new(
            "archive_list_failed",
            format!("Failed to read ZIP archive {}: {error}", path.display()),
        )
    })?;
    let mut images = Vec::new();

    for entry_index in 0..archive.len() {
        let file = archive.by_index(entry_index).map_err(|error| {
            ImageCoreError::new(
                "archive_list_failed",
                format!("Failed to inspect ZIP entry {entry_index}: {error}"),
            )
        })?;
        let Some(name) = normalized_archive_name(file.name()) else {
            continue;
        };
        if !file.is_file() || !is_supported_archive_image(&name) {
            continue;
        }
        images.push(ArchiveImageInfo {
            entry_index,
            name,
            source_name: file.name().to_string(),
            size_bytes: file.size(),
            encrypted: file.encrypted(),
        });
    }

    Ok(images)
}

fn read_zip_entry_bytes(
    path: &Path,
    image_info: &ArchiveImageInfo,
) -> Result<Vec<u8>, ImageCoreError> {
    let file = File::open(path).map_err(|error| {
        ImageCoreError::new(
            "archive_open_failed",
            format!("Failed to open ZIP archive {}: {error}", path.display()),
        )
    })?;
    let mut archive = zip::ZipArchive::new(file).map_err(|error| {
        ImageCoreError::new(
            "archive_extract_failed",
            format!("Failed to read ZIP archive {}: {error}", path.display()),
        )
    })?;
    let mut file = archive.by_index(image_info.entry_index).map_err(|error| {
        ImageCoreError::new(
            "archive_entry_not_found",
            format!(
                "Failed to open ZIP entry {}: {error}",
                image_info.entry_index
            ),
        )
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|error| {
        ImageCoreError::new(
            "archive_extract_failed",
            format!("Failed to extract {}: {error}", image_info.name),
        )
    })?;
    Ok(bytes)
}

fn list_rar_images(path: &Path) -> Result<Vec<ArchiveImageInfo>, ImageCoreError> {
    #[cfg(not(feature = "rar-archives"))]
    {
        let _ = path;
        Err(rar_archives_disabled_error())
    }

    #[cfg(feature = "rar-archives")]
    {
        let archive = unrar_ng::Archive::new(path)
            .open_for_listing()
            .map_err(|error| {
                ImageCoreError::new(
                    "archive_list_failed",
                    format!("Failed to read RAR archive {}: {error}", path.display()),
                )
            })?;
        let mut images = Vec::new();

        for (entry_index, header) in archive.enumerate() {
            let header = header.map_err(|error| {
                ImageCoreError::new(
                    "archive_list_failed",
                    format!("Failed to inspect RAR entry {entry_index}: {error}"),
                )
            })?;
            let source_name = header.filename.to_string_lossy().to_string();
            let Some(name) = normalized_archive_name(&source_name) else {
                continue;
            };
            if !header.is_file() || !is_supported_archive_image(&name) {
                continue;
            }
            images.push(ArchiveImageInfo {
                entry_index,
                name,
                source_name,
                size_bytes: header.unpacked_size,
                encrypted: header.is_encrypted(),
            });
        }

        Ok(images)
    }
}

fn read_rar_entry_bytes(
    path: &Path,
    image_info: &ArchiveImageInfo,
) -> Result<Vec<u8>, ImageCoreError> {
    #[cfg(not(feature = "rar-archives"))]
    {
        let _ = (path, image_info);
        Err(rar_archives_disabled_error())
    }

    #[cfg(feature = "rar-archives")]
    {
        let mut archive = unrar_ng::Archive::new(path)
            .open_for_processing()
            .map_err(|error| {
                ImageCoreError::new(
                    "archive_extract_failed",
                    format!("Failed to open RAR archive {}: {error}", path.display()),
                )
            })?;
        let mut entry_index = 0usize;

        while let Some(header) = archive.read_header().map_err(|error| {
            ImageCoreError::new(
                "archive_extract_failed",
                format!("Failed to read RAR header: {error}"),
            )
        })? {
            if entry_index == image_info.entry_index {
                if header.entry().is_encrypted() {
                    return Err(ImageCoreError::new(
                        "archive_password_required",
                        format!("{} is encrypted and needs a password.", image_info.name),
                    ));
                }
                let (bytes, _) = header.read().map_err(|error| {
                    ImageCoreError::new(
                        "archive_extract_failed",
                        format!("Failed to extract {}: {error}", image_info.name),
                    )
                })?;
                return Ok(bytes);
            }

            archive = header.skip().map_err(|error| {
                ImageCoreError::new(
                    "archive_extract_failed",
                    format!("Failed to skip RAR entry {entry_index}: {error}"),
                )
            })?;
            entry_index += 1;
        }

        Err(ImageCoreError::new(
            "archive_entry_not_found",
            format!(
                "Archive entry {} was not found in {}",
                image_info.entry_index,
                path.display()
            ),
        ))
    }
}

#[cfg(not(feature = "rar-archives"))]
fn rar_archives_disabled_error() -> ImageCoreError {
    ImageCoreError::new(
        "unsupported_archive_format",
        "RAR archive support is not enabled in this build.",
    )
}

fn list_seven_zip_images(path: &Path) -> Result<Vec<ArchiveImageInfo>, ImageCoreError> {
    let archive = sevenz_rust2::Archive::open(path).map_err(|error| {
        ImageCoreError::new(
            "archive_list_failed",
            format!("Failed to read 7z archive {}: {error}", path.display()),
        )
    })?;
    let mut images = Vec::new();

    for (entry_index, entry) in archive.files.iter().enumerate() {
        let Some(name) = normalized_archive_name(&entry.name) else {
            continue;
        };
        if entry.is_directory || !entry.has_stream || !is_supported_archive_image(&name) {
            continue;
        }
        images.push(ArchiveImageInfo {
            entry_index,
            name,
            source_name: entry.name.clone(),
            size_bytes: entry.size,
            encrypted: false,
        });
    }

    Ok(images)
}

fn read_seven_zip_entry_bytes(
    path: &Path,
    image_info: &ArchiveImageInfo,
) -> Result<Vec<u8>, ImageCoreError> {
    let mut archive = sevenz_rust2::ArchiveReader::open(path, sevenz_rust2::Password::empty())
        .map_err(|error| {
            ImageCoreError::new(
                "archive_extract_failed",
                format!("Failed to open 7z archive {}: {error}", path.display()),
            )
        })?;
    archive.read_file(&image_info.source_name).map_err(|error| {
        ImageCoreError::new(
            "archive_extract_failed",
            format!("Failed to extract {}: {error}", image_info.name),
        )
    })
}

pub(crate) fn archive_entry_from_info(
    archive_label: &str,
    archive_id: &str,
    image_info: &ArchiveImageInfo,
) -> ImageEntry {
    let media_path = format!("archive://{archive_id}/{}", image_info.entry_index);
    let media = MediaEntry {
        id: archive_entry_id(archive_id, image_info),
        path: media_path,
        file_name: format!("{archive_label}/{}", image_info.name),
        extension: archive_entry_extension(&image_info.name),
        size_bytes: image_info.size_bytes,
        modified_at_unix: None,
        kind: MediaKind::Image,
    };

    ImageEntry {
        media,
        format: detect_format(&image_info.name),
        dimensions: None,
    }
}

pub(crate) fn archive_entry_id(archive_id: &str, image_info: &ArchiveImageInfo) -> String {
    let mut hasher = DefaultHasher::new();
    archive_id.hash(&mut hasher);
    image_info.entry_index.hash(&mut hasher);
    image_info.name.hash(&mut hasher);
    image_info.size_bytes.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub(crate) fn archive_entry_extension(entry_name: &str) -> Option<String> {
    Path::new(entry_name)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
}

fn normalized_archive_name(name: &str) -> Option<String> {
    let normalized = name.replace('\\', "/");
    let mut safe_parts = Vec::new();

    for component in Path::new(&normalized).components() {
        match component {
            Component::Normal(part) => {
                let part = part.to_str()?;
                if !part.is_empty() {
                    safe_parts.push(part.to_string());
                }
            }
            Component::CurDir => {}
            _ => return None,
        }
    }

    if safe_parts.is_empty() {
        None
    } else {
        Some(safe_parts.join("/"))
    }
}

pub(crate) fn archive_cache_dir(path: &Path) -> Result<PathBuf, ImageCoreError> {
    Ok(std::env::temp_dir()
        .join("tench-view-archives")
        .join(archive_cache_key(path)?))
}

pub(crate) fn archive_cache_key(path: &Path) -> Result<String, ImageCoreError> {
    let metadata = fs::metadata(path).map_err(|error| {
        ImageCoreError::new(
            "archive_metadata_failed",
            format!(
                "Failed to read archive metadata for {}: {error}",
                path.display()
            ),
        )
    })?;
    let modified_at_unix = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs());
    let canonical_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let mut hasher = DefaultHasher::new();
    canonical_path.display().to_string().hash(&mut hasher);
    metadata.len().hash(&mut hasher);
    modified_at_unix.hash(&mut hasher);

    Ok(format!("{:016x}", hasher.finish()))
}

pub(crate) fn safe_archive_file_name(index: usize, entry_name: &str) -> String {
    let path = Path::new(entry_name);
    let raw_stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("image");
    let mut safe_stem: String = raw_stem
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if safe_stem.is_empty() {
        safe_stem = "image".to_string();
    }

    match path.extension().and_then(|value| value.to_str()) {
        Some(extension) => format!("{index:04}-{safe_stem}.{}", extension.to_ascii_lowercase()),
        None => format!("{index:04}-{safe_stem}"),
    }
}
