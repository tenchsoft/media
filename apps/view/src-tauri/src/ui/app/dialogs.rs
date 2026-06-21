// ---------------------------------------------------------------------------
// Native dialogs and recent-file persistence
// ---------------------------------------------------------------------------

use crate::{dialog_sender, DialogResult};
use tench_image_runtime::view::service as image_service;

use super::ViewApp;

impl ViewApp {
    /// Opens a native file dialog to pick an image file.
    pub(super) fn open_file_dialog(&mut self) {
        if self.app_handle.is_none() {
            // Test mode: use injected file path
            if let Some(path) = self.test_next_file.take() {
                self.add_recent_file(&path);
                self.load_image_from_path(&path);
            }
            return;
        }
        use tauri_plugin_dialog::DialogExt;

        let Some(ref handle) = self.app_handle else {
            return;
        };
        let Some(tx) = dialog_sender() else { return };

        let tx = tx.clone();
        handle
            .dialog()
            .file()
            .add_filter(
                "Images",
                &[
                    "png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff", "tif", "ico", "svg", "avif",
                ],
            )
            .add_filter("All Files", &["*"])
            .set_title("Open Image")
            .pick_file(move |path| {
                if let Some(p) = path {
                    let _ = tx.send(DialogResult::File(p.to_string()));
                }
            });
    }

    /// Opens a native folder dialog.
    pub(super) fn open_folder_dialog(&mut self) {
        if self.app_handle.is_none() {
            // Test mode: use injected folder path
            if let Some(path) = self.test_next_folder.take() {
                self.open_folder(&path);
            }
            return;
        }
        use tauri_plugin_dialog::DialogExt;

        let Some(ref handle) = self.app_handle else {
            return;
        };
        let Some(tx) = dialog_sender() else { return };

        let tx = tx.clone();
        handle
            .dialog()
            .file()
            .set_title("Open Folder")
            .pick_folder(move |path| {
                if let Some(p) = path {
                    let _ = tx.send(DialogResult::Folder(p.to_string()));
                }
            });
    }

    /// Opens a native file dialog to pick an archive.
    pub(super) fn open_archive_dialog(&mut self) {
        if self.app_handle.is_none() {
            // Test mode: use injected file path (archives reuse file injection)
            if let Some(path) = self.test_next_file.take() {
                self.open_archive(&path);
            }
            return;
        }
        use tauri_plugin_dialog::DialogExt;

        let Some(ref handle) = self.app_handle else {
            return;
        };
        let Some(tx) = dialog_sender() else { return };

        let tx = tx.clone();
        handle
            .dialog()
            .file()
            .add_filter("Archives", &["zip", "cbz", "7z", "rar", "tar", "gz"])
            .set_title("Open Archive")
            .pick_file(move |path| {
                if let Some(p) = path {
                    let _ = tx.send(DialogResult::File(p.to_string()));
                }
            });
    }

    /// Opens a save-as dialog for converting images.
    pub(super) fn open_save_as_dialog(&mut self, _default_name: &str) {
        if self.app_handle.is_none() {
            // Test mode: use injected save-as path
            if let Some(path) = self.test_save_as_path.take() {
                self.state.convert_output_path = Some(path);
            }
            return;
        }
        use tauri_plugin_dialog::DialogExt;

        let Some(ref handle) = self.app_handle else {
            return;
        };
        let Some(tx) = dialog_sender() else { return };

        let tx = tx.clone();
        handle
            .dialog()
            .file()
            .set_title("Save Converted Image As")
            .set_file_name(_default_name)
            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "tiff"])
            .save_file(move |path| {
                if let Some(p) = path {
                    let _ = tx.send(DialogResult::ConvertOutputPath(p.to_string()));
                }
            });
    }

    /// Opens a native folder dialog to pick batch output folder.
    pub(super) fn open_batch_output_dialog(&mut self) {
        if self.app_handle.is_none() {
            // Test mode: use injected batch output path
            if let Some(path) = self.test_batch_output_path.take() {
                self.state.batch_output_folder = path;
            }
            return;
        }
        use tauri_plugin_dialog::DialogExt;

        let Some(ref handle) = self.app_handle else {
            return;
        };
        let Some(tx) = dialog_sender() else { return };

        let tx = tx.clone();
        handle
            .dialog()
            .file()
            .set_title("Select Batch Output Folder")
            .pick_folder(move |path| {
                if let Some(p) = path {
                    let _ = tx.send(DialogResult::BatchOutputFolder(p.to_string()));
                }
            });
    }

    /// Process any pending dialog results from async dialogs.
    pub(super) fn process_dialog_results(&mut self) {
        let results: Vec<DialogResult> = {
            let Some(ref rx) = self.dialog_rx else { return };
            rx.try_iter().collect()
        };
        for result in results {
            match result {
                DialogResult::File(path) => {
                    // Determine if it's an archive or regular image
                    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
                    if ["zip", "cbz", "7z", "rar", "tar", "gz"].contains(&ext.as_str()) {
                        self.open_archive(&path);
                    } else {
                        self.add_recent_file(&path);
                        self.load_image_from_path(&path);
                    }
                }
                DialogResult::Folder(path) => {
                    self.open_folder(&path);
                }
                DialogResult::ConvertOutputPath(path) => {
                    self.state.convert_output_path = Some(path);
                }
                DialogResult::BatchOutputFolder(path) => {
                    self.state.batch_output_folder = path;
                }
            }
        }
    }

    /// Add a file to the recent files list.
    pub(super) fn add_recent_file(&mut self, path: &str) {
        // Remove duplicate if exists
        self.state.recent_files.retain(|p| p != path);
        // Insert at front
        self.state.recent_files.insert(0, path.to_string());
        // Keep max 20 entries
        self.state.recent_files.truncate(20);
        // Persist to disk
        self.save_recent_files();
    }

    /// Load recent files from disk.
    pub(super) fn load_recent_files(&mut self) {
        let path = self.recent_files_path();
        if let Ok(files) = image_service::load_recent_files(&path) {
            self.state.recent_files = files;
        }
    }

    /// Save recent files to disk.
    fn save_recent_files(&self) {
        let path = self.recent_files_path();
        let _ = image_service::save_recent_files(&path, &self.state.recent_files);
    }

    /// Returns the path for the recent files JSON.
    fn recent_files_path(&self) -> std::path::PathBuf {
        image_service::recent_files_path()
    }
}
