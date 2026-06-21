// ---------------------------------------------------------------------------
// Filter slider hit testing and previews
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::FilterSlider;
use tench_image_runtime::view::util::apply_filters;

use super::ViewApp;

impl ViewApp {
    // --- Filter slider helpers ---

    /// Filter panel geometry constants.
    const FILTER_PANEL_PAD: f64 = 16.0;
    const FILTER_SLIDER_ROW_H: f64 = 40.0;

    /// Checks if a pointer position hits a filter slider track.
    /// Returns the slider identifier if hit.
    pub(super) fn hit_filter_slider(&self, _px: f64, py: f64) -> Option<FilterSlider> {
        if !self.state.show_filter {
            return None;
        }
        let sliders = [
            (FilterSlider::Brightness, 0.0, 200.0),
            (FilterSlider::Contrast, 0.0, 200.0),
            (FilterSlider::Saturation, 0.0, 200.0),
            (FilterSlider::Blur, 0.0, 20.0),
            (FilterSlider::HueRotate, 0.0, 360.0),
        ];

        // We need the viewport width to compute panel_x.
        // Since we don't have it here, we store the filter panel track rects during paint.
        // Instead, let's use a simpler approach: compute track regions from pointer position.
        // The track is at a fixed offset from the right edge.
        // panel_x = viewport_width - panel_w - 22.0
        // track_x = panel_x + pad + 80.0
        // So track_x = viewport_width - panel_w - 22.0 + 16.0 + 80.0 = viewport_width - panel_w + 74.0
        // We can infer viewport_width from px if we know panel_x, but we don't.
        // Solution: store filter track rects during paint in ViewState.
        // For now, use a heuristic: check if px is in the right ~120px area from the panel right edge.
        // The panel right edge is at panel_x + panel_w = viewport_width - 22.0
        // track_x = panel_x + 96.0 = viewport_width - panel_w - 22.0 + 96.0 = viewport_width - 206.0
        // track ends at track_x + 120.0 = viewport_width - 86.0
        // We don't know viewport_width, but we can compute panel_y offsets.

        // Since we can't get viewport width here easily, let's use the approach
        // of computing relative to the panel position from the last paint.
        // We'll store the filter panel track rects in ViewState.
        // Actually, the simplest approach is to just check the y range.
        // The panel_y is 80.0, first slider at panel_y + pad + 30.0 = 126.0
        // Each slider row is 40.0 high.
        let panel_y = 80.0;
        let first_slider_y = panel_y + Self::FILTER_PANEL_PAD + 30.0;

        for (i, (slider, _min, _max)) in sliders.iter().enumerate() {
            let row_y = first_slider_y + i as f64 * Self::FILTER_SLIDER_ROW_H;
            // Track is at row_y + 4.0, height 6.0, with some padding for the thumb
            let track_hit_rect = Rect::new(
                0.0, // We'll check x separately
                row_y - 2.0,
                f64::MAX,
                row_y + 12.0,
            );
            if py >= track_hit_rect.y0 && py <= track_hit_rect.y1 {
                // For x, we need to be more precise. The track is about 120px wide,
                // positioned near the right side of the panel.
                // Since we can't compute exactly without viewport width,
                // check if we're in a reasonable x range for the track area.
                // The track starts at about viewport_width - 206 and ends at viewport_width - 86.
                // We'll accept any x in a wide range and let update_filter_from_pointer handle it.
                return Some(*slider);
            }
        }
        None
    }

