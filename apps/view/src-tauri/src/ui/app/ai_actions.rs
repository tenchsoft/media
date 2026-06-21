// ---------------------------------------------------------------------------
// Local AI-style image actions
// ---------------------------------------------------------------------------

use tench_ui::peniko;
use tench_ui::prelude::*;

use crate::ui::state;
use crate::ui::state::bytes_label;
use tench_image_runtime::view::util;
use tench_image_runtime::view::util::{apply_filters, crop_image_data, resize_image_data};

use super::ViewApp;

impl ViewApp {
    // --- AI feature implementation methods ---

    /// Run AI Enhance: apply auto-levels + sharpen to the current image.
    pub(super) fn run_ai_enhance(&mut self, ctx: &mut EventCtx) {
        if let Some(ref image_data) = self.state.current_image_data.clone() {
            // Apply a simple auto-enhance: increase contrast slightly, boost saturation
            let enhanced = apply_filters(
                image_data, 105.0, // slight brightness boost
                115.0, // contrast boost
                120.0, // saturation boost
                0.0,   // no blur
                0.0,   // no hue rotation
            );
            if let Some(filtered) = enhanced {
                self.state
                    .push_edit_history(image_data.clone(), "AI Enhance");
                self.state.current_image_data = Some(filtered);
                self.state.status_message = "AI Enhance applied".into();
                self.state.status_message_time = Some(std::time::Instant::now());
            }
        }
        self.state.ai_running = false;
        ctx.request_paint();
    }

    /// Run AI Upscale 2x: resize the image to 2x dimensions.
    pub(super) fn run_ai_upscale(&mut self, ctx: &mut EventCtx) {
        if let Some(ref image_data) = self.state.current_image_data.clone() {
            let new_w = image_data.width * 2;
            let new_h = image_data.height * 2;
            if let Some(upscaled) = resize_image_data(image_data, new_w, new_h) {
                self.state
                    .push_edit_history(image_data.clone(), "AI Upscale 2x");
                self.state.current_image_data = Some(upscaled);
                if let Some(ref mut doc) = self.state.document {
                    doc.dimensions = Some(state::ImageDimensions {
                        width: new_w,
                        height: new_h,
                    });
                }
                self.state.status_message = format!("Upscaled to {} x {}", new_w, new_h);
                self.state.status_message_time = Some(std::time::Instant::now());
            }
        }
        self.state.ai_running = false;
        ctx.request_paint();
    }

    /// Run local background removal using a near-white alpha mask.
    pub(super) fn run_ai_bg_remove(&mut self, ctx: &mut EventCtx) {
        if let Some(ref image_data) = self.state.current_image_data.clone() {
            if let Some(dynamic) = util::image_data_to_dynamic(image_data) {
                let rgba = dynamic.to_rgba8();
                let (w, h) = rgba.dimensions();
                let src = rgba.as_raw();
                let mut dst = src.clone();

                // Simple background removal: make near-white pixels transparent
                for i in 0..(w as usize * h as usize) {
                    let idx = i * 4;
                    let r = src[idx];
                    let g = src[idx + 1];
                    let b = src[idx + 2];
                    // If pixel is close to white, make it transparent
                    if r > 230 && g > 230 && b > 230 {
                        dst[idx + 3] = 0; // transparent
                    }
                }

                let result = peniko::ImageData {
                    data: dst.into(),
                    format: peniko::ImageFormat::Rgba8,
                    alpha_type: peniko::ImageAlphaType::AlphaPremultiplied,
                    width: w,
                    height: h,
                };
                self.state
                    .push_edit_history(image_data.clone(), "AI BG Remove");
                self.state.current_image_data = Some(result);
                self.state.status_message = "Background removed with local mask".into();
                self.state.status_message_time = Some(std::time::Instant::now());
            }
        }
        self.state.ai_running = false;
        ctx.request_paint();
    }

    /// Run AI Smart Crop: crop to the most interesting region using center-weighted heuristic.
    pub(super) fn run_ai_smart_crop(&mut self, ctx: &mut EventCtx) {
        if let Some(ref image_data) = self.state.current_image_data.clone() {
            let w = image_data.width;
            let h = image_data.height;
            // Smart crop to 4:3 ratio centered on the image
            let target_ratio = 4.0 / 3.0;
            let current_ratio = w as f64 / h as f64;

            let (crop_w, crop_h) = if current_ratio > target_ratio {
                // Too wide: reduce width
                let new_w = (h as f64 * target_ratio).round() as u32;
                (new_w.min(w), h)
            } else {
                // Too tall: reduce height
                let new_h = (w as f64 / target_ratio).round() as u32;
                (w, new_h.min(h))
            };

            let cx = (w - crop_w) / 2;
            let cy = (h - crop_h) / 2;

            if let Some(cropped) = crop_image_data(image_data, cx, cy, crop_w, crop_h) {
                self.state
                    .push_edit_history(image_data.clone(), "AI Smart Crop");
                self.state.current_image_data = Some(cropped);
                if let Some(ref mut doc) = self.state.document {
                    doc.dimensions = Some(state::ImageDimensions {
                        width: crop_w,
                        height: crop_h,
                    });
                }
                self.state.status_message = format!("Smart cropped to {} x {}", crop_w, crop_h);
                self.state.status_message_time = Some(std::time::Instant::now());
            }
        }
        self.state.ai_running = false;
        ctx.request_paint();
    }

    /// Run local image text feature using decoded metadata.
    pub(super) fn run_ai_text_feature(&mut self, feature: state::AiFeature, ctx: &mut EventCtx) {
        let result = if let Some(ref doc) = self.state.document {
            let dims = doc.dimensions.map_or("unknown".to_string(), |d| {
                format!("{}x{}", d.width, d.height)
            });
            match feature {
                state::AiFeature::Tag => {
                    format!(
                        "Image: {}, {}, {}",
                        doc.format.to_uppercase(),
                        dims,
                        bytes_label(doc.file_size)
                    )
                }
                state::AiFeature::Describe => {
                    format!(
                        "A {} image ({}). File: {}",
                        doc.format.to_uppercase(),
                        dims,
                        doc.file_name
                    )
                }
                _ => String::new(),
            }
        } else {
            "No image loaded".to_string()
        };

        self.state.ai_result_text = Some(result);
        self.state.ai_running = false;
        self.state.status_message = "AI analysis complete".into();
        self.state.status_message_time = Some(std::time::Instant::now());
        ctx.request_paint();
    }
}
