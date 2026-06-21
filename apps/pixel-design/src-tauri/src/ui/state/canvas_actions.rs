use super::*;
use tench_pixel_core::{BrushStroke, PixelBuffer, StrokePoint};

impl PixelDesignState {
    /// Helper: convert fg_color to (r, g, b, a) tuple.
    pub fn fg_color_rgba(&self) -> (u8, u8, u8, u8) {
        let packed = self.fg_color.to_u32();
        (
            ((packed >> 24) & 0xFF) as u8,
            ((packed >> 16) & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
            255u8,
        )
    }

    pub fn begin_canvas_action(&mut self, x: f64, y: f64) {
        self.mouse_pos = CanvasPoint { x, y };

        // If space is held, start panning regardless of tool
        if self.space_held {
            self.is_panning = true;
            self.pan_start = Some(CanvasPoint { x, y });
            return;
        }

        match self.active_tool {
            Tool::Brush | Tool::Eraser => {
                self.is_drawing = true;
                self.push_history("Draw");

                let color = self.fg_color_rgba();
                let is_eraser = self.active_tool == Tool::Eraser;
                let mut stroke = BrushStroke::new(
                    color,
                    self.brush_size as f64,
                    self.brush_opacity as f32 / 100.0,
                    is_eraser,
                );
                stroke.add_point(StrokePoint::new(x, y));
                self.current_stroke = Some(stroke);
                self.status_msg = format!("Drawing at {}, {}", x.round(), y.round());
            }
            Tool::Fill => {
                self.push_history("Fill");
                let px = x as u32;
                let py = y as u32;
                let color = self.fg_color_rgba();
                let norm_sel = self.normalized_selection();
                if let Some(layer) = self.document.active_layer_mut() {
                    // Check if we have a selection to constrain
                    if let Some(sel) = norm_sel {
                        let sx = sel.start.x as u32;
                        let sy = sel.start.y as u32;
                        let sw = (sel.end.x - sel.start.x) as u32;
                        let sh = (sel.end.y - sel.start.y) as u32;
                        // Fill only within selection rect
                        let mut temp = layer.buffer.extract_rect(sx, sy, sw, sh);
                        let local_x = px.saturating_sub(sx);
                        let local_y = py.saturating_sub(sy);
                        if local_x < sw && local_y < sh {
                            temp.flood_fill(
                                local_x, local_y, color.0, color.1, color.2, color.3, 32,
                            );
                            layer.buffer.paste(&temp, sx, sy);
                        }
                    } else {
                        layer
                            .buffer
                            .flood_fill(px, py, color.0, color.1, color.2, color.3, 32);
                    }
                    self.status_msg = "Filled region".into();
                }
            }
            Tool::Text => {
                self.text_pos = CanvasPoint { x, y };
                self.text_input.clear();
                self.show_text_input = true;
                self.status_msg = "Text insertion point set".into();
            }
            Tool::Select | Tool::Crop | Tool::Gradient => {
                self.is_drawing = true;
                let point = CanvasPoint { x, y };
                self.selection = Some(SelectionRect {
                    start: point,
                    end: point,
                });
                self.status_msg = format!("{} drag started", self.active_tool.label());
            }
            Tool::Shape => {
                self.is_drawing = true;
                self.push_history("Shape");
                self.shape_drag_start = Some(CanvasPoint { x, y });
                self.status_msg = "Shape drag started".into();
            }
            Tool::Move => {
                self.is_drawing = true;
                self.push_history("Move layer");
                self.move_drag_start = Some(CanvasPoint { x, y });
                if let Some(layer) = self.active_layer() {
                    self.move_layer_start_offset = Some((layer.offset_x, layer.offset_y));
                }
                self.status_msg = "Moving layer".into();
            }
            Tool::Hand => {
                self.is_panning = true;
                self.pan_start = Some(CanvasPoint { x, y });
                self.status_msg = "Canvas panned".into();
            }
            Tool::Eyedropper => {
                self.sample_color_at(x, y);
            }
        }
    }

    pub fn move_canvas_action(&mut self, x: f64, y: f64) {
        self.mouse_pos = CanvasPoint { x, y };

        // Handle panning
        if self.is_panning {
            if let Some(start) = self.pan_start {
                let dx = x - start.x;
                let dy = y - start.y;
                self.viewport_offset_x += dx;
                self.viewport_offset_y += dy;
                self.pan_start = Some(CanvasPoint { x, y });
            }
            return;
        }

        if self.is_drawing {
            if let Some(ref mut stroke) = self.current_stroke {
                stroke.add_point(StrokePoint::new(x, y));
            }
            if let Some(selection) = &mut self.selection {
                selection.end = CanvasPoint { x, y };
            }
            // Handle move tool drag
            if self.active_tool == Tool::Move {
                if let Some(start) = self.move_drag_start {
                    let dx = (x - start.x) as i32;
                    let dy = (y - start.y) as i32;
                    if let Some((ox, oy)) = self.move_layer_start_offset {
                        if let Some(layer) = self.document.active_layer_mut() {
                            layer.offset_x = ox + dx;
                            layer.offset_y = oy + dy;
                        }
                    }
                }
            }
        }
    }

    pub fn finish_canvas_action(&mut self) {
        // Handle panning end
        if self.is_panning {
            self.is_panning = false;
            self.pan_start = None;
            return;
        }

        if !self.is_drawing && self.current_stroke.is_none() {
            return;
        }
        self.is_drawing = false;

        // Rasterize brush stroke onto active layer
        if let Some(stroke) = self.current_stroke.take() {
            let norm_sel = self.normalized_selection();
            let doc_w = self.document.width;
            let doc_h = self.document.height;
            if let Some(layer) = self.document.active_layer_mut() {
                // If selection exists, constrain stroke to selection
                if let Some(sel) = norm_sel {
                    let mut temp_buf = PixelBuffer::new(doc_w, doc_h);
                    stroke.rasterize(&mut temp_buf);
                    // Mask: only copy pixels within selection
                    let sx = sel.start.x as u32;
                    let sy = sel.start.y as u32;
                    let ex = sel.end.x as u32;
                    let ey = sel.end.y as u32;
                    for py in sy..ey {
                        for px in sx..ex {
                            let (r, g, b, a) = temp_buf.pixel(px, py);
                            if a > 0 {
                                layer.buffer.set_pixel(px, py, r, g, b, a);
                            }
                        }
                    }
                } else {
                    stroke.rasterize(&mut layer.buffer);
                }
            }
            self.status_msg = "Stroke committed".into();
            return;
        }

        match self.active_tool {
            Tool::Crop => {
                if let Some(sel) = self.normalized_selection() {
                    if sel.end.x - sel.start.x > 2.0 && sel.end.y - sel.start.y > 2.0 {
                        self.document.crop(
                            sel.start.x as u32,
                            sel.start.y as u32,
                            (sel.end.x - sel.start.x) as u32,
                            (sel.end.y - sel.start.y) as u32,
                        );
                        self.push_history("Crop");
                        self.composited_image = None;
                        self.status_msg = "Cropped canvas".into();
                    }
                }
            }
            Tool::Gradient => {
                self.push_history("Gradient");
                if let Some(sel) = self.normalized_selection() {
                    self.apply_gradient(sel);
                } else {
                    // Apply to full canvas
                    let sel = SelectionRect {
                        start: CanvasPoint { x: 0.0, y: 0.0 },
                        end: CanvasPoint {
                            x: self.document.width as f64,
                            y: self.document.height as f64,
                        },
                    };
                    self.apply_gradient(sel);
                }
                self.status_msg = "Gradient applied".into();
            }
            Tool::Shape => {
                if let Some(start) = self.shape_drag_start.take() {
                    self.apply_shape(
                        start,
                        CanvasPoint {
                            x: self.mouse_pos.x,
                            y: self.mouse_pos.y,
                        },
                    );
                }
                self.status_msg = "Shape placed".into();
            }
            Tool::Move => {
                self.move_drag_start = None;
                self.move_layer_start_offset = None;
                self.status_msg = "Layer moved".into();
            }
            Tool::Select => self.status_msg = "Selection updated".into(),
            _ => {}
        }
    }

    fn apply_gradient(&mut self, sel: SelectionRect) {
        let fg_packed = self.fg_color.to_u32();
        let bg_packed = self.bg_color.to_u32();
        let fg_r = ((fg_packed >> 24) & 0xFF) as u8;
        let fg_g = ((fg_packed >> 16) & 0xFF) as u8;
        let fg_b = ((fg_packed >> 8) & 0xFF) as u8;
        let bg_r = ((bg_packed >> 24) & 0xFF) as u8;
        let bg_g = ((bg_packed >> 16) & 0xFF) as u8;
        let bg_b = ((bg_packed >> 8) & 0xFF) as u8;

        if let Some(layer) = self.document.active_layer_mut() {
            let x0 = sel.start.x as u32;
            let y0 = sel.start.y as u32;
            let x1 = sel.end.x as u32;
            let y1 = sel.end.y as u32;
            let width = (x1 - x0).max(1);

            for py in y0..y1 {
                for px in x0..x1 {
                    let t = if width > 0 {
                        (px - x0) as f32 / width as f32
                    } else {
                        0.0
                    };
                    let r = (fg_r as f32 + (bg_r as f32 - fg_r as f32) * t) as u8;
                    let g = (fg_g as f32 + (bg_g as f32 - fg_g as f32) * t) as u8;
                    let b = (fg_b as f32 + (bg_b as f32 - fg_b as f32) * t) as u8;
                    layer.buffer.set_pixel(px, py, r, g, b, 255);
                }
            }
        }
    }

    fn apply_shape(&mut self, start: CanvasPoint, end: CanvasPoint) {
        let color = self.fg_color_rgba();
        if let Some(layer) = self.document.active_layer_mut() {
            let x0 = start.x.min(end.x) as u32;
            let y0 = start.y.min(end.y) as u32;
            let x1 = start.x.max(end.x) as u32;
            let y1 = start.y.max(end.y) as u32;
            let w = (x1 - x0).max(1);
            let h = (y1 - y0).max(1);

            match self.shape_type {
                ShapeType::Rectangle => {
                    layer
                        .buffer
                        .draw_rect(x0, y0, w, h, color.0, color.1, color.2, color.3);
                }
                ShapeType::Ellipse => {
                    let cx = (x0 + x1) as f64 / 2.0;
                    let cy = (y0 + y1) as f64 / 2.0;
                    let rx = w as f64 / 2.0;
                    let ry = h as f64 / 2.0;
                    layer
                        .buffer
                        .draw_ellipse(cx, cy, rx, ry, color.0, color.1, color.2, color.3);
                }
                ShapeType::Line => {
                    layer.buffer.draw_line(
                        start.x as i32,
                        start.y as i32,
                        end.x as i32,
                        end.y as i32,
                        color.0,
                        color.1,
                        color.2,
                        color.3,
                        2,
                    );
                }
            }
        }
    }

    pub fn normalized_selection(&self) -> Option<SelectionRect> {
        let selection = self.selection?;
        Some(SelectionRect {
            start: CanvasPoint {
                x: selection.start.x.min(selection.end.x),
                y: selection.start.y.min(selection.end.y),
            },
            end: CanvasPoint {
                x: selection.start.x.max(selection.end.x),
                y: selection.start.y.max(selection.end.y),
            },
        })
    }

    pub fn commit_text_input(&mut self) {
        if self.show_text_input && !self.text_input.trim().is_empty() {
            self.push_history("Text");
            // Rasterize text to active layer buffer
            let color = self.fg_color_rgba();
            if let Some(layer) = self.document.active_layer_mut() {
                layer.buffer.render_text(
                    &self.text_input,
                    self.text_pos.x as u32,
                    self.text_pos.y as u32,
                    color.0,
                    color.1,
                    color.2,
                    16,
                );
            }
            self.add_recent_color(self.fg_color);
            self.status_msg = "Text added".into();
        }
        self.show_text_input = false;
        self.text_input.clear();
    }
}
