use super::*;

impl PixelDesignState {
    // Phase 4: Apply preset filters
    pub fn apply_adjust_preset(&mut self, preset: &str) {
        let filter = match preset {
            "Warm" => tench_pixel_core::Filter::Temperature(30),
            "Cool" => tench_pixel_core::Filter::Temperature(-30),
            "B&W" => tench_pixel_core::Filter::Saturation(-100),
            "Vintage" => tench_pixel_core::Filter::Sepia(0.6),
            "Vivid" => tench_pixel_core::Filter::Saturation(50),
            "Muted" => tench_pixel_core::Filter::Saturation(-40),
            "Film" => tench_pixel_core::Filter::Contrast(20),
            "HDR" => tench_pixel_core::Filter::Contrast(40),
            _ => return,
        };
        self.push_history("Apply preset");
        if let Some(layer) = self.document.active_layer_mut() {
            layer.apply_filter(&filter);
        }
        self.status_msg = format!("Applied preset: {}", preset);
    }

    // Phase 4: Apply individual adjust filter
    pub fn apply_adjust_filter(&mut self, index: usize) {
        let filter = match index {
            0 => Some(tench_pixel_core::Filter::Brightness(
                self.adjust_values.brightness,
            )),
            1 => Some(tench_pixel_core::Filter::Contrast(
                self.adjust_values.contrast,
            )),
            2 => Some(tench_pixel_core::Filter::Saturation(
                self.adjust_values.saturation,
            )),
            3 => Some(tench_pixel_core::Filter::Temperature(
                self.adjust_values.temperature,
            )),
            4 => Some(tench_pixel_core::Filter::Sharpen(
                self.adjust_values.sharpness as f64 / 100.0,
            )),
            5 => Some(tench_pixel_core::Filter::Sharpen(
                self.adjust_values.sharpness as f64 / 100.0,
            )),
            6 => Some(tench_pixel_core::Filter::GaussianBlur(
                self.adjust_values.blur as f64 / 10.0,
            )),
            _ => None,
        };
        if let Some(f) = filter {
            if let Some(layer) = self.document.active_layer_mut() {
                layer.apply_filter(&f);
            }
        }
    }
}
