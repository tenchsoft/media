// ---------------------------------------------------------------------------
// Click action dispatch: edit file
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::ClickAction;
use tench_image_runtime::view::service as image_service;

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_edit_file_action(
        &mut self,
        action: &ClickAction,
        ctx: &mut EventCtx,
    ) -> bool {
        match action {
            ClickAction::DeleteCancel => {
                self.state.show_delete_confirm = false;
                ctx.request_paint();
                true
            }
            ClickAction::DeleteConfirm => {
                if let Some(ref doc) = self.state.document.clone() {
                    let path = doc.path.clone();
                    let _ = image_service::delete_image_file(&path);
                    // Remove from folder entries
                    self.state.folder_entries.retain(|e| e.path != path);
                    self.state.sorted_entries.retain(|e| e.path != path);
                    self.image_cache.remove(&path);
                    self.state.show_delete_confirm = false;
                    // Navigate to next image if available
                    if !self.state.sorted_entries.is_empty() {
                        let next_path = self.state.sorted_entries.first().map(|e| e.path.clone());
                        if let Some(np) = next_path {
                            self.load_image_from_path(&np);
                        }
                    } else {
                        self.state.document = None;
                        self.state.current_image_data = None;
                        self.state.original_image_data = None;
                        self.state.status_message = "Deleted".into();
                    }
                }
                ctx.request_paint();
                true
            }
            ClickAction::EditSave => {
                if let (Some(ref image_data), Some(ref doc)) =
                    (&self.state.current_image_data, &self.state.document)
                {
                    let path = doc.path.clone();
                    let w = image_data.width;
                    let h = image_data.height;
                    let pixels = image_data.data.data().to_vec();
                    std::thread::spawn(move || {
                        let _ = image_service::save_rgba_pixels_to_path(w, h, pixels, &path);
                    });
                    self.state.has_edited_image = false;
                    self.state.original_image_data = Some(image_data.clone());
                    self.state.edit_history.clear();
                    self.state.edit_history_index = 0;
                    self.state.status_message = "Saved".into();
                } else {
                    self.state.status_message = "Nothing to save".into();
                }
                self.update_window_title();
                ctx.request_paint();
                true
            }
            ClickAction::EditDiscard => {
                if let Some(ref original) = self.state.original_image_data {
                    self.state.current_image_data = Some(original.clone());
                    self.state.has_edited_image = false;
                }
                self.update_window_title();
                ctx.request_paint();
                true
            }
            ClickAction::SortByKey => {
                self.state.sort_key = self.state.sort_key.cycle();
                self.state.sort_entries();
                ctx.request_paint();
                true
            }
            ClickAction::ToggleSortOrder => {
                self.state.sort_order = self.state.sort_order.toggle();
                self.state.sort_entries();
                ctx.request_paint();
                true
            }
            ClickAction::OpenBatch => {
                self.state.show_batch = true;
                ctx.request_paint();
                true
            }
            // --- Phase 5: Edit tools real behavior ---

            // Crop Apply: perform actual crop on current image data
            _ => false,
        }
    }
}
