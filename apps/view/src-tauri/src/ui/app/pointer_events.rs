// ---------------------------------------------------------------------------
// Pointer event handling
// ---------------------------------------------------------------------------

use tench_ui::core::events::PointerButton;
use tench_ui::prelude::*;

use crate::ui::image_stage;
use crate::ui::state::{self, AnnotationTool, ClickAction, EditTool, FitMode};

use super::ViewApp;

impl ViewApp {
    pub(super) fn handle_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::Scroll(e) => {
                // Zoom with scroll wheel
                if self.state.document.is_some() {
                    if e.delta.y > 0.0 {
                        self.state.zoom = (self.state.zoom * 1.1).clamp(0.1, 8.0);
                    } else {
                        self.state.zoom = (self.state.zoom / 1.1).clamp(0.1, 8.0);
                    }
                    self.state.fit_mode = FitMode::Actual;
                    self.state.clamp_pan(ctx.state.size);
                    ctx.request_paint();
                }
            }
            PointerEvent::Down(btn) => {
                match btn.button {
                    PointerButton::Primary => {
                        // Check click regions (populated during paint) first
                        let action = self
                            .state
                            .click_action_at(btn.pos.x, btn.pos.y)
                            .cloned()
                            .or_else(|| {
                                // Fallback: compute rects directly when click_regions
                                // is empty (e.g. before first paint in headless tests).
                                self.state.click_action_at_computed(
                                    btn.pos.x,
                                    btn.pos.y,
                                    ctx.state.size.width,
                                    ctx.state.size.height,
                                )
                            });

                        if matches!(action, Some(ClickAction::CompareDragStart)) {
                            self.state.compare_dragging = true;
                            self.state.compare_split =
                                (btn.pos.x / ctx.state.size.width * 100.0).clamp(5.0, 95.0);
                            ctx.request_paint();
                            return;
                        }

                        if let Some(action) = action {
                            self.dispatch_click_action(&action, ctx);
                            return;
                        }

                        // Crop tool: start drag selection
                        if self.state.active_edit_tool == Some(EditTool::Crop) {
                            if let Some(ref image_data) = self.state.current_image_data {
                                let img_w = image_data.width as f64;
                                let img_h = image_data.height as f64;
                                let toolbar_h = 44.0;
                                let canvas_w = ctx.state.size.width;
                                let canvas_h = ctx.state.size.height - toolbar_h;
                                let scale_x = canvas_w / img_w;
                                let scale_y = canvas_h / img_h;
                                let scale = scale_x.min(scale_y).min(1.0);
                                let display_w = img_w * scale;
                                let display_h = img_h * scale;
                                let img_x = (canvas_w - display_w) / 2.0;
                                let img_y = toolbar_h + (canvas_h - display_h) / 2.0;

                                // Convert screen coords to image pixel coords
                                let px = (btn.pos.x - img_x) / scale;
                                let py = (btn.pos.y - img_y) / scale;
                                if px >= 0.0 && px < img_w && py >= 0.0 && py < img_h {
                                    self.state.crop_start = Some((px, py));
                                    self.state.crop_selection = None;
                                    self.state.crop_dragging = true;
                                    ctx.request_paint();
                                    return;
                                }
                            }
                        }

                        // Annotation tool: start drawing
                        if let Some(tool) = self.state.active_annotation_tool {
                            if self.state.document.is_some() {
                                if let Some(ref image_data) = self.state.current_image_data {
                                    let img_w = image_data.width as f64;
                                    let img_h = image_data.height as f64;
                                    let toolbar_h = 44.0;
                                    let canvas_w = ctx.state.size.width;
                                    let canvas_h = ctx.state.size.height - toolbar_h;
                                    let scale_x = canvas_w / img_w;
                                    let scale_y = canvas_h / img_h;
                                    let scale = scale_x.min(scale_y).min(1.0);
                                    let display_w = img_w * scale;
                                    let display_h = img_h * scale;
                                    let img_x = (canvas_w - display_w) / 2.0;
                                    let img_y = toolbar_h + (canvas_h - display_h) / 2.0;

                                    let px = (btn.pos.x - img_x) / scale;
                                    let py = (btn.pos.y - img_y) / scale;

                                    if px >= 0.0 && px < img_w && py >= 0.0 && py < img_h {
                                        // Eraser: hit-test annotations and remove
                                        if tool == AnnotationTool::Eraser {
                                            let hit_radius = 8.0;
                                            if let Some(idx) =
                                                self.state.annotations.iter().position(|ann| {
                                                    let ax = ann.x;
                                                    let ay = ann.y;
                                                    let bx = ann.x + ann.w;
                                                    let by = ann.y + ann.h;
                                                    // Point-to-rect distance
                                                    let cx = px.max(ax).min(bx);
                                                    let cy = py.max(ay).min(by);
                                                    let dx = px - cx;
                                                    let dy = py - cy;
                                                    (dx * dx + dy * dy) <= hit_radius * hit_radius
                                                })
                                            {
                                                // Save undo snapshot before removing
                                                self.state
                                                    .annotation_undo_stack
                                                    .push(self.state.annotations.clone());
                                                self.state.annotation_redo_stack.clear();
                                                self.state.annotations.remove(idx);
                                            }
                                            ctx.request_paint();
                                            return;
                                        }

                                        // Text: set annotation_text_input position
                                        if tool == AnnotationTool::Text {
                                            self.state.annotation_text_input = Some(String::new());
                                            self.state.annotation_drag_start = Some((px, py));
                                            ctx.request_paint();
                                            return;
                                        }

                                        // Others (Arrow, Rectangle, Circle, Freeform, BlurMask): start drag
                                        self.state.annotation_drag_start = Some((px, py));
                                        self.state.annotation_dragging = true;
                                        ctx.request_paint();
                                        return;
                                    }
                                }
                            }
                        }

                        // Filter panel: check if clicking on a slider track
                        if self.state.show_filter {
                            if let Some(slider) = self.hit_filter_slider(btn.pos.x, btn.pos.y) {
                                self.state.filter_dragging = Some(slider);
                                self.update_filter_from_pointer(slider, btn.pos.x);
                                ctx.request_paint();
                                return;
                            }
                        }

                        // Compare mode: check if clicking on split handle
                        if self.state.show_compare {
                            if let Some(action) = self.state.click_action_at(btn.pos.x, btn.pos.y) {
                                if matches!(action, ClickAction::CompareDragStart) {
                                    self.state.compare_dragging = true;
                                    self.state.compare_split =
                                        (btn.pos.x / ctx.state.size.width * 100.0).clamp(5.0, 95.0);
                                    ctx.request_paint();
                                    return;
                                }
                            }
                        }

                        // Double-click detection
                        if self.state.is_double_click(btn.pos.x, btn.pos.y) {
                            // Toggle Fit <-> Actual
                            match self.state.fit_mode {
                                FitMode::Fit => {
                                    self.state.fit_mode = FitMode::Actual;
                                    self.state.zoom = 1.0;
                                }
                                FitMode::Actual => {
                                    self.state.fit_mode = FitMode::Fit;
                                    self.state.zoom = 1.0;
                                    self.state.pan_x = 0.0;
                                    self.state.pan_y = 0.0;
                                }
                            }
                            ctx.request_paint();
                            return;
                        }

                        // Start pan drag in actual mode
                        if self.state.fit_mode == FitMode::Actual && self.state.document.is_some() {
                            self.state.drag_state = Some(state::DragState {
                                start_x: btn.pos.x,
                                start_y: btn.pos.y,
                                pan_x: self.state.pan_x,
                                pan_y: self.state.pan_y,
                            });
                        }
                        // Show chrome on click
                        if !self.state.show_chrome {
                            self.state.show_chrome = true;
                        }
                        ctx.request_paint();
                    }
                    PointerButton::Secondary => {
                        // Right-click: context menu
                        self.state.show_context_menu = true;
                        self.state.context_menu_x = btn.pos.x;
                        self.state.context_menu_y = btn.pos.y;
                        ctx.request_paint();
                    }
                    _ => {}
                }
            }
            PointerEvent::Up(_) => {
                // End pan drag
                self.state.drag_state = None;
                // End crop drag
                self.state.crop_dragging = false;
                // End filter drag
                self.state.filter_dragging = None;
                // End compare split drag
                self.state.compare_dragging = false;

                // Finalize annotation drag
                if self.state.annotation_dragging {
                    self.state.annotation_dragging = false;
                    if let (Some((sx, sy)), Some(ref image_data)) = (
                        self.state.annotation_drag_start,
                        &self.state.current_image_data,
                    ) {
                        let tool = self
                            .state
                            .active_annotation_tool
                            .unwrap_or(AnnotationTool::Freeform);
                        let img_w = image_data.width as f64;
                        let img_h = image_data.height as f64;
                        let toolbar_h = 44.0;
                        let canvas_w = ctx.state.size.width;
                        let canvas_h = ctx.state.size.height - toolbar_h;
                        let scale_x = canvas_w / img_w;
                        let scale_y = canvas_h / img_h;
                        let scale = scale_x.min(scale_y).min(1.0);
                        let display_w = img_w * scale;
                        let display_h = img_h * scale;
                        let _img_x = (canvas_w - display_w) / 2.0;
                        let _img_y = toolbar_h + (canvas_h - display_h) / 2.0;

                        // Use the last known pointer position stored in drag_start
                        // For a proper implementation, we'd track the end position.
                        // For now, use drag_start as end (zero-size annotation = point click).
                        let ex = sx;
                        let ey = sy;

                        let x = sx.min(ex);
                        let y = sy.min(ey);
                        let w = (sx - ex).abs().max(2.0);
                        let h = (sy - ey).abs().max(2.0);

                        // Save undo snapshot before adding
                        self.state
                            .annotation_undo_stack
                            .push(self.state.annotations.clone());
                        self.state.annotation_redo_stack.clear();

                        self.state.annotations.push(state::Annotation {
                            tool,
                            x,
                            y,
                            w,
                            h,
                            text: String::new(),
                            color: self.state.annotation_color,
                            line_width: self.state.annotation_line_width,
                        });
                    }
                    self.state.annotation_drag_start = None;
                    ctx.request_paint();
                }
            }
            PointerEvent::Move(m) => {
                // Annotation drag: update in-progress annotation
                if self.state.annotation_dragging {
                    if let (Some((sx, sy)), Some(ref image_data)) = (
                        self.state.annotation_drag_start,
                        &self.state.current_image_data,
                    ) {
                        let img_w = image_data.width as f64;
                        let img_h = image_data.height as f64;
                        let toolbar_h = 44.0;
                        let canvas_w = ctx.state.size.width;
                        let canvas_h = ctx.state.size.height - toolbar_h;
                        let scale_x = canvas_w / img_w;
                        let scale_y = canvas_h / img_h;
                        let scale = scale_x.min(scale_y).min(1.0);
                        let display_w = img_w * scale;
                        let display_h = img_h * scale;
                        let img_x = (canvas_w - display_w) / 2.0;
                        let img_y = toolbar_h + (canvas_h - display_h) / 2.0;

                        let px = ((m.pos.x - img_x) / scale).clamp(0.0, img_w);
                        let py = ((m.pos.y - img_y) / scale).clamp(0.0, img_h);

                        // Store current end position for the Up handler to finalize
                        // Update the last annotation in-place if one was already created,
                        // or update drag_start to reflect the current endpoint.
                        // We'll finalize in the Up handler, so just update the preview.
                        // For live preview, we store the current endpoint in the last annotation.
                        let tool = self
                            .state
                            .active_annotation_tool
                            .unwrap_or(AnnotationTool::Freeform);
                        let x = sx.min(px);
                        let y = sy.min(py);
                        let w = (sx - px).abs();
                        let h = (sy - py).abs();

                        // Remove the last annotation if it was a preview (same tool, same start)
                        if let Some(last) = self.state.annotations.last() {
                            if last.tool == tool
                                && (last.x - sx).abs() < 1.0
                                && (last.y - sy).abs() < 1.0
                            {
                                self.state.annotations.pop();
                            }
                        }

                        if w > 1.0 || h > 1.0 {
                            self.state.annotations.push(state::Annotation {
                                tool,
                                x,
                                y,
                                w,
                                h,
                                text: String::new(),
                                color: self.state.annotation_color,
                                line_width: self.state.annotation_line_width,
                            });
                        }
                        ctx.request_paint();
                        return;
                    }
                }

                // Crop drag: update selection
                if self.state.crop_dragging {
                    if let (Some((sx, sy)), Some(ref image_data)) =
                        (self.state.crop_start, &self.state.current_image_data)
                    {
                        let img_w = image_data.width as f64;
                        let img_h = image_data.height as f64;
                        let toolbar_h = 44.0;
                        let canvas_w = ctx.state.size.width;
                        let canvas_h = ctx.state.size.height - toolbar_h;
                        let scale_x = canvas_w / img_w;
                        let scale_y = canvas_h / img_h;
                        let scale = scale_x.min(scale_y).min(1.0);
                        let display_w = img_w * scale;
                        let display_h = img_h * scale;
                        let img_x = (canvas_w - display_w) / 2.0;
                        let img_y = toolbar_h + (canvas_h - display_h) / 2.0;

                        let px = ((m.pos.x - img_x) / scale).clamp(0.0, img_w);
                        let py = ((m.pos.y - img_y) / scale).clamp(0.0, img_h);

                        let (mut w, mut h) = ((sx - px).abs(), (sy - py).abs());
                        if let Some((rw, rh)) = self.state.crop_aspect_ratio {
                            let target_ratio = rw as f64 / rh as f64;
                            if w / h > target_ratio {
                                w = h * target_ratio;
                            } else {
                                h = w / target_ratio;
                            }
                        }
                        let x = if px < sx { sx - w } else { sx };
                        let y = if py < sy { sy - h } else { sy };
                        self.state.crop_selection = Some((x, y, w, h));
                        ctx.request_paint();
                        return;
                    }
                }

                // Filter slider drag
                if let Some(slider) = self.state.filter_dragging {
                    self.update_filter_from_pointer(slider, m.pos.x);
                    ctx.request_paint();
                    return;
                }

                // Compare split drag
                if self.state.compare_dragging {
                    self.state.compare_split =
                        (m.pos.x / ctx.state.size.width * 100.0).clamp(5.0, 95.0);
                    ctx.request_paint();
                    return;
                }

                // Pan drag
                if let Some(ref drag) = self.state.drag_state {
                    self.state.pan_x = drag.pan_x + m.pos.x - drag.start_x;
                    self.state.pan_y = drag.pan_y + m.pos.y - drag.start_y;
                    self.state.clamp_pan(ctx.state.size);
                    ctx.request_paint();
                }

                // Pixel color picker: sample pixel under cursor
                if let Some(ref image_data) = self.state.current_image_data {
                    let doc = self.state.document.as_ref();
                    let dims = doc.and_then(|d| d.dimensions);
                    let img_rect =
                        image_stage::compute_image_rect(&self.state, ctx.state.size, dims);
                    let img_w = image_data.width as f64;
                    let img_h = image_data.height as f64;
                    let rel_x = m.pos.x - img_rect.x0;
                    let rel_y = m.pos.y - img_rect.y0;
                    if rel_x >= 0.0
                        && rel_y >= 0.0
                        && rel_x < img_rect.width()
                        && rel_y < img_rect.height()
                    {
                        let px = (rel_x / img_rect.width() * img_w) as u32;
                        let py = (rel_y / img_rect.height() * img_h) as u32;
                        if px < image_data.width && py < image_data.height {
                            let idx = (py * image_data.width + px) as usize * 4;
                            let pixels = image_data.data.data();
                            if idx + 3 < pixels.len() {
                                self.state.pixel_info = Some(state::PixelInfo {
                                    x: px,
                                    y: py,
                                    r: pixels[idx],
                                    g: pixels[idx + 1],
                                    b: pixels[idx + 2],
                                    a: pixels[idx + 3],
                                });
                            }
                        }
                    } else {
                        self.state.pixel_info = None;
                    }
                }
            }
            PointerEvent::Enter | PointerEvent::Leave => {
                self.state.drag_state = None;
                self.state.crop_dragging = false;
                self.state.filter_dragging = None;
                self.state.compare_dragging = false;
                self.state.annotation_dragging = false;
            }
        }
    }
}
