// ---------------------------------------------------------------------------
// Click action dispatch: panels
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::ClickAction;

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_panel_action(
        &mut self,
        action: &ClickAction,
        ctx: &mut EventCtx,
    ) -> bool {
        match action {
            ClickAction::ToggleMetadata => {
                self.state.show_metadata = !self.state.show_metadata;
                ctx.request_paint();
                true
            }
            ClickAction::ToggleQuickEdit => {
                self.state.show_quick_edit = !self.state.show_quick_edit;
                ctx.request_paint();
                true
            }
            ClickAction::ToggleFilter => {
                self.state.show_filter = !self.state.show_filter;
                ctx.request_paint();
                true
            }
            ClickAction::ToggleAi => {
                self.state.show_ai = !self.state.show_ai;
                ctx.request_paint();
                true
            }
            ClickAction::ToggleFileInfo => {
                self.state.show_file_info = !self.state.show_file_info;
                ctx.request_paint();
                true
            }
            ClickAction::ToggleBatch => {
                self.state.show_batch = !self.state.show_batch;
                ctx.request_paint();
                true
            }
            ClickAction::ShowInFiles => {
                if let Some(ref doc) = self.state.document.clone() {
                    let path = doc.path.clone();
                    std::thread::spawn(move || {
                        let _ = crate::platform_util::show_in_file_manager(&path);
                    });
                }
                true
            }
            ClickAction::CopyPath => {
                if let Some(ref doc) = self.state.document {
                    let path = doc.path.clone();
                    std::thread::spawn(move || {
                        let _ = crate::platform_util::copy_to_clipboard_text(&path);
                    });
                    self.state.status_message = format!("Copied: {}", doc.path);
                    ctx.request_paint();
                }
                true
            }
            ClickAction::CopyImage => {
                if let Some(ref image_data) = self.state.current_image_data {
                    let w = image_data.width as usize;
                    let h = image_data.height as usize;
                    let pixels = image_data.data.data().to_vec();
                    std::thread::spawn(move || {
                        let _ = crate::platform_util::copy_to_clipboard_image(w, h, pixels);
                    });
                    self.state.status_message = "Copied image to clipboard".into();
                } else {
                    self.state.status_message = "No image to copy".into();
                }
                ctx.request_paint();
                true
            }
            ClickAction::ToggleCompare => {
                if self.state.document.is_some() {
                    self.state.show_compare = !self.state.show_compare;
                    ctx.request_paint();
                }
                true
            }
            ClickAction::ToggleMenu => {
                self.state.show_menu = !self.state.show_menu;
                ctx.request_paint();
                true
            }

            // --- Settings panel ---
            ClickAction::ToggleSettings => {
                self.state.show_settings = !self.state.show_settings;
                ctx.request_paint();
                true
            }
            ClickAction::SettingsTab(tab) => {
                self.state.settings_tab = *tab;
                ctx.request_paint();
                true
            }
            ClickAction::SettingsClose => {
                self.state.show_settings = false;
                ctx.request_paint();
                true
            }

            // --- Canvas advanced ---
            ClickAction::ToggleCheckerboard => {
                self.state.checkerboard_bg = !self.state.checkerboard_bg;
                self.state.status_message = if self.state.checkerboard_bg {
                    "Checkerboard: On".to_string()
                } else {
                    "Checkerboard: Off".to_string()
                };
                ctx.request_paint();
                true
            }

            _ => false,
        }
    }
}
