mod action_batch_dialogs;
mod action_context;
mod action_dispatch;
mod action_edit_file;
mod action_metadata;
mod action_nav;
mod action_panels;
mod action_tools;
mod ai_actions;
mod automation;
mod automation_extra;
mod automation_ids;
mod automation_nodes;
mod automation_status;
mod batch_actions;
mod dialogs;
mod filter_actions;
mod image_io;
mod keyboard_events;
mod pointer_events;
mod window_events;

use super::{controls, image_stage, overlays, panels, state, tools};
use tench_ui::prelude::*;

use super::state::{BgColor, ViewState};
use tench_ui::core::events::WindowEvent;

use crate::DialogResult;

use super::theme::{NEUTRAL_50, NEUTRAL_700, NEUTRAL_900};

/// The root widget for the View image viewer.
///
/// This widget manages the complete view experience state and delegates
/// painting to the submodules for each UI section. It matches the React
/// `ViewExperience` component structure exactly.
pub struct ViewApp {
    state: ViewState,
    image_cache: ImageCache,
    text_cache: TextCache,
    app_handle: Option<tauri::AppHandle>,
    dialog_rx: Option<std::sync::mpsc::Receiver<DialogResult>>,
    /// Test injection: next file path for open-file/archive dialogs.
    test_next_file: Option<String>,
    /// Test injection: next folder path for open-folder dialogs.
    test_next_folder: Option<String>,
    /// Test injection: save-as path for convert/batch output dialogs.
    test_save_as_path: Option<String>,
    /// Test injection: batch output folder path.
    test_batch_output_path: Option<String>,
}

impl Default for ViewApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewApp {
    pub fn new() -> Self {
        Self {
            state: ViewState::default(),
            image_cache: ImageCache::default_capacity(),
            text_cache: TextCache::new(),
            app_handle: None,
            dialog_rx: None,
            test_next_file: None,
            test_next_folder: None,
            test_save_as_path: None,
            test_batch_output_path: None,
        }
    }

    /// Set the Tauri AppHandle for native dialogs.
    pub fn set_app_handle(&mut self, handle: tauri::AppHandle) {
        self.app_handle = Some(handle);
    }

    /// Set the dialog result receiver.
    pub fn set_dialog_receiver(&mut self, rx: std::sync::mpsc::Receiver<DialogResult>) {
        self.dialog_rx = Some(rx);
    }

    /// Mutable access to the internal ViewState (for testing).
    pub fn state_mut(&mut self) -> &mut state::ViewState {
        &mut self.state
    }

    /// Inject a file path for the next open-file/archive dialog (test only).
    pub fn inject_next_file(&mut self, path: String) {
        self.test_next_file = Some(path);
    }

    /// Inject a folder path for the next open-folder dialog (test only).
    pub fn inject_next_folder(&mut self, path: String) {
        self.test_next_folder = Some(path);
    }

    /// Inject a save-as path for the next save-as dialog (test only).
    pub fn inject_save_as_path(&mut self, path: String) {
        self.test_save_as_path = Some(path);
    }

    /// Inject a batch output folder path (test only).
    pub fn inject_batch_output_path(&mut self, path: String) {
        self.test_batch_output_path = Some(path);
    }

    /// Inject a synthetic image for testing purposes.
    /// Creates a small colored image and sets up folder entries for navigation testing.
    pub fn inject_test_image(&mut self, width: u32, height: u32) {
        use tench_ui::peniko::{ImageAlphaType, ImageData, ImageFormat};

        // Create a simple RGBA image
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        // Fill with a gradient pattern to make it non-blank
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                pixels[idx] = (x % 256) as u8; // R
                pixels[idx + 1] = (y % 256) as u8; // G
                pixels[idx + 2] = 128; // B
                pixels[idx + 3] = 255; // A
            }
        }

        let image_data = ImageData {
            data: pixels.into(),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::AlphaPremultiplied,
            width,
            height,
        };

        self.state.load_image(state::ImageMetadata {
            file_name: "test_image.png".to_string(),
            format: "png".to_string(),
            dimensions: Some(state::ImageDimensions { width, height }),
            file_size: (width * height * 4) as u64,
            path: "/test/test_image.png".to_string(),
        });

        self.state.set_image_data(image_data);

        // Add folder entries for navigation
        self.state.sorted_entries = vec![
            state::FolderEntry {
                id: "1".to_string(),
                path: "/test/img1.png".to_string(),
                file_name: "img1.png".to_string(),
                size_bytes: 1024,
                modified_unix: None,
                is_archive_entry: false,
            },
            state::FolderEntry {
                id: "2".to_string(),
                path: "/test/test_image.png".to_string(),
                file_name: "test_image.png".to_string(),
                size_bytes: 2048,
                modified_unix: None,
                is_archive_entry: false,
            },
            state::FolderEntry {
                id: "3".to_string(),
                path: "/test/img3.png".to_string(),
                file_name: "img3.png".to_string(),
                size_bytes: 3072,
                modified_unix: None,
                is_archive_entry: false,
            },
        ];
    }

    /// Inject synthetic recent file paths for selector-based E2E tests.
    pub fn inject_test_recent_files(&mut self, count: usize) {
        self.state.recent_files = (0..count)
            .map(|idx| format!("/test/recent_{}.png", idx + 1))
            .collect();
    }

    /// Load persisted state (recent files, etc.). Call once after construction.
    pub fn load_persisted_state(&mut self) {
        self.load_recent_files();
    }

    pub fn with_image(path: String) -> Self {
        let mut app = Self::new();
        app.load_image_from_path(&path);
        app
    }
}

