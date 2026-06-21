//! Pixel Design UI: native image editor shell.

pub mod canvas;
mod color_picker;
pub mod layers;
pub mod panels;
pub mod state;
pub mod theme;
pub mod toolbar;
mod widget;

use canvas::{canvas_document_rect, paint_canvas_viewport};
use layers::paint_edit_panel;
use panels::{paint_adjust_panel, paint_ai_panel, paint_export_panel};
use state::{AiTool, PanelTab, Persona, PixelDesignState, Tool};
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use toolbar::{paint_tool_strip, paint_top_bar, COLOR_BG_Y, COLOR_FG_Y, RECENT_COLOR_Y};

const TOP_BAR_H: f64 = 48.0;
const TOOL_STRIP_W: f64 = 48.0;
const RIGHT_PANEL_W: f64 = 270.0;
const STATUS_BAR_H: f64 = 28.0;

/// Root widget for the Pixel Design image editor.
pub struct PixelDesignApp {
    state: PixelDesignState,
}

impl PixelDesignApp {
    // new_without_default: PixelDesignApp wraps PixelDesignState which has no Default
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut app = Self {
            state: PixelDesignState::new(),
        };
        app.refresh_flattened();
        app
    }

    /// Loads an image file into the editor.
    pub fn load_image(&mut self, path: &str) {
        match tench_image_runtime::pixel::load_document(path) {
            Ok(document) => {
                self.state.document = document;
                self.state.status_msg = format!("Loaded: {}", self.state.document.name);
                self.refresh_flattened();
            }
            Err(e) => {
                self.state.status_msg = format!("Failed to load: {e}");
            }
        }
    }

    /// Refreshes the flattened image cache.
    fn refresh_flattened(&mut self) {
        let flat = self.state.document.flatten();
        self.state.composited_image = Some(flat);
        self.state.update_all_thumbnails();
    }

    /// Saves the current document.
    pub fn save_document(&mut self) {
        let path = self.state.document.file_path.clone();
        if let Some(ref path) = path {
            self.save_to_path(path);
        } else {
            self.state.status_msg = "No file path set. Use Save As.".into();
        }
    }

    /// Saves the document to a specific path.
    pub fn save_to_path(&mut self, path: &str) {
        match tench_image_runtime::pixel::save_document_image(&self.state.document, path) {
            Ok(name) => {
                self.state.document.file_path = Some(path.to_string());
                self.state.document.dirty = false;
                self.state.document.name = name;
                self.state.status_msg = format!("Saved: {}", self.state.document.name);
            }
            Err(e) => {
                self.state.status_msg = format!("Save failed: {e}");
            }
        }
    }

    /// Exports the current document as an image file.
    pub fn export_document(&mut self, path: &str) {
        let flat = self.state.document.flatten();
        let scale = self.state.export_scale as f64 / 100.0;
        let out_w = (flat.width as f64 * scale) as u32;
        let out_h = (flat.height as f64 * scale) as u32;

        // Resize if needed
        let export_img = if scale != 1.0 {
            let img = flat.to_dynamic_image();
            let resized = img.resize_exact(out_w, out_h, image::imageops::FilterType::Lanczos3);
            tench_pixel_core::PixelBuffer::from_dynamic_image(&resized)
        } else {
            flat
        };

        let image = export_img.to_dynamic_image();
        match image.save(path) {
            Ok(_) => {
                self.state.status_msg = format!("Exported: {}", path);
            }
            Err(e) => {
                self.state.status_msg = format!("Export failed: {e}");
            }
        }
    }

    /// Returns a mutable reference to the internal state (for testing).
    pub fn state_mut(&mut self) -> &mut PixelDesignState {
        &mut self.state
    }

    fn color_picker_modal(size: Size) -> Rect {
        let width = 340.0;
        let height = 260.0;
        let x0 = (size.width - width) * 0.5;
        let y0 = (size.height - height) * 0.5;
        Rect::new(x0, y0, x0 + width, y0 + height)
    }

    fn color_picker_hue_rect(modal: Rect) -> Rect {
        Rect::new(
            modal.x0 + 28.0,
            modal.y0 + 82.0,
            modal.x1 - 28.0,
            modal.y0 + 106.0,
        )
    }

    fn color_picker_sv_rect(modal: Rect) -> Rect {
        Rect::new(
            modal.x0 + 28.0,
            modal.y0 + 122.0,
            modal.x1 - 28.0,
            modal.y0 + 174.0,
        )
    }

    fn color_picker_apply_rect(modal: Rect) -> Rect {
        Rect::new(
            modal.x1 - 154.0,
            modal.y1 - 52.0,
            modal.x1 - 28.0,
            modal.y1 - 18.0,
        )
    }

    fn color_picker_cancel_rect(modal: Rect) -> Rect {
        Rect::new(
            modal.x0 + 28.0,
            modal.y1 - 52.0,
            modal.x0 + 142.0,
            modal.y1 - 18.0,
        )
    }

    fn update_picker_from_point(&mut self, point: Point, modal: Rect) -> bool {
        let hue = Self::color_picker_hue_rect(modal);
        if hue.contains(point) {
            let h = ((point.x - hue.x0) / hue.width()).clamp(0.0, 1.0) as f32 * 360.0;
            self.state
                .set_color_picker_hsv(h, self.state.color_saturation, self.state.color_value);
            return true;
        }

        let sv = Self::color_picker_sv_rect(modal);
        if sv.contains(point) {
            let s = ((point.x - sv.x0) / sv.width()).clamp(0.0, 1.0) as f32;
            let v = (1.0 - ((point.y - sv.y0) / sv.height()).clamp(0.0, 1.0)) as f32;
            self.state.set_color_picker_hsv(self.state.color_hue, s, v);
            return true;
        }

        false
    }

    fn handle_color_picker_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) -> bool {
        if !self.state.show_color_picker {
            return false;
        }

        let modal = Self::color_picker_modal(ctx.state.size);
        match event {
            PointerEvent::Down(e) => {
                if Self::color_picker_apply_rect(modal).contains(e.pos) {
                    self.state.apply_color_picker();
                    return true;
                }
                if Self::color_picker_cancel_rect(modal).contains(e.pos) {
                    self.state.cancel_color_picker();
                    return true;
                }
                self.update_picker_from_point(e.pos, modal) || modal.contains(e.pos)
            }
            PointerEvent::Move(e) => self.update_picker_from_point(e.pos, modal),
            PointerEvent::Up(e) => modal.contains(e.pos),
            _ => false,
        }
    }

    fn handle_top_bar_click(&mut self, x: f64, y: f64, width: f64) -> bool {
        if y >= TOP_BAR_H {
            return false;
        }

        let personas = Persona::ALL;
        for (idx, persona) in personas.iter().enumerate() {
            let rect = Rect::new(
                12.0 + idx as f64 * 70.0,
                8.0,
                76.0 + idx as f64 * 70.0,
                40.0,
            );
            if rect.contains(Point::new(x, y)) {
                self.state.select_persona(*persona);
                return true;
            }
        }

        let action_x = width - 148.0;
        if x >= action_x && x <= width - 8.0 {
            let idx = ((x - action_x) / 34.0).floor() as usize;
            match idx {
                0 => {
                    self.state.undo();
                    self.refresh_flattened();
                }
                1 => {
                    self.state.redo();
                    self.refresh_flattened();
                }
                2 => {
                    self.state.status_msg = "Open image file".into();
                    self.state.pending_file_action = Some(state::FileAction::Open);
                }
                3 => self.save_document(),
                _ => return false,
            }
            return true;
        }

        false
    }

    fn handle_tool_strip_click(&mut self, x: f64, y: f64) -> bool {
        if x >= TOOL_STRIP_W || y < TOP_BAR_H {
            return false;
        }

        let local_y = y - TOP_BAR_H;
        if self.state.persona == Persona::Edit {
            for (idx, tool) in Tool::ALL.iter().enumerate() {
                let rect = Rect::new(6.0, 8.0 + idx as f64 * 42.0, 42.0, 44.0 + idx as f64 * 42.0);
                if rect.contains(Point::new(x, local_y)) {
                    self.state.set_active_tool(*tool);
                    return true;
                }
            }
        } else if self.state.persona == Persona::AI {
            for (idx, tool) in AiTool::ALL.iter().enumerate() {
                let rect = Rect::new(6.0, 8.0 + idx as f64 * 46.0, 42.0, 44.0 + idx as f64 * 46.0);
                if rect.contains(Point::new(x, local_y)) {
                    self.state.expanded_ai = *tool;
                    self.state.status_msg = format!("AI tool: {}", tool.label());
                    return true;
                }
            }
        }

        let fg = Rect::new(8.0, COLOR_FG_Y, 36.0, COLOR_FG_Y + 28.0);
        let bg = Rect::new(16.0, COLOR_BG_Y, 44.0, COLOR_BG_Y + 28.0);
        if fg.contains(Point::new(x, local_y)) {
            self.state.open_color_picker(true);
            return true;
        }
        if bg.contains(Point::new(x, local_y)) {
            self.state.open_color_picker(false);
            return true;
        }

        let recent = Rect::new(4.0, RECENT_COLOR_Y, 46.0, RECENT_COLOR_Y + 26.0);
        if recent.contains(Point::new(x, local_y)) {
            let col = ((x - 4.0) / 14.0).floor().clamp(0.0, 2.0) as usize;
            let row = ((local_y - RECENT_COLOR_Y) / 14.0).floor().clamp(0.0, 1.0) as usize;
            let idx = row * 3 + col;
            if let Some(color) = self.state.recent_colors.get(idx).copied() {
                self.state.fg_color = color;
                self.state.status_msg = format!("Recent color {}", idx + 1);
                return true;
            }
        }

        false
    }

    fn handle_right_panel_click(&mut self, x: f64, y: f64, width: f64) -> bool {
        let panel_x = width - RIGHT_PANEL_W;
        if x < panel_x || y < TOP_BAR_H {
            return false;
        }

        let local_x = x - panel_x;
        let local_y = y - TOP_BAR_H;
        match self.state.persona {
            Persona::Edit => self.handle_edit_panel_click(local_x, local_y),
            Persona::AI => self.handle_ai_panel_click(local_x, local_y),
            Persona::Adjust => self.handle_adjust_panel_click(local_x, local_y),
            Persona::Export => self.handle_export_panel_click(local_x, local_y),
        }
    }

    fn handle_edit_panel_click(&mut self, x: f64, y: f64) -> bool {
        for (idx, tab) in PanelTab::ALL.iter().enumerate() {
            let rect = Rect::new(
                10.0 + idx as f64 * 88.0,
                10.0,
                92.0 + idx as f64 * 88.0,
                40.0,
            );
            if rect.contains(Point::new(x, y)) {
                self.state.panel_tab = *tab;
                return true;
            }
        }

        match self.state.panel_tab {
            PanelTab::Layers => {
                // Opacity slider area
                if Rect::new(16.0, 92.0, 268.0, 120.0).contains(Point::new(x, y)) {
                    self.state
                        .nudge_active_layer_opacity(if x < 142.0 { -5 } else { 5 });
                    self.refresh_flattened();
                    return true;
                }

                // Layer rows
                for idx in 0..self.state.document.layers.len() {
                    let row = Rect::new(
                        14.0,
                        134.0 + idx as f64 * 42.0,
                        272.0,
                        170.0 + idx as f64 * 42.0,
                    );
                    if row.contains(Point::new(x, y)) {
                        if x < 46.0 {
                            // Visibility toggle
                            self.state.toggle_layer_visibility(idx);
                        } else if x > 240.0 && y > 154.0 + idx as f64 * 42.0 {
                            // Lock toggle (right side, lower portion)
                            self.state.toggle_layer_lock(idx);
                        } else {
                            // Select layer
                            let id = self.state.document.layers[idx].id.clone();
                            self.state.document.active_layer_id = id;
                        }
                        self.refresh_flattened();
                        return true;
                    }
                }

                // Add/Delete layer buttons
                if Rect::new(16.0, 262.0, 132.0, 294.0).contains(Point::new(x, y)) {
                    self.state.add_layer();
                    self.refresh_flattened();
                    return true;
                }
                if Rect::new(146.0, 262.0, 270.0, 294.0).contains(Point::new(x, y)) {
                    self.state.delete_layer();
                    self.refresh_flattened();
                    return true;
                }

                // Layer reorder buttons (up/down arrows below delete)
                let reorder_y = 300.0;
                if Rect::new(16.0, reorder_y, 130.0, reorder_y + 28.0).contains(Point::new(x, y)) {
                    self.state.move_layer_up(self.state.active_layer_index());
                    self.refresh_flattened();
                    return true;
                }
                if Rect::new(146.0, reorder_y, 270.0, reorder_y + 28.0).contains(Point::new(x, y)) {
                    self.state.move_layer_down(self.state.active_layer_index());
                    self.refresh_flattened();
                    return true;
                }

                // Layer context menu (Duplicate/Merge/Flatten)
                let ctx_y = 336.0;
                if Rect::new(16.0, ctx_y, 130.0, ctx_y + 28.0).contains(Point::new(x, y)) {
                    self.state.duplicate_layer(self.state.active_layer_index());
                    self.refresh_flattened();
                    return true;
                }
                if Rect::new(146.0, ctx_y, 270.0, ctx_y + 28.0).contains(Point::new(x, y)) {
                    self.state.flatten_layers();
                    self.refresh_flattened();
                    return true;
                }
            }
            PanelTab::Properties => {
                if Rect::new(18.0, 92.0, 268.0, 122.0).contains(Point::new(x, y)) {
                    self.state.brush_size = if x < 142.0 {
                        self.state.brush_size.saturating_sub(4).max(1)
                    } else {
                        (self.state.brush_size + 4).min(200)
                    };
                    return true;
                }
                if Rect::new(18.0, 140.0, 268.0, 170.0).contains(Point::new(x, y)) {
                    self.state.brush_opacity = if x < 142.0 {
                        self.state.brush_opacity.saturating_sub(5).max(1)
                    } else {
                        (self.state.brush_opacity + 5).min(100)
                    };
                    return true;
                }
                // Hardness
                if Rect::new(18.0, 188.0, 268.0, 218.0).contains(Point::new(x, y)) {
                    self.state.brush_hardness = if x < 142.0 {
                        self.state.brush_hardness.saturating_sub(5)
                    } else {
                        (self.state.brush_hardness + 5).min(100)
                    };
                    return true;
                }
            }
            PanelTab::History => {
                if Rect::new(20.0, 92.0, 130.0, 124.0).contains(Point::new(x, y)) {
                    self.state.undo();
                    self.refresh_flattened();
                    return true;
                }
                if Rect::new(146.0, 92.0, 256.0, 124.0).contains(Point::new(x, y)) {
                    self.state.redo();
                    self.refresh_flattened();
                    return true;
                }
                // History step click to jump
                for idx in 0..self.state.history.len().min(8) {
                    let step_y = 152.0 + idx as f64 * 20.0;
                    if Rect::new(20.0, step_y - 12.0, 200.0, step_y + 4.0)
                        .contains(Point::new(x, y))
                    {
                        self.state.history_index = idx;
                        let snapshot = self.state.history[idx].clone();
                        self.state.document = snapshot.document;
                        self.refresh_flattened();
                        return true;
                    }
                }
            }
        }

        if self.state.active_tool.uses_brush() {
            for (idx, preset) in PixelDesignState::brush_presets().iter().enumerate() {
                let col = idx % 2;
                let row = idx / 2;
                let rect = Rect::new(
                    16.0 + col as f64 * 128.0,
                    370.0 + row as f64 * 58.0,
                    132.0 + col as f64 * 128.0,
                    420.0 + row as f64 * 58.0,
                );
                if rect.contains(Point::new(x, y)) {
                    self.state.set_brush_preset(preset.id);
                    return true;
                }
            }
        }

        false
    }

    fn handle_ai_panel_click(&mut self, x: f64, y: f64) -> bool {
        // Run AI Job button
        if Rect::new(18.0, 160.0, 268.0, 196.0).contains(Point::new(x, y)) {
            self.state.run_ai_job();
            return true;
        }

        // Cancel button (next to Run)
        if Rect::new(18.0, 202.0, 268.0, 230.0).contains(Point::new(x, y)) {
            self.state.cancel_ai_job();
            return true;
        }

        // AI prompt area click — focus prompt
        if Rect::new(18.0, 94.0, 268.0, 148.0).contains(Point::new(x, y)) {
            self.state.ai_prompt_focused = true;
            return true;
        }

        for (idx, tool) in AiTool::ALL.iter().enumerate() {
            let rect = Rect::new(
                18.0,
                262.0 + idx as f64 * 34.0,
                268.0,
                290.0 + idx as f64 * 34.0,
            );
            if rect.contains(Point::new(x, y)) {
                self.state.expanded_ai = *tool;
                return true;
            }
        }

        false
    }

    fn handle_adjust_panel_click(&mut self, x: f64, y: f64) -> bool {
        // Preset buttons
        for (idx, preset) in PixelDesignState::adjust_presets().iter().enumerate() {
            let col = idx % 2;
            let row = idx / 2;
            let rect = Rect::new(
                18.0 + col as f64 * 124.0,
                60.0 + row as f64 * 38.0,
                132.0 + col as f64 * 124.0,
                90.0 + row as f64 * 38.0,
            );
            if rect.contains(Point::new(x, y)) {
                self.state.active_adjust = if self.state.active_adjust.as_deref() == Some(*preset) {
                    None
                } else {
                    Some((*preset).into())
                };
                // Apply real filter for preset
                self.state.apply_adjust_preset(preset);
                self.refresh_flattened();
                return true;
            }
        }

        // Adjust sliders (now 8 rows)
        let adjust_start_y = 260.0;
        if Rect::new(20.0, adjust_start_y, 266.0, adjust_start_y + 8.0 * 40.0)
            .contains(Point::new(x, y))
        {
            let idx = ((y - adjust_start_y) / 40.0).floor() as usize;
            if idx < 8 {
                let delta = if x < 143.0 { -5 } else { 5 };
                self.state.adjust_values.nudge(idx, delta);
                self.state.apply_adjust_filter(idx);
                self.refresh_flattened();
            }
            return true;
        }

        false
    }

    fn handle_export_panel_click(&mut self, x: f64, y: f64) -> bool {
        if Rect::new(18.0, 58.0, 268.0, 90.0).contains(Point::new(x, y)) {
            self.state.cycle_export_format();
            return true;
        }
        if Rect::new(18.0, 116.0, 268.0, 146.0).contains(Point::new(x, y)) {
            self.state.export_quality = if x < 143.0 {
                self.state.export_quality.saturating_sub(5).max(1)
            } else {
                (self.state.export_quality + 5).min(100)
            };
            return true;
        }
        if Rect::new(18.0, 174.0, 268.0, 204.0).contains(Point::new(x, y)) {
            self.state.export_scale = if x < 143.0 {
                self.state.export_scale.saturating_sub(10).max(10)
            } else {
                (self.state.export_scale + 10).min(400)
            };
            return true;
        }
        // Export button — creates actual file
        if Rect::new(18.0, 310.0, 268.0, 348.0).contains(Point::new(x, y)) {
            self.state.pending_file_action = Some(state::FileAction::Export);
            self.state.status_msg =
                format!("Export {} — choose destination", self.state.export_format);
            return true;
        }

        false
    }

    fn handle_status_bar_click(&mut self, x: f64, y: f64, width: f64, height: f64) -> bool {
        let status_y = height - STATUS_BAR_H;
        let status_rect = Rect::new(
            TOOL_STRIP_W,
            status_y,
            width - RIGHT_PANEL_W,
            status_y + STATUS_BAR_H,
        );
        if !status_rect.contains(Point::new(x, y)) {
            return false;
        }

        let zoom_x = width - RIGHT_PANEL_W - 142.0;
        // Zoom out button
        if Rect::new(zoom_x, status_y, zoom_x + 28.0, status_y + STATUS_BAR_H)
            .contains(Point::new(x, y))
        {
            self.state.zoom_out();
            return true;
        }
        // Zoom in button
        if Rect::new(
            zoom_x + 100.0,
            status_y,
            zoom_x + 128.0,
            status_y + STATUS_BAR_H,
        )
        .contains(Point::new(x, y))
        {
            self.state.zoom_in();
            return true;
        }
        // Zoom slider area (click to set zoom)
        if Rect::new(
            zoom_x + 30.0,
            status_y,
            zoom_x + 96.0,
            status_y + STATUS_BAR_H,
        )
        .contains(Point::new(x, y))
        {
            let t = (x - (zoom_x + 30.0)) / 66.0;
            self.state.zoom = (t * 3200.0).clamp(10.0, 3200.0) as u32;
            return true;
        }

        false
    }

    fn handle_canvas_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) -> bool {
        let size = ctx.state.size;
        let viewport = Rect::new(
            TOOL_STRIP_W,
            TOP_BAR_H,
            size.width - RIGHT_PANEL_W,
            size.height - STATUS_BAR_H,
        );
        let doc = canvas_document_rect(&self.state, viewport);

        match event {
            PointerEvent::Down(e) => {
                if !doc.contains(e.pos) {
                    return false;
                }
                let x = ((e.pos.x - doc.x0) / doc.width() * self.state.document.width as f64)
                    .clamp(0.0, self.state.document.width as f64);
                let y = ((e.pos.y - doc.y0) / doc.height() * self.state.document.height as f64)
                    .clamp(0.0, self.state.document.height as f64);
                self.state.begin_canvas_action(x, y);
                true
            }
            PointerEvent::Move(e) => {
                if !viewport.contains(e.pos) {
                    return false;
                }
                let x = ((e.pos.x - doc.x0) / doc.width() * self.state.document.width as f64)
                    .clamp(0.0, self.state.document.width as f64);
                let y = ((e.pos.y - doc.y0) / doc.height() * self.state.document.height as f64)
                    .clamp(0.0, self.state.document.height as f64);
                self.state.move_canvas_action(x, y);
                true
            }
            PointerEvent::Up(e) => {
                if !viewport.contains(e.pos) && !self.state.is_drawing && !self.state.is_panning {
                    return false;
                }
                self.state.finish_canvas_action();
                self.refresh_flattened();
                true
            }
            PointerEvent::Scroll(e) => {
                if !viewport.contains(e.pos) {
                    return false;
                }
                if e.delta.y < 0.0 {
                    self.state.zoom_in();
                } else {
                    self.state.zoom_out();
                }
                true
            }
            PointerEvent::Leave => {
                if self.state.is_drawing || self.state.is_panning {
                    self.state.finish_canvas_action();
                    self.refresh_flattened();
                    return true;
                }
                false
            }
            PointerEvent::Enter => false,
        }
    }
}
