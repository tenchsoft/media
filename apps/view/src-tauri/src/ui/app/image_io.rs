// ---------------------------------------------------------------------------
// Image loading, navigation, thumbnails, and URL loading
// ---------------------------------------------------------------------------

use std::io::Read;

use crate::ui::state;
use tauri::Manager;
use tench_image_runtime::view::service as image_service;
use tench_image_runtime::view::util;
use tench_image_runtime::view::util::{dynamic_to_image_data, HistogramData};

use super::ViewApp;

impl ViewApp {
    /// Updates the native window title to reflect the current document state.
    pub fn update_window_title(&self) {
        if let Some(ref handle) = self.app_handle {
            if let Some(wvw) = handle.get_webview_window("main") {
                let title = if let Some(ref doc) = self.state.document {
                    let prefix = if self.state.has_edited_image { "*" } else { "" };
                    format!("{}{} \u{2014} View", prefix, doc.file_name)
                } else {
                    "View".to_string()
                };
                let _ = wvw.set_title(&title);
            }
        }
    }

    /// Loads an image file directly via tench_image_core (no Tauri IPC).
    pub fn load_image_from_path(&mut self, path: &str) {
        let metadata = image_service::image_file_metadata(path);

        self.state.load_image(state::ImageMetadata {
            file_name: metadata.file_name,
            format: metadata.format,
            dimensions: metadata
                .dimensions
                .map(|dimensions| state::ImageDimensions {
                    width: dimensions.width,
                    height: dimensions.height,
                }),
            file_size: metadata.file_size,
            path: metadata.path,
        });

        // Decode the image using image_service and store pixel data
        if let Ok(dynamic_img) = image_service::load_image_dynamic(path) {
            // Compute histogram
            self.state.histogram = Some(HistogramData::from_dynamic_image(&dynamic_img));

            // Convert to peniko ImageData
            let image_data = dynamic_to_image_data(&dynamic_img);
            self.state.set_image_data(image_data.clone());

            // Update dimensions from decoded image
            if let Some(ref mut doc) = self.state.document {
                doc.dimensions = Some(state::ImageDimensions {
                    width: dynamic_img.width(),
                    height: dynamic_img.height(),
                });
            }

            // Cache the image
            self.image_cache.insert(path.to_string(), image_data);
        }

        // Load EXIF metadata
        self.load_exif_data(path);

        // Update window title
        self.update_window_title();
    }

    /// Loads EXIF metadata from tench_image_core.
    pub(super) fn load_exif_data(&mut self, path: &str) {
        match tench_image_core::read_metadata(path) {
            Ok(meta) => {
                self.state.exif_tags = meta
                    .tags
                    .into_iter()
                    .map(|t| state::MetadataTag {
                        group: t.group,
                        name: t.name,
                        value: t.value,
                    })
                    .collect();
            }
            Err(_) => {
                self.state.exif_tags = Vec::new();
            }
        }
    }

    /// Generates thumbnails for filmstrip entries that are visible and not yet cached.
    /// `visible_range` is the (start_index, end_index) of entries visible in the filmstrip.
    pub(super) fn generate_visible_thumbnails(&mut self, visible_start: usize, visible_end: usize) {
        let thumb_size = 64u32; // Decode at slightly higher res than display for quality
        for i in visible_start..visible_end {
            if let Some(entry) = self.state.sorted_entries.get(i) {
                let path = &entry.path;
                if self.state.thumbnail_cache.contains_key(path) {
                    continue;
                }
                // Skip archive entries that don't have real file paths on disk
                if entry.is_archive_entry {
                    continue;
                }
                // Try to decode a small thumbnail
                if let Ok(dynamic_img) = image_service::load_image_dynamic(path) {
                    let thumb = dynamic_img.thumbnail(thumb_size, thumb_size);
                    let image_data = util::dynamic_to_image_data(&thumb);
                    self.state.thumbnail_cache.insert(path.clone(), image_data);
                }
            }
        }
    }

    /// Opens a folder and lists images in it.
    pub fn open_folder(&mut self, path: &str) {
        match tench_image_core::list_images_in_folder(path) {
            Ok(entries) => {
                let folder_entries: Vec<state::FolderEntry> = entries
                    .into_iter()
                    .map(|e| state::FolderEntry {
                        id: e.media.id,
                        path: e.media.path,
                        file_name: e.media.file_name,
                        size_bytes: e.media.size_bytes,
                        modified_unix: None,
                        is_archive_entry: false,
                    })
                    .collect();

                if let Some(first) = folder_entries.first() {
                    let first_path = first.path.clone();
                    self.state.set_folder_entries(folder_entries);
                    self.load_image_from_path(&first_path);
                } else {
                    self.state.status_message = "No images found in folder".into();
                }
            }
            Err(e) => {
                self.state.status_message = format!("Failed to open folder: {}", e.message);
            }
        }
    }

