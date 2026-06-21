// ---------------------------------------------------------------------------
// Click action dispatch: tools
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::{self, ClickAction};
use tench_image_runtime::view::service as image_service;
use tench_image_runtime::view::util::{apply_filters, crop_image_data, resize_image_data};

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_tool_action(
        &mut self,
        action: &ClickAction,
        ctx: &mut EventCtx,
    ) -> bool {
        match action {
            ClickAction::CropApply => {
                if let Some((x, y, w, h)) = self.state.crop_selection {
                    if w > 2.0 && h > 2.0 {
                        // Extract values to release borrow before mutating state
                        let crop_result =
                            self.state
                                .current_image_data
                                .as_ref()
                                .and_then(|image_data| {
                                    let cx = x.round() as u32;
                                    let cy = y.round() as u32;
                                    let cw = w.round() as u32;
                                    let ch = h.round() as u32;
                                    let cx = cx.min(image_data.width.saturating_sub(1));
                                    let cy = cy.min(image_data.height.saturating_sub(1));
                                    let cw = cw.min(image_data.width - cx);
                                    let ch = ch.min(image_data.height - cy);
                                    if cw > 0 && ch > 0 {
                                        Some((image_data.clone(), cx, cy, cw, ch))
                                    } else {
                                        None
                                    }
                                });
                        if let Some((data, cx, cy, cw, ch)) = crop_result {
                            self.state.push_edit_history(data.clone(), "crop");
                            if let Some(cropped) = crop_image_data(&data, cx, cy, cw, ch) {
                                self.state.current_image_data = Some(cropped);
                                if let Some(ref mut doc) = self.state.document {
                                    doc.dimensions = Some(state::ImageDimensions {
                                        width: cw,
                                        height: ch,
                                    });
                                }
                                self.state.status_message = format!("Cropped to {} x {}", cw, ch);
                            }
                        }
                    }
                }
                self.state.active_edit_tool = None;
                self.state.crop_start = None;
                self.state.crop_selection = None;
                ctx.request_paint();
                true
            }
            // Crop Cancel: dismiss crop tool
            ClickAction::CropCancel => {
                self.state.active_edit_tool = None;
                self.state.crop_start = None;
                self.state.crop_selection = None;
                ctx.request_paint();
                true
            }
            // Crop aspect ratio: set constraint
            ClickAction::CropAspectRatio(w, h) => {
                self.state.crop_aspect_ratio = Some((*w, *h));
                ctx.request_paint();
                true
            }
            // Crop aspect ratio: free (no constraint)
            ClickAction::CropAspectRatioFree => {
                self.state.crop_aspect_ratio = None;
                ctx.request_paint();
                true
            }
            // Resize Apply: resize image with Lanczos3
            ClickAction::ResizeApply => {
                let new_w = self.state.resize_width.max(1);
                let new_h = self.state.resize_height.max(1);
                // Extract to release borrow before mutating state
                let resize_input = self.state.current_image_data.clone();
                if let Some(data) = resize_input {
                    self.state.push_edit_history(data.clone(), "resize");
                    if let Some(resized) = resize_image_data(&data, new_w, new_h) {
                        self.state.current_image_data = Some(resized);
                        if let Some(ref mut doc) = self.state.document {
                            doc.dimensions = Some(state::ImageDimensions {
                                width: new_w,
                                height: new_h,
                            });
                        }
                        self.state.status_message = format!("Resized to {} x {}", new_w, new_h);
                    }
                }
                self.state.active_edit_tool = None;
                ctx.request_paint();
                true
            }
            // Resize Cancel: dismiss resize tool
            ClickAction::ResizeCancel => {
                self.state.active_edit_tool = None;
                ctx.request_paint();
                true
            }
            // Resize width minus
            ClickAction::ResizeWidthMinus => {
                let step = if self.state.resize_width > 100 { 10 } else { 1 };
                self.state.resize_width = self.state.resize_width.saturating_sub(step).max(1);
                if self.state.resize_maintain_aspect && self.state.resize_orig_width > 0 {
                    let ratio =
                        self.state.resize_orig_height as f64 / self.state.resize_orig_width as f64;
                    self.state.resize_height =
                        (self.state.resize_width as f64 * ratio).round() as u32;
                }
                ctx.request_paint();
                true
            }
            // Resize width plus
            ClickAction::ResizeWidthPlus => {
                let step = if self.state.resize_width >= 100 {
                    10
                } else {
                    1
                };
                self.state.resize_width = self.state.resize_width.saturating_add(step).min(10000);
                if self.state.resize_maintain_aspect && self.state.resize_orig_width > 0 {
                    let ratio =
                        self.state.resize_orig_height as f64 / self.state.resize_orig_width as f64;
                    self.state.resize_height =
                        (self.state.resize_width as f64 * ratio).round() as u32;
                }
                ctx.request_paint();
                true
            }
            // Resize height minus
            ClickAction::ResizeHeightMinus => {
                let step = if self.state.resize_height > 100 {
                    10
                } else {
                    1
                };
                self.state.resize_height = self.state.resize_height.saturating_sub(step).max(1);
                if self.state.resize_maintain_aspect && self.state.resize_orig_height > 0 {
                    let ratio =
                        self.state.resize_orig_width as f64 / self.state.resize_orig_height as f64;
                    self.state.resize_width =
                        (self.state.resize_height as f64 * ratio).round() as u32;
                }
                ctx.request_paint();
                true
            }
            // Resize height plus
            ClickAction::ResizeHeightPlus => {
                let step = if self.state.resize_height >= 100 {
                    10
                } else {
                    1
                };
                self.state.resize_height = self.state.resize_height.saturating_add(step).min(10000);
                if self.state.resize_maintain_aspect && self.state.resize_orig_height > 0 {
                    let ratio =
                        self.state.resize_orig_width as f64 / self.state.resize_orig_height as f64;
                    self.state.resize_width =
                        (self.state.resize_height as f64 * ratio).round() as u32;
                }
                ctx.request_paint();
                true
            }
            // Toggle maintain aspect ratio
            ClickAction::ResizeToggleAspect => {
                self.state.resize_maintain_aspect = !self.state.resize_maintain_aspect;
                if self.state.resize_maintain_aspect && self.state.resize_orig_width > 0 {
                    let ratio =
                        self.state.resize_orig_height as f64 / self.state.resize_orig_width as f64;
                    self.state.resize_height =
                        (self.state.resize_width as f64 * ratio).round() as u32;
                }
                ctx.request_paint();
                true
            }
            // Convert: select format
            ClickAction::ConvertSelectFormat(fmt) => {
                self.state.convert_format = fmt.clone();
                ctx.request_paint();
                true
            }
            // Convert Apply: save image in the selected format
            ClickAction::ConvertApply => {
                if let (Some(ref image_data), Some(ref doc)) =
                    (&self.state.current_image_data, &self.state.document)
                {
                    let format = self.state.convert_format.clone();
                    let w = image_data.width;
                    let h = image_data.height;
                    let pixels = image_data.data.data().to_vec();

                    // Compute new file path with new extension
                    let new_path = self.state.convert_output_path.clone().unwrap_or_else(|| {
                        let original_path = doc.path.clone();
                        original_path
                            .rsplit_once('.')
                            .map(|(base, _)| format!("{}.{}", base, format))
                            .unwrap_or_else(|| format!("{}.{}", original_path, format))
                    });

                    let format_label = format.to_uppercase();
                    std::thread::spawn(move || {
                        let _ = image_service::convert_rgba_pixels_to_path(
                            w, h, pixels, &new_path, &format,
                        );
                    });
                    self.state.status_message = format!("Converted to {}", format_label);
                }
                self.state.active_edit_tool = None;
                ctx.request_paint();
                true
            }
            // Convert Cancel: dismiss convert tool
            ClickAction::ConvertCancel => {
                self.state.active_edit_tool = None;
                ctx.request_paint();
                true
            }
            // --- Phase 6: Filter real behavior ---

            // Filter Reset: restore default filter values and original image
            ClickAction::FilterReset => {
                self.state.reset_filters();
                // Restore original image data if available
                if let Some(ref original) = self.state.original_image_data {
                    self.state.current_image_data = Some(original.clone());
                }
                self.state.filter_dirty = false;
                ctx.request_paint();
                true
            }
            // Filter Apply: apply current filter values to the image
            ClickAction::FilterApply => {
                if let Some(ref original) = self.state.original_image_data {
                    let brightness = self.state.filter_brightness;
                    let contrast = self.state.filter_contrast;
                    let saturation = self.state.filter_saturation;
                    let blur = self.state.filter_blur;
                    let hue_rotate = self.state.filter_hue_rotate;
                    // Check if any filter is non-default
                    let is_modified = brightness != 100.0
                        || contrast != 100.0
                        || saturation != 100.0
                        || hue_rotate != 0.0
                        || blur != 0.0;
                    if is_modified {
                        if let Some(filtered) = apply_filters(
                            original, brightness, contrast, saturation, blur, hue_rotate,
                        ) {
                            // Push to history before replacing
                            if let Some(ref current) = self.state.current_image_data {
                                self.state.push_edit_history(current.clone(), "filter");
                            }
                            self.state.current_image_data = Some(filtered);
                            self.state.status_message = "Filters applied".into();
                            self.state.status_message_time = Some(std::time::Instant::now());
                        }
                    }
                }
                self.state.filter_dirty = false;
                ctx.request_paint();
                true
            }

            // --- Phase 7: Batch processing ---

            // Batch: Toggle individual file selection
            _ => false,
        }
    }
}
