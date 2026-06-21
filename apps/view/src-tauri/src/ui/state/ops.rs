use super::*;

impl ViewState {
    /// Loads image metadata into the viewer and resets transient view state.
    pub fn load_image(&mut self, metadata: ImageMetadata) {
        self.document = Some(metadata);
        self.reset_for_new_image();
        self.status_message = "Loaded".into();
    }

    /// Loads decoded image data into the viewer state.
    pub fn set_image_data(&mut self, data: peniko::ImageData) {
        // Preserve original for compare/discard
        if !self.has_edited_image {
            self.original_image_data = Some(data.clone());
        }
        self.current_image_data = Some(data);
        // Update resize dimensions from current image data
        if let Some(ref img) = self.current_image_data {
            self.resize_width = img.width;
            self.resize_height = img.height;
            self.resize_orig_width = img.width;
            self.resize_orig_height = img.height;
        }
    }

    /// Pushes an edit onto the history stack.
    pub fn push_edit_history(&mut self, data: peniko::ImageData, label: &str) {
        // Truncate any redo entries beyond current index
        self.edit_history.truncate(self.edit_history_index);
        self.edit_history.push(EditHistoryEntry {
            image_data: data,
            label: label.to_string(),
        });
        self.edit_history_index = self.edit_history.len();
        self.has_edited_image = true;
    }

    /// Undoes the last edit.
    pub fn undo_edit(&mut self) -> bool {
        if self.edit_history_index == 0 {
            return false;
        }
        self.edit_history_index -= 1;
        if let Some(entry) = self.edit_history.get(self.edit_history_index) {
            self.current_image_data = Some(entry.image_data.clone());
        }
        true
    }

    /// Redoes the next edit.
    pub fn redo_edit(&mut self) -> bool {
        if self.edit_history_index >= self.edit_history.len() {
            return false;
        }
        self.edit_history_index += 1;
        if let Some(entry) = self.edit_history.get(self.edit_history_index) {
            self.current_image_data = Some(entry.image_data.clone());
        }
        true
    }

    /// Replaces the current folder entries and applies the active sort order.
    pub fn set_folder_entries(&mut self, entries: Vec<FolderEntry>) {
        self.folder_entries = entries;
        self.sort_entries();
    }

    /// Returns the currently selected index in the sorted entries.
    pub fn selected_index(&self) -> Option<usize> {
        let current_path = self.document.as_ref().map(|d| &d.path)?;
        self.sorted_entries
            .iter()
            .position(|e| e.path == *current_path)
    }

    /// Resets filters to defaults.
    pub fn reset_filters(&mut self) {
        self.filter_brightness = 100.0;
        self.filter_contrast = 100.0;
        self.filter_saturation = 100.0;
        self.filter_blur = 0.0;
        self.filter_hue_rotate = 0.0;
    }

    /// Resets view state for a new image.
    pub fn reset_for_new_image(&mut self) {
        self.fit_mode = FitMode::Fit;
        self.zoom = 1.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
        self.rotation = 0;
        self.show_chrome = false;
        self.has_edited_image = false;
        self.active_edit_tool = None;
        self.current_image_data = None;
        self.original_image_data = None;
        self.exif_tags = Vec::new();
        self.histogram = None;
        self.edit_history.clear();
        self.edit_history_index = 0;
        self.img_transform = kurbo::Affine::IDENTITY;
        self.user_transform = kurbo::Affine::IDENTITY;
        self.reset_filters();
        self.crop_start = None;
        self.crop_selection = None;
        self.crop_dragging = false;
        self.filter_dragging = None;
        self.filter_dirty = false;
    }

    /// Dismisses all overlays (Escape key).
    pub fn dismiss_all(&mut self) {
        self.show_metadata = false;
        self.show_quick_edit = false;
        self.show_delete_confirm = false;
        self.show_compare = false;
        self.show_batch = false;
        self.show_ai = false;
        self.show_filter = false;
        self.show_context_menu = false;
        self.show_file_info = false;
        self.show_rename = false;
        self.show_url_dialog = false;
        self.show_print_dialog = false;
        self.show_help = false;
        self.show_settings = false;
        self.show_menu = false;
        self.active_edit_tool = None;
        self.slideshow_playing = false;
        self.slideshow_timer = None;
        self.slideshow_prev_image = None;
        self.slideshow_fade_alpha = 1.0;
        self.slideshow_fade_timer = None;
    }