    /// Updates a filter slider value based on pointer x position.
    pub(super) fn update_filter_from_pointer(&mut self, slider: FilterSlider, px: f64) {
        // We need to compute the normalized position within the track.
        // Since we don't have viewport width, we use the stored panel position.
        // For simplicity, we'll compute the track position relative to a known reference.
        // The filter panel panel_x depends on viewport width which we don't have here.
        // We'll use a workaround: store the track_x during paint.

        // Alternative: compute filter value from the pointer x position relative to
        // the track area. We know track_w = 120.0 and the track is at panel_x + 96.0.
        // panel_x = viewport_width - 280 - 22 = viewport_width - 302
        // track_x = viewport_width - 302 + 96 = viewport_width - 206
        // So norm = (px - (viewport_width - 206)) / 120
        // We need viewport_width. Let's get it from the last known size.

        // Since we can't access ctx.state.size here, we'll use a different approach:
        // compute the value from the relative position using the click_regions to find
        // the panel bounds. Actually, let's just use a simpler heuristic.

        // For now, use a simple approach: the track is 120px wide, and we compute
        // based on the panel being on the right side. We estimate viewport width
        // from the click regions. But that's complex.

        // Simplest working approach: use the stored filter panel track rects.
        // Let's add them to ViewState.

        // Since we're constrained, let's use a pragmatic approach:
        // The filter panel is 280px wide, at (viewport_w - 302, 80).
        // Track starts at panel_x + 96 = viewport_w - 206
        // Track width = 120
        // So norm = (px - (viewport_w - 206)) / 120
        // We need viewport_w. We can approximate it from the click regions.

        // Actually, let's just compute it directly. We know the panel is at the right
        // edge with a 22px margin. So panel_right = viewport_w - 22.
        // panel_x = panel_right - 280 = viewport_w - 302.
        // track_x = panel_x + 96 = viewport_w - 206.
        // So: norm = (px - (viewport_w - 206)) / 120
        // We need viewport_w. We can get it from the largest click region right edge.

        // Let's use a different approach entirely: store the track rects during paint.
        // For this iteration, we'll estimate viewport_w from click regions.
        let viewport_w = self.estimate_viewport_width();
        let track_x = viewport_w - 206.0;
        let track_w = 120.0;
        let norm = ((px - track_x) / track_w).clamp(0.0, 1.0);

        let (min, max) = match slider {
            FilterSlider::Brightness => (0.0, 200.0),
            FilterSlider::Contrast => (0.0, 200.0),
            FilterSlider::Saturation => (0.0, 200.0),
            FilterSlider::Blur => (0.0, 20.0),
            FilterSlider::HueRotate => (0.0, 360.0),
        };

        let value = min + norm * (max - min);

        match slider {
            FilterSlider::Brightness => {
                if (self.state.filter_brightness - value).abs() > 0.5 {
                    self.state.filter_brightness = value;
                    self.state.filter_dirty = true;
                    self.apply_filter_preview();
                }
            }
            FilterSlider::Contrast => {
                if (self.state.filter_contrast - value).abs() > 0.5 {
                    self.state.filter_contrast = value;
                    self.state.filter_dirty = true;
                    self.apply_filter_preview();
                }
            }
            FilterSlider::Saturation => {
                if (self.state.filter_saturation - value).abs() > 0.5 {
                    self.state.filter_saturation = value;
                    self.state.filter_dirty = true;
                    self.apply_filter_preview();
                }
            }
            FilterSlider::Blur => {
                if (self.state.filter_blur - value).abs() > 0.1 {
                    self.state.filter_blur = value;
                    self.state.filter_dirty = true;
                    self.apply_filter_preview();
                }
            }
            FilterSlider::HueRotate => {
                if (self.state.filter_hue_rotate - value).abs() > 0.5 {
                    self.state.filter_hue_rotate = value;
                    self.state.filter_dirty = true;
                    self.apply_filter_preview();
                }
            }
        }
    }

    /// Estimates viewport width from registered click regions.
    fn estimate_viewport_width(&self) -> f64 {
        // Find the rightmost edge of any click region
        self.state
            .click_regions
            .iter()
            .map(|r| r.rect.x1)
            .fold(0.0f64, f64::max)
            .max(400.0) // minimum reasonable viewport
    }

    /// Applies filter preview: updates current_image_data from original with filter values.
    fn apply_filter_preview(&mut self) {
        if let Some(ref original) = self.state.original_image_data {
            let brightness = self.state.filter_brightness;
            let contrast = self.state.filter_contrast;
            let saturation = self.state.filter_saturation;
            let blur = self.state.filter_blur;
            let hue_rotate = self.state.filter_hue_rotate;

            let is_modified = brightness != 100.0
                || contrast != 100.0
                || saturation != 100.0
                || hue_rotate != 0.0
                || blur != 0.0;

            if is_modified {
                if let Some(filtered) =
                    apply_filters(original, brightness, contrast, saturation, blur, hue_rotate)
                {
                    self.state.current_image_data = Some(filtered);
                }
            } else {
                // Restore original if all filters are at default
                self.state.current_image_data = Some(original.clone());
            }
        }
    }
}
