// ---------------------------------------------------------------------------
// Batch image actions
// ---------------------------------------------------------------------------

use tench_image_runtime::view::service as image_service;

use super::ViewApp;

impl ViewApp {
    /// Executes batch resize on selected files.
    pub(super) fn execute_batch_resize(&mut self) {
        let selected: Vec<usize> = self.state.batch_selected.iter().copied().collect();
        if selected.is_empty() {
            return;
        }
        let new_w = self.state.batch_width.max(1);
        let new_h = self.state.batch_height.max(1);
        let total = selected.len();
        self.state.batch_running = true;
        self.state.batch_progress = Some((0, total));

        let paths: Vec<String> = selected
            .iter()
            .filter_map(|&idx| {
                self.state
                    .sorted_entries
                    .get(idx)
                    .map(|entry| entry.path.clone())
            })
            .collect();
        let report = image_service::batch_resize_images(&paths, new_w, new_h);
        for done in 0..report.attempted {
            self.state.batch_progress = Some((done + 1, total));
        }

        self.state.batch_running = false;
        self.state.batch_progress = None;
        self.state.status_message = format!("Batch resize complete: {} files", total);
    }

    /// Executes batch format conversion on selected files.
    pub(super) fn execute_batch_convert(&mut self) {
        let selected: Vec<usize> = self.state.batch_selected.iter().copied().collect();
        if selected.is_empty() {
            return;
        }
        let target_format = self.state.batch_format.clone();
        let total = selected.len();
        self.state.batch_running = true;
        self.state.batch_progress = Some((0, total));

        let paths: Vec<String> = selected
            .iter()
            .filter_map(|&idx| {
                self.state
                    .sorted_entries
                    .get(idx)
                    .map(|entry| entry.path.clone())
            })
            .collect();
        let report = image_service::batch_convert_images(&paths, &target_format);
        for done in 0..report.attempted {
            self.state.batch_progress = Some((done + 1, total));
        }

        self.state.batch_running = false;
        self.state.batch_progress = None;
        self.state.status_message = format!(
            "Batch convert complete: {} files to {}",
            total,
            target_format.to_uppercase()
        );
    }
}