impl Widget for ViewApp {
    fn measure(&mut self, _ctx: &mut MeasureCtx, _axis: Axis, available: f64) -> f64 {
        // Fill available space
        available
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _size: Size) {
        // No-op - we fill whatever space we're given
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let size = ctx.size();

        // Process any pending dialog results
        self.process_dialog_results();

        // Clear click regions from the previous frame
        self.state.clear_click_regions();

        // Update the image transform for the current viewport
        self.state.update_img_transform(size);

        //  1. Main background (matches `.view-immersive[data-bg]`)
        paint_background(&self.state, size, scene);

        //  2. Image stage or empty state
        image_stage::paint_image_stage(&mut self.state, &mut self.text_cache, size, scene);

        //  2b. Slideshow transition overlay
        if self.state.slideshow_fade_alpha < 1.0 {
            if let Some(ref _prev_image) = self.state.slideshow_prev_image {
                let doc = self.state.document.as_ref();
                let dims = doc.and_then(|d| d.dimensions);
                let img_rect = image_stage::compute_image_rect(&self.state, size, dims);
                let alpha = 1.0 - self.state.slideshow_fade_alpha;
                let alpha_u8 = (alpha * 255.0).round() as u8;
                if alpha > 0.01 {
                    let mut trans_painter = Painter::new(scene);
                    match self.state.slideshow_transition {
                        state::SlideshowTransition::Fade => {
                            // Cross-fade: dark overlay that gradually becomes transparent
                            let overlay_color = Color::rgba8(0x0F, 0x0F, 0x0F, alpha_u8);
                            trans_painter.fill_rect(img_rect, overlay_color);
                        }
                        state::SlideshowTransition::Slide => {
                            // Slide: a vertical band sweeps from left to right
                            let split_x = img_rect.x0 + img_rect.width() * (1.0 - alpha);
                            let left_rect =
                                Rect::new(img_rect.x0, img_rect.y0, split_x, img_rect.y1);
                            let overlay_color = Color::rgba8(0x0F, 0x0F, 0x0F, 200);
                            trans_painter.fill_rect(left_rect, overlay_color);
                        }
                        state::SlideshowTransition::Dissolve => {
                            // Dissolve: noise-like pattern with random opacity
                            // Draw alternating strips to simulate dissolve
                            let strip_h = 4.0_f64;
                            let mut sy = img_rect.y0;
                            let mut idx = 0u32;
                            while sy < img_rect.y1 {
                                let h = strip_h.min(img_rect.y1 - sy);
                                // Vary opacity per strip using index-based pattern
                                let strip_alpha = if idx.is_multiple_of(3) {
                                    alpha_u8
                                } else if idx % 3 == 1 {
                                    (alpha_u8 as u16 * 60 / 100).min(255) as u8
                                } else {
                                    (alpha_u8 as u16 * 30 / 100).min(255) as u8
                                };
                                let strip_color = Color::rgba8(0x0F, 0x0F, 0x0F, strip_alpha);
                                trans_painter.fill_rect(
                                    Rect::new(img_rect.x0, sy, img_rect.x1, sy + h),
                                    strip_color,
                                );
                                sy += strip_h;
                                idx += 1;
                            }
                        }
                        state::SlideshowTransition::None => {
                            // No transition overlay
                        }
                    }
                }
            }
            // Animation frames are requested from on_window_event
        }

        //  3. Top overlay (toolbar)
        if self.state.show_chrome || self.state.document.is_none() {
            overlays::paint_top_overlay(&mut self.state, &mut self.text_cache, size, scene);
        }

        //  4. Nav edges (prev/next arrows, breadcrumb)
        if self.state.show_chrome {
            overlays::paint_nav_edges(&mut self.state, &mut self.text_cache, size, scene);
        }

        //  5. Bottom overlay (zoom controls, filmstrip)
        if self.state.show_chrome || self.state.document.is_none() {
            // Generate thumbnails for visible filmstrip entries before painting
            if self.state.show_thumbnails && !self.state.sorted_entries.is_empty() {
                let thumb_size = 48.0_f64;
                let thumb_gap = 8.0;
                let item_w = thumb_size + thumb_gap;
                let max_visible = ((size.width - 20.0) / item_w).floor() as usize;
                let count = self.state.sorted_entries.len().min(max_visible);
                let selected = self.state.selected_index().unwrap_or(0);
                // Center the view around the selected item for virtual scrolling
                let half = count / 2;
                let start = selected.saturating_sub(half);
                let end = (start + count).min(self.state.sorted_entries.len());
                let start = end.saturating_sub(count);
                self.generate_visible_thumbnails(start, end);
            }
            overlays::paint_bottom_overlay(&mut self.state, &mut self.text_cache, size, scene);
        }

        //  6. Slideshow controls
        if self.state.slideshow_playing {
            controls::paint_slideshow_controls(&mut self.state, &mut self.text_cache, size, scene);
        }

        //  7. Compare panel
        if self.state.show_compare {
            panels::paint_compare_panel(&mut self.state, &mut self.text_cache, size, scene);
        }

        //  8. Batch panel
        if self.state.show_batch {
            controls::paint_batch_panel(&mut self.state, &mut self.text_cache, size, scene);
        }

        //  9. Filter panel
        if self.state.show_filter {
            panels::paint_filter_panel(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 10. Context menu
        if self.state.show_context_menu {
            controls::paint_context_menu(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 11. File info overlay
        if self.state.show_file_info {
            panels::paint_file_info_overlay(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 12. AI panel
        if self.state.show_ai {
            panels::paint_ai_panel(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 13. Crop tool
        if self.state.active_edit_tool == Some(state::EditTool::Crop) {
            tools::paint_crop_tool(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 14. Resize tool
        if self.state.active_edit_tool == Some(state::EditTool::Resize) {
            tools::paint_resize_tool(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 15. Convert tool
        if self.state.active_edit_tool == Some(state::EditTool::Convert) {
            tools::paint_convert_tool(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 16. Quick edit overlay
        if self.state.show_quick_edit {
            panels::paint_quick_edit_overlay(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 17. Metadata drawer
        if self.state.show_metadata {
            panels::paint_metadata_drawer(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 18. Loading overlay
        if self.state.is_loading {
            controls::paint_loading_overlay(size, scene);
        }

        // 19. Edit banner
        if self.state.has_edited_image {
            controls::paint_edit_banner(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 20. Delete confirm
        if self.state.show_delete_confirm {
            controls::paint_delete_confirm(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 21. Rename dialog
        if self.state.show_rename {
            controls::paint_rename_dialog(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 22. URL dialog
        overlays::paint_url_dialog(&mut self.state, &mut self.text_cache, size, scene);

        // 23. Help overlay
        if self.state.show_help {
            panels::paint_help_overlay(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 24. Print dialog
        overlays::paint_print_dialog(&mut self.state, &mut self.text_cache, size, scene);

        // 25. Annotations overlay
        overlays::paint_annotations_overlay(&mut self.state, &mut self.text_cache, size, scene);

        // 26. Annotation color picker
        overlays::paint_annotation_color_picker(&mut self.state, &mut self.text_cache, size, scene);

        // 27. Empty state (drop zone + recent files)
        overlays::paint_empty_state(&mut self.state, &mut self.text_cache, size, scene);

        // 28. Batch trigger button
        if self.state.sorted_entries.len() > 1 && !self.state.show_batch {
            controls::paint_batch_trigger(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 29. Settings panel
        if self.state.show_settings {
            panels::paint_settings_panel(&mut self.state, &mut self.text_cache, size, scene);
        }

        // 30. Hamburger menu
        if self.state.show_menu {
            controls::paint_hamburger_menu(&mut self.state, &mut self.text_cache, size, scene);
        }
    }
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        self.handle_pointer_event(ctx, event);
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        self.handle_text_event(ctx, event);
    }

    fn on_window_event(&mut self, ctx: &mut EventCtx, event: &WindowEvent) {
        self.handle_window_event(ctx, event);
    }

    fn debug_id(&self) -> Option<&str> {
        Some("view.root")
    }

    fn accessibility_tree(
        &self,
        state: &tench_ui::core::widget::WidgetState,
    ) -> tench_ui::core::widget::AccessibilityNode {
        tench_ui::core::widget::AccessibilityNode {
            role: tench_ui::core::widget::AccessRole::Window,
            label: Some("Tench View".to_string()),
            value: None,
            focused: state.is_focused,
            disabled: state.is_disabled,
            children: Vec::new(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn automation_children(
        &self,
        state: &tench_ui::core::widget::WidgetState,
    ) -> Vec<tench_ui::UiAutomationNode> {
        self.automation_children_nodes(state)
    }
}

/// Paint the main background based on the current bg_color setting.
/// Matches `.view-immersive[data-bg]` CSS rules.
fn paint_background(state: &ViewState, size: Size, scene: &mut Scene) {
    let mut painter = Painter::new(scene);
    let bg = match state.bg_color {
        BgColor::Black => NEUTRAL_900,
        BgColor::Gray => NEUTRAL_700,
        BgColor::White => NEUTRAL_50,
    };
    painter.fill_background(size, bg);
}