    /// Opens an archive and lists images in it.
    pub fn open_archive(&mut self, path: &str) {
        match tench_image_core::list_images_in_archive(path) {
            Ok(entries) => {
                let folder_entries: Vec<state::FolderEntry> = entries
                    .into_iter()
                    .map(|e| state::FolderEntry {
                        id: e.media.id,
                        path: e.media.path,
                        file_name: e.media.file_name,
                        size_bytes: e.media.size_bytes,
                        modified_unix: None,
                        is_archive_entry: true,
                    })
                    .collect();

                if let Some(first) = folder_entries.first() {
                    let first_path = first.path.clone();
                    self.state.set_folder_entries(folder_entries);
                    self.load_image_from_path(&first_path);
                } else {
                    self.state.status_message = "No images found in archive".into();
                }
            }
            Err(e) => {
                self.state.status_message = format!("Failed to open archive: {}", e.message);
            }
        }
    }

    /// Navigate and load the next/prev image from cache or disk.
    pub(super) fn navigate_and_load(&mut self, forward: bool) {
        let next_path = if forward {
            let idx = self.state.selected_index();
            let Some(idx) = idx else { return };
            self.state
                .sorted_entries
                .get(idx + 1)
                .map(|e| e.path.clone())
        } else {
            let idx = self.state.selected_index();
            let Some(idx) = idx else { return };
            if idx == 0 {
                return;
            }
            self.state
                .sorted_entries
                .get(idx - 1)
                .map(|e| e.path.clone())
        };

        if let Some(path) = next_path {
            // Navigate state first
            if forward {
                self.state.navigate_next();
            } else {
                self.state.navigate_prev();
            }

            // Check cache
            if let Some(cached) = self.image_cache.get(&path).cloned() {
                self.state.set_image_data(cached);
                self.load_exif_data(&path);
            } else {
                // Load from disk
                self.load_image_from_path(&path);
            }
        }
    }

    /// Prefetch adjacent images in the background (nomacs updateCacher pattern).
    pub(super) fn prefetch_adjacent(&mut self) {
        let Some(idx) = self.state.selected_index() else {
            return;
        };
        let prefetch_range = 3;

        for delta in -(prefetch_range as isize)..=(prefetch_range as isize) {
            if delta == 0 {
                continue;
            }
            let target = (idx as isize + delta) as usize;
            if let Some(entry) = self.state.sorted_entries.get(target) {
                let path = &entry.path;
                if !self.image_cache.contains(path) {
                    // Try to decode and cache
                    if let Ok(dynamic_img) = image_service::load_image_dynamic(path) {
                        let image_data = dynamic_to_image_data(&dynamic_img);
                        self.image_cache.insert(path.clone(), image_data);
                    }
                }
            }
        }
    }

    /// Loads an image from a URL.
    pub fn load_image_from_url(&mut self, url: &str) {
        match ureq::get(url).call() {
            Ok(response) => {
                let mut bytes = Vec::new();
                if response.into_reader().read_to_end(&mut bytes).is_ok() {
                    if let Ok(dynamic_img) = image::load_from_memory(&bytes) {
                        let image_data = dynamic_to_image_data(&dynamic_img);
                        self.state.histogram =
                            Some(HistogramData::from_dynamic_image(&dynamic_img));
                        self.state.document = Some(state::ImageMetadata {
                            file_name: url.split('/').next_back().unwrap_or("image").to_string(),
                            format: "png".to_string(),
                            dimensions: Some(state::ImageDimensions {
                                width: dynamic_img.width(),
                                height: dynamic_img.height(),
                            }),
                            file_size: bytes.len() as u64,
                            path: url.to_string(),
                        });
                        self.state.reset_for_new_image();
                        self.state.set_image_data(image_data);
                        self.state.status_message = "Loaded from URL".into();
                    } else {
                        self.state.status_message = "Failed to decode image from URL".into();
                    }
                } else {
                    self.state.status_message = "Failed to read response body".into();
                }
            }
            Err(e) => {
                self.state.status_message = format!("Failed to fetch URL: {}", e);
            }
        }
        self.state.status_message_time = Some(std::time::Instant::now());
    }
}
