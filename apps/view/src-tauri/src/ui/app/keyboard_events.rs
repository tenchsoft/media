// ---------------------------------------------------------------------------
// Text and keyboard event handling
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::{self, ClickAction, FitMode};
use tench_image_runtime::view::util::rotate_image_data;

use super::ViewApp;

impl ViewApp {
    pub(super) fn handle_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        if let TextEvent::Keyboard(e) = event {
            if e.is_pressed && !e.is_repeat {
                self.handle_keypress(e, ctx);
            }
        }
    }

    /// Handle keyboard shortcuts - mirrors the React keyboard handler exactly.
    fn handle_keypress(&mut self, e: &tench_ui::core::events::KeyboardEvent, ctx: &mut EventCtx) {
        // If rename dialog is open, capture all keyboard input for it
        if self.state.show_rename {
            self.handle_rename_keypress(e, ctx);
            return;
        }

        // If URL dialog is open, capture keyboard input for it
        if self.state.show_url_dialog {
            self.handle_url_keypress(e, ctx);
            return;
        }

        let ctrl = e.modifiers.control || e.modifiers.super_key;

        match &e.logical_key {
            // Arrow keys: navigate images
            LogicalKey::Named(NamedKey::ArrowRight) => {
                self.navigate_and_load(true);
                self.prefetch_adjacent();
                ctx.request_paint();
            }
            LogicalKey::Named(NamedKey::ArrowLeft) => {
                self.navigate_and_load(false);
                self.prefetch_adjacent();
                ctx.request_paint();
            }
            // +/= : zoom in
            LogicalKey::Character(ch) if ch == "+" || ch == "=" => {
                self.state.fit_mode = FitMode::Actual;
                self.state.zoom = (self.state.zoom + 0.1).clamp(0.1, 8.0);
                ctx.request_paint();
            }
            // - : zoom out
            LogicalKey::Character(ch) if ch == "-" => {
                self.state.fit_mode = FitMode::Actual;
                self.state.zoom = (self.state.zoom - 0.1).clamp(0.1, 8.0);
                ctx.request_paint();
            }
            // 0: fit mode
            LogicalKey::Character(ch) if ch == "0" => {
                self.state.fit_mode = FitMode::Fit;
                self.state.zoom = 1.0;
                self.state.pan_x = 0.0;
                self.state.pan_y = 0.0;
                ctx.request_paint();
            }
            // 1: actual size
            LogicalKey::Character(ch) if ch == "1" => {
                self.state.fit_mode = FitMode::Actual;
                self.state.zoom = 1.0;
                self.state.pan_x = 0.0;
                self.state.pan_y = 0.0;
                ctx.request_paint();
            }
            // M or I: toggle metadata
            LogicalKey::Character(ch) if ch == "m" || ch == "M" || ch == "i" || ch == "I" => {
                self.state.show_metadata = !self.state.show_metadata;
                ctx.request_paint();
            }
            // E: quick edit
            LogicalKey::Character(ch) if ch == "e" || ch == "E" => {
                self.state.show_quick_edit = true;
                ctx.request_paint();
            }
            // T: toggle thumbnails
            LogicalKey::Character(ch) if ch == "t" || ch == "T" => {
                self.state.show_thumbnails = !self.state.show_thumbnails;
                ctx.request_paint();
            }
            // B: cycle background color
            LogicalKey::Character(ch) if ch == "b" || ch == "B" => {
                self.state.bg_color = self.state.bg_color.cycle();
                ctx.request_paint();
            }
            // R: rotate image 90 degrees clockwise
            LogicalKey::Character(ch) if ch == "r" || ch == "R" => {
                if let Some(ref image_data) = self.state.current_image_data.clone() {
                    if let Some(rotated) = rotate_image_data(image_data, 1) {
                        // Push to history before applying
                        if let Some(ref current) = self.state.current_image_data {
                            self.state.push_edit_history(current.clone(), "rotate");
                        }
                        self.state.current_image_data = Some(rotated);
                        // Update document dimensions
                        if let Some(ref mut doc) = self.state.document {
                            if let Some(ref dims) = doc.dimensions {
                                doc.dimensions = Some(state::ImageDimensions {
                                    width: dims.height,
                                    height: dims.width,
                                });
                            }
                        }
                    }
                }
                ctx.request_paint();
            }
            // F11: fullscreen (handled by window manager)
            LogicalKey::Named(NamedKey::F(11)) => {
                ctx.request_paint();
            }
            // F2: rename file
            LogicalKey::Named(NamedKey::F(2)) if self.state.document.is_some() => {
                if let Some(ref doc) = self.state.document {
                    self.state.rename_original_name = doc.file_name.clone();
                    self.state.rename_input_text = doc.file_name.clone();
                    self.state.show_rename = true;
                }
                ctx.request_paint();
            }
            // Delete: show delete confirm
            LogicalKey::Named(NamedKey::Delete) if self.state.document.is_some() => {
                self.state.show_delete_confirm = true;
                ctx.request_paint();
            }
            // Escape: dismiss all overlays
            LogicalKey::Named(NamedKey::Escape) => {
                self.state.dismiss_all();
                ctx.request_paint();
            }
            // A: toggle AI panel
            LogicalKey::Character(ch) if ch == "a" || ch == "A" => {
                self.state.show_ai = !self.state.show_ai;
                ctx.request_paint();
            }
            // S (without Ctrl): toggle slideshow
            LogicalKey::Character(ch) if (ch == "s" || ch == "S") && !ctrl => {
                self.state.slideshow_playing = !self.state.slideshow_playing;
                ctx.request_paint();
            }
            // D: compare
            LogicalKey::Character(ch)
                if (ch == "d" || ch == "D") && self.state.document.is_some() =>
            {
                self.state.show_compare = true;
                ctx.request_paint();
            }
            // F: toggle filter panel
            LogicalKey::Character(ch) if ch == "f" || ch == "F" => {
                self.state.show_filter = !self.state.show_filter;
                ctx.request_paint();
            }
            // Q: toggle file info
            LogicalKey::Character(ch) if ch == "q" || ch == "Q" => {
                self.state.show_file_info = !self.state.show_file_info;
                ctx.request_paint();
            }
            // ?: help overlay
            LogicalKey::Character(ch) if ch == "?" => {
                self.state.show_help = !self.state.show_help;
                ctx.request_paint();
            }
            // P: toggle animated playing
            LogicalKey::Character(ch) if ch == "p" || ch == "P" => {
                self.state.animated_playing = !self.state.animated_playing;
                ctx.request_paint();
            }
            // Ctrl+O: open file dialog
            LogicalKey::Character(ch) if (ch == "o" || ch == "O") && ctrl => {
                self.open_file_dialog();
            }
            // Ctrl+Z: undo
            LogicalKey::Character(ch)
                if (ch == "z" || ch == "Z") && ctrl && self.state.undo_edit() =>
            {
                ctx.request_paint();
            }
            // Ctrl+Y or Ctrl+Shift+Z: redo
            LogicalKey::Character(ch)
                if (ch == "y" || ch == "Y") && ctrl && self.state.redo_edit() =>
            {
                ctx.request_paint();
            }
            _ => {}
        }
    }

    /// Handle keyboard input for the rename dialog.
    fn handle_rename_keypress(
        &mut self,
        e: &tench_ui::core::events::KeyboardEvent,
        ctx: &mut EventCtx,
    ) {
        match &e.logical_key {
            LogicalKey::Named(NamedKey::Escape) => {
                self.state.show_rename = false;
                ctx.request_paint();
            }
            LogicalKey::Named(NamedKey::Enter) => {
                // Confirm rename
                self.dispatch_click_action(&ClickAction::RenameConfirm, ctx);
            }
            LogicalKey::Named(NamedKey::Backspace) => {
                self.state.rename_input_text.pop();
                ctx.request_paint();
            }
            // Append printable characters.
            LogicalKey::Character(ch) if ch.chars().all(|c| !c.is_control()) => {
                self.state.rename_input_text.push_str(ch);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    /// Handle keyboard input for the URL dialog.
    fn handle_url_keypress(
        &mut self,
        e: &tench_ui::core::events::KeyboardEvent,
        ctx: &mut EventCtx,
    ) {
        match &e.logical_key {
            LogicalKey::Named(NamedKey::Escape) => {
                self.state.show_url_dialog = false;
                ctx.request_paint();
            }
            LogicalKey::Named(NamedKey::Enter) => {
                let url = self.state.url_input_text.clone();
                if !url.is_empty() {
                    self.state.show_url_dialog = false;
                    self.load_image_from_url(&url);
                    ctx.request_paint();
                }
            }
            LogicalKey::Named(NamedKey::Backspace) => {
                self.state.url_input_text.pop();
                ctx.request_paint();
            }
            LogicalKey::Character(ch) if ch.chars().all(|c| !c.is_control()) => {
                self.state.url_input_text.push_str(ch);
                ctx.request_paint();
            }
            _ => {}
        }
    }
}