    /// Sorts folder entries according to current sort settings.
    pub fn sort_entries(&mut self) {
        let mut entries = self.folder_entries.clone();
        let dir: i32 = if self.sort_order == SortOrder::Asc {
            1
        } else {
            -1
        };
        entries.sort_by(|a, b| {
            let cmp = match self.sort_key {
                SortKey::Name => a.file_name.cmp(&b.file_name),
                SortKey::Modified => {
                    let ta = a.modified_unix.unwrap_or(0);
                    let tb = b.modified_unix.unwrap_or(0);
                    ta.cmp(&tb)
                }
                SortKey::Size => a.size_bytes.cmp(&b.size_bytes),
            };
            if dir == -1 {
                cmp.reverse()
            } else {
                cmp
            }
        });
        self.sorted_entries = entries;
    }

    /// Navigates to the next image entry when possible.
    pub fn navigate_next(&mut self) -> bool {
        let Some(idx) = self.selected_index() else {
            return false;
        };
        let Some(next_entry) = self.sorted_entries.get(idx + 1).cloned() else {
            return false;
        };
        self.load_image(metadata_from_entry(&next_entry));
        true
    }

    /// Navigates to the previous image entry when possible.
    pub fn navigate_prev(&mut self) -> bool {
        let Some(idx) = self.selected_index() else {
            return false;
        };
        if idx == 0 {
            return false;
        }
        let Some(prev_entry) = self.sorted_entries.get(idx - 1).cloned() else {
            return false;
        };
        self.load_image(metadata_from_entry(&prev_entry));
        true
    }

    /// Computes the combined transform for rendering (img * user).
    pub fn combined_transform(&self) -> kurbo::Affine {
        self.img_transform * self.user_transform
    }

    /// Computes the effective zoom level from the dual transform.
    pub fn effective_zoom(&self) -> f64 {
        let coeffs = self.combined_transform().as_coeffs();
        coeffs[0].abs()
    }

    /// Updates the image→viewport fit transform.
    pub fn update_img_transform(&mut self, viewport: Size) {
        let Some(ref doc) = self.document else {
            self.img_transform = kurbo::Affine::IDENTITY;
            return;
        };
        let Some(dims) = doc.dimensions else {
            self.img_transform = kurbo::Affine::IDENTITY;
            return;
        };

        let nat_w = dims.width as f64;
        let nat_h = dims.height as f64;

        match self.fit_mode {
            FitMode::Fit => {
                let scale_x = viewport.width / nat_w;
                let scale_y = viewport.height / nat_h;
                let scale = scale_x.min(scale_y).min(1.0);
                let w = nat_w * scale;
                let h = nat_h * scale;
                let tx = (viewport.width - w) / 2.0;
                let ty = (viewport.height - h) / 2.0;
                self.img_transform =
                    kurbo::Affine::translate((tx, ty)) * kurbo::Affine::scale(scale);
                self.user_transform = kurbo::Affine::IDENTITY;
            }
            FitMode::Actual => {
                let scale = self.zoom;
                let w = nat_w * scale;
                let h = nat_h * scale;
                let cx = viewport.width / 2.0 + self.pan_x;
                let cy = viewport.height / 2.0 + self.pan_y;
                let tx = cx - w / 2.0;
                let ty = cy - h / 2.0;
                self.img_transform =
                    kurbo::Affine::translate((tx, ty)) * kurbo::Affine::scale(scale);
                self.user_transform = kurbo::Affine::IDENTITY;
            }
        }
    }

