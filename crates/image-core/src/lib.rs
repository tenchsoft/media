use std::fs;
use std::path::Path;

use tench_media_core::{
    entry_for_path, parent_folder, scan_folder, MediaEntry, MediaKind, MediaScanOptions,
};

mod archive;
pub mod metadata;
pub mod types;

use metadata::image_dimensions;

pub use metadata::{detect_format, generate_thumbnail, read_metadata};
pub use types::*;

use archive::{
    archive_cache_dir, archive_cache_key, archive_entry_extension, archive_entry_from_info,
    archive_entry_id, list_archive_image_infos, read_archive_entry_bytes, safe_archive_file_name,
};

pub fn open_image(path: impl AsRef<Path>) -> Result<ImageDocument, ImageCoreError> {
    let path = path.as_ref();
    let entry = image_entry_for_path(path, true)?;
    let metadata = read_metadata(path)?;
    let folder = parent_folder(path).map(|folder| folder.display().to_string());

    Ok(ImageDocument {
        entry,
        metadata,
        folder,
    })
}

pub fn list_images_in_folder(path: impl AsRef<Path>) -> Result<Vec<ImageEntry>, ImageCoreError> {
    let entries = scan_folder(path, &MediaScanOptions::images())?;
    entries
        .into_iter()
        .map(|media| image_entry_from_media(media, false))
        .collect()
}

pub fn list_images_in_archive(path: impl AsRef<Path>) -> Result<Vec<ImageEntry>, ImageCoreError> {
    let path = path.as_ref();
    let mut image_infos = list_archive_image_infos(path)?;
    image_infos.sort_by_key(|info| info.name.to_lowercase());
    let archive_label = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("archive")
        .to_string();
    let archive_id = archive_cache_key(path)?;

    Ok(image_infos
        .into_iter()
        .map(|info| archive_entry_from_info(&archive_label, &archive_id, &info))
        .collect())
}

pub fn open_image_from_archive(
    path: impl AsRef<Path>,
    entry_index: usize,
) -> Result<ImageDocument, ImageCoreError> {
    let path = path.as_ref();
    let image_info = list_archive_image_infos(path)?
        .into_iter()
        .find(|info| info.entry_index == entry_index)
        .ok_or_else(|| {
            ImageCoreError::new(
                "archive_entry_not_found",
                format!(
                    "Archive entry {entry_index} was not found in {}",
                    path.display()
                ),
            )
        })?;

    if image_info.encrypted {
        return Err(ImageCoreError::new(
            "archive_password_required",
            format!("{} is encrypted and needs a password.", image_info.name),
        ));
    }

    let bytes = read_archive_entry_bytes(path, &image_info)?;
    let cache_dir = archive_cache_dir(path)?;
    fs::create_dir_all(&cache_dir).map_err(|error| {
        ImageCoreError::new(
            "archive_cache_create_failed",
            format!(
                "Failed to create archive cache {}: {error}",
                cache_dir.display()
            ),
        )
    })?;

    let extracted_path = cache_dir.join(safe_archive_file_name(
        image_info.entry_index,
        &image_info.name,
    ));
    fs::write(&extracted_path, bytes).map_err(|error| {
        ImageCoreError::new(
            "archive_cache_write_failed",
            format!(
                "Failed to write extracted image {}: {error}",
                extracted_path.display()
            ),
        )
    })?;

    let archive_label = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("archive")
        .to_string();
    let archive_id = archive_cache_key(path)?;
    let display_name = format!("{archive_label}/{}", image_info.name);
    let mut entry = image_entry_for_path(&extracted_path, true)?;
    entry.media.id = archive_entry_id(&archive_id, &image_info);
    entry.media.file_name = display_name.clone();
    entry.media.extension = archive_entry_extension(&image_info.name);
    entry.media.size_bytes = image_info.size_bytes;
    entry.format = detect_format(&image_info.name);

    let mut metadata = read_metadata(&extracted_path)?;
    metadata.file_name = display_name;
    metadata.format = entry.format.clone();
    metadata.tags.push(MetadataTag {
        group: "Archive".to_string(),
        name: "Container".to_string(),
        value: path.display().to_string(),
    });
    metadata.tags.push(MetadataTag {
        group: "Archive".to_string(),
        name: "Entry".to_string(),
        value: image_info.name,
    });

    Ok(ImageDocument {
        entry,
        metadata,
        folder: Some(path.display().to_string()),
    })
}

fn image_entry_for_path(
    path: &Path,
    include_dimensions: bool,
) -> Result<ImageEntry, ImageCoreError> {
    let media = entry_for_path(path, MediaKind::Image)?;
    image_entry_from_media(media, include_dimensions)
}

fn image_entry_from_media(
    media: MediaEntry,
    include_dimensions: bool,
) -> Result<ImageEntry, ImageCoreError> {
    let format = detect_format(&media.path);
    let dimensions = if include_dimensions {
        image_dimensions(&media.path).ok()
    } else {
        None
    };

    Ok(ImageEntry {
        media,
        format,
        dimensions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_format_from_extension() {
        assert_eq!(detect_format("sample.JPG"), ImageFormat::Jpeg);
        assert_eq!(detect_format("sample.webp"), ImageFormat::Webp);
        assert_eq!(detect_format("sample.unknown"), ImageFormat::Unknown);
    }

    #[test]
    fn clamps_thumbnail_size() {
        let request = ThumbnailRequest {
            path: "x.png".to_string(),
            max_size: 1,
        };
        assert_eq!(request.max_size, 1);
    }

    #[cfg(not(feature = "rar-archives"))]
    #[test]
    fn rar_archives_return_unsupported_when_feature_is_disabled() {
        let error = list_images_in_archive("sample.rar").expect_err("RAR should be feature-gated");

        assert_eq!(error.code, "unsupported_archive_format");
        assert!(error.message.contains("RAR"));
    }
}
