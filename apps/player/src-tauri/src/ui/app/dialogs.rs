use super::PlayerApp;
use crate::{dialog_sender, DialogResult};

impl PlayerApp {
    pub(crate) fn open_file_dialog(&mut self) {
        #[cfg(test)]
        {
            if !self.test_next_files.is_empty() {
                let files = std::mem::take(&mut self.test_next_files);
                for path in files {
                    self.state.playlist.push(crate::ui::state::PlaylistEntry {
                        title: path
                            .split(['/', '\\'])
                            .next_back()
                            .unwrap_or("file")
                            .to_string(),
                        duration: 0.0,
                        path: path.clone(),
                    });
                }
                self.state.show_toast("Files added to playlist");
                return;
            }
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
                "Video",
                &[
                    "mp4", "webm", "mkv", "avi", "mov", "wmv", "flv", "m4v", "mpg", "mpeg", "ogv",
                    "3gp",
                ],
            )
            .set_title("Open Media")
            .pick_file(move |path| {
                if let Some(p) = path {
                    let _ = tx.send(DialogResult::File(p.to_string()));
                }
            });
    }

    /// Open a native folder dialog.
    pub(crate) fn open_folder_dialog(&self) {
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

    /// Open a native subtitle file dialog.
    pub(crate) fn open_subtitle_dialog(&self) {
        use tauri_plugin_dialog::DialogExt;

        let Some(ref handle) = self.app_handle else {
            return;
        };
        let Some(tx) = dialog_sender() else { return };

        let tx = tx.clone();
        handle
            .dialog()
            .file()
            .add_filter("Subtitle", &["srt", "vtt", "ass", "ssa", "sub"])
            .set_title("Open Subtitle")
            .pick_file(move |path| {
                if let Some(p) = path {
                    let _ = tx.send(DialogResult::Subtitle(p.to_string()));
                }
            });
    }

    /// Process any pending dialog results from async dialogs.
    pub(crate) fn process_dialog_results(&mut self) {
        let results: Vec<DialogResult> = {
            let Some(ref rx) = self.dialog_rx else { return };
            rx.try_iter().collect()
        };
        for result in results {
            match result {
                DialogResult::File(path) => {
                    self.load_and_play(&path);
                }
                DialogResult::Folder(path) => {
                    self.state.open_folder(&path);
                    // Auto-play first file via backend
                    if let Some(first) = self.state.playlist.first().cloned() {
                        self.load_and_play(&first.path);
                    }
                }
                DialogResult::Subtitle(path) => {
                    self.state.load_subtitles(&path);
                }
            }
        }
    }
}