    /// Clamps the pan so the image stays visible (nomacs controlImagePosition).
    pub fn clamp_pan(&mut self, viewport: Size) {
        if self.fit_mode != FitMode::Actual {
            return;
        }
        let Some(ref doc) = self.document else {
            return;
        };
        let Some(dims) = doc.dimensions else {
            return;
        };

        let img_w = dims.width as f64 * self.zoom;
        let img_h = dims.height as f64 * self.zoom;

        let margin_x = viewport.width / 2.0;
        let margin_y = viewport.height / 2.0;

        if img_w <= viewport.width {
            self.pan_x = self.pan_x.clamp(
                -(viewport.width - img_w) / 2.0,
                (viewport.width - img_w) / 2.0,
            );
        } else {
            self.pan_x = self
                .pan_x
                .clamp(-(img_w - viewport.width + margin_x), margin_x);
        }
        if img_h <= viewport.height {
            self.pan_y = self.pan_y.clamp(
                -(viewport.height - img_h) / 2.0,
                (viewport.height - img_h) / 2.0,
            );
        } else {
            self.pan_y = self
                .pan_y
                .clamp(-(img_h - viewport.height + margin_y), margin_y);
        }
    }

    /// Detects if a pointer event at (x,y) is a double-click.
    pub fn is_double_click(&mut self, x: f64, y: f64) -> bool {
        let now = std::time::Instant::now();
        let threshold_dist = 6.0;
        let threshold_time = std::time::Duration::from_millis(400);

        if let (Some(last_time), Some((lx, ly))) = (self.last_click_time, self.last_click_pos) {
            let dist = ((x - lx).powi(2) + (y - ly).powi(2)).sqrt();
            if now.duration_since(last_time) < threshold_time && dist < threshold_dist {
                self.last_click_time = None;
                self.last_click_pos = None;
                return true;
            }
        }

        self.last_click_time = Some(now);
        self.last_click_pos = Some((x, y));
        false
    }

    /// Clears all registered click regions (call at start of paint).
    pub fn clear_click_regions(&mut self) {
        self.click_regions.clear();
    }

    /// Registers a clickable region with the given action.
    pub fn register_click(&mut self, rect: Rect, action: ClickAction) {
        self.click_regions.push(ClickRegion { rect, action });
    }

    /// Finds the action for a click at the given position, if any.
    /// Returns the last (topmost-painted) matching region.
    pub fn click_action_at(&self, x: f64, y: f64) -> Option<&ClickAction> {
        // Iterate in reverse to find the topmost (last-painted) region first.
        for region in self.click_regions.iter().rev() {
            if region.rect.contains(kurbo::Point::new(x, y)) {
                return Some(&region.action);
            }
        }
        None
    }

    /// Compute the click action at (x, y) without relying on paint-time
    /// click_regions.  This is used when the widget receives a pointer event
    /// before the first paint (e.g. in headless E2E tests).
    pub fn click_action_at_computed(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Option<ClickAction> {
        use super::super::overlays::{
            bottom_overlay_button_rects, nav_edge_button_rects, top_overlay_button_rects,
        };

        // Top overlay
        for (action, rect) in top_overlay_button_rects(self, width) {
            if rect.contains(kurbo::Point::new(x, y)) {
                return Some(action);
            }
        }

        // Bottom overlay
        for (action, rect) in bottom_overlay_button_rects(self, kurbo::Size::new(width, height)) {
            if rect.contains(kurbo::Point::new(x, y)) {
                return Some(action);
            }
        }

        // Navigation
        for (action, rect) in nav_edge_button_rects(self, kurbo::Size::new(width, height)) {
            if rect.contains(kurbo::Point::new(x, y)) {
                return Some(action);
            }
        }

        None
    }
}

fn metadata_from_entry(entry: &FolderEntry) -> ImageMetadata {
    let format = entry
        .file_name
        .rsplit('.')
        .next()
        .unwrap_or("png")
        .to_lowercase();

    ImageMetadata {
        file_name: entry.file_name.clone(),
        format,
        dimensions: None,
        file_size: entry.size_bytes,
        path: entry.path.clone(),
    }
}

/// Format a byte count as a human-readable string.
pub fn bytes_label(bytes: u64) -> String {
    if bytes > 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes > 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

/// Format dimensions as a label.
pub fn dimensions_label(w: u32, h: u32) -> String {
    format!("{} x {}", w, h)
}
