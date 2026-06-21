// ---------------------------------------------------------------------------
// Click action dispatch: batch dialogs
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::{self, ClickAction};
use tench_image_runtime::view::service as image_service;

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_batch_dialog_action(
        &mut self,
        action: &ClickAction,
        ctx: &mut EventCtx,
    ) -> bool {
        match action {
            ClickAction::BatchToggleFile(idx) => {
                if self.state.batch_selected.contains(idx) {
                    self.state.batch_selected.remove(idx);
                } else {
                    self.state.batch_selected.insert(*idx);
                }
                ctx.request_paint();
                true
            }

            // Batch: Select all / deselect all
            ClickAction::BatchToggleSelectAll => {
                let total = self.state.sorted_entries.len();
                if self.state.batch_selected.len() == total {
                    // All selected -> deselect all
                    self.state.batch_selected.clear();
                } else {
                    // Select all
                    self.state.batch_selected = (0..total).collect();
                }
                ctx.request_paint();
                true
            }

            // Batch: Switch to resize mode
            ClickAction::BatchModeResize => {
                self.state.batch_mode_resize = true;
                ctx.request_paint();
                true
            }

            // Batch: Switch to convert mode
            ClickAction::BatchModeConvert => {
                self.state.batch_mode_resize = false;
                ctx.request_paint();
                true
            }

            // Batch: Select format for batch conversion
            ClickAction::BatchSelectFormat(fmt) => {
                self.state.batch_format = fmt.clone();
                ctx.request_paint();
                true
            }

            // Batch: Apply batch operation
            ClickAction::BatchApply => {
                if self.state.batch_mode_resize {
                    self.execute_batch_resize();
                } else {
                    self.execute_batch_convert();
                }
                ctx.request_paint();
                true
            }

            // --- Phase 8: Slideshow ---

            // Slideshow: Cycle interval through 1s/2s/3s/5s/10s
            ClickAction::SlideshowCycleInterval => {
                let intervals = [1000u64, 2000, 3000, 5000, 10000];
                let current = self.state.slideshow_interval_ms;
                let next = intervals
                    .iter()
                    .find(|&&i| i > current)
                    .unwrap_or(&intervals[0]);
                self.state.slideshow_interval_ms = *next;
                // Update the timer interval if slideshow is playing
                if let Some(ref mut timer) = self.state.slideshow_timer {
                    timer.interval_ms = *next as f64;
                    timer.reset();
                }
                ctx.request_paint();
                true
            }

            // --- Phase 12: Rename dialog ---
            ClickAction::RenameConfirm => {
                if let Some(ref doc) = self.state.document.clone() {
                    let old_path = doc.path.clone();
                    let new_name = self.state.rename_input_text.trim().to_string();
                    if !new_name.is_empty() {
                        match image_service::rename_image_file(&old_path, &new_name) {
                            Ok(new_path_str) => {
                                // Update document metadata
                                self.state.document = Some(state::ImageMetadata {
                                    file_name: new_name.clone(),
                                    format: new_name
                                        .rsplit('.')
                                        .next()
                                        .unwrap_or("png")
                                        .to_lowercase(),
                                    dimensions: self
                                        .state
                                        .document
                                        .as_ref()
                                        .and_then(|d| d.dimensions),
                                    file_size: self
                                        .state
                                        .document
                                        .as_ref()
                                        .map(|d| d.file_size)
                                        .unwrap_or(0),
                                    path: new_path_str.clone(),
                                });
                                // Update folder entries
                                for entry in &mut self.state.folder_entries {
                                    if entry.path == old_path {
                                        entry.path = new_path_str.clone();
                                        entry.file_name = new_name.clone();
                                    }
                                }
                                self.state.sort_entries();
                                // Update image cache key
                                let cached_img = self.image_cache.get(&old_path).cloned();
                                if cached_img.is_some() {
                                    self.image_cache.remove(&old_path);
                                }
                                if let Some(img) = cached_img {
                                    self.image_cache.insert(new_path_str, img);
                                }
                                self.state.status_message = format!("Renamed to {}", new_name);
                            }
                            Err(e) => {
                                self.state.status_message = format!("Rename failed: {}", e);
                            }
                        }
                    }
                }
                self.state.show_rename = false;
                ctx.request_paint();
                true
            }
            ClickAction::RenameCancel => {
                self.state.show_rename = false;
                ctx.request_paint();
                true
            }

            // --- Phase 9: Compare mode ---

            // Compare: Drag start (handled in pointer events, but also register here)
            ClickAction::CompareDragStart => {
                // Actual drag handling is in on_pointer_event
                ctx.request_paint();
                true
            }

            // --- AI panel ---
            ClickAction::SelectAiFeature(feature) => {
                self.state.ai_selected_feature = Some(*feature);
                self.state.ai_result_text = None;
                ctx.request_paint();
                true
            }

            ClickAction::RunAi => {
                if let Some(feature) = self.state.ai_selected_feature {
                    self.state.ai_running = true;
                    match feature {
                        state::AiFeature::Enhance => self.run_ai_enhance(ctx),
                        state::AiFeature::Upscale => self.run_ai_upscale(ctx),
                        state::AiFeature::BackgroundRemoval => self.run_ai_bg_remove(ctx),
                        state::AiFeature::SmartCrop => self.run_ai_smart_crop(ctx),
                        state::AiFeature::Tag | state::AiFeature::Describe => {
                            self.run_ai_text_feature(feature, ctx);
                        }
                    }
                }
                true
            }

            // --- Convert output path ---
            ClickAction::ConvertBrowseOutput => {
                self.open_save_as_dialog("converted");
                ctx.request_paint();
                true
            }

            // --- URL open ---
            ClickAction::OpenFromUrl => {
                self.state.show_url_dialog = true;
                ctx.request_paint();
                true
            }

            ClickAction::LoadFromUrl => {
                let url = self.state.url_input_text.clone();
                if !url.is_empty() {
                    self.state.show_url_dialog = false;
                    self.load_image_from_url(&url);
                    ctx.request_paint();
                }
                true
            }
            ClickAction::UrlCancel => {
                self.state.show_url_dialog = false;
                ctx.request_paint();
                true
            }

            // --- Rating ---
            _ => false,
        }
    }
}
