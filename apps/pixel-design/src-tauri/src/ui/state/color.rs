use super::*;
use tench_ui::prelude::Color;

impl PixelDesignState {
    // Phase 2: Color picker
    pub fn open_color_picker(&mut self, target_fg: bool) {
        self.show_color_picker = true;
        self.color_picker_target_fg = target_fg;
        self.active_modal = ModalType::ColorPicker;
        // Convert current color to HSV
        let color = if target_fg {
            self.fg_color
        } else {
            self.bg_color
        };
        self.color_picker_original = color;
        self.color_picker_preview = color;
        let packed = color.to_u32();
        let r = ((packed >> 24) & 0xFF) as f32 / 255.0;
        let g = ((packed >> 16) & 0xFF) as f32 / 255.0;
        let b = ((packed >> 8) & 0xFF) as f32 / 255.0;
        let (h, s, v) = rgb_to_hsv(r, g, b);
        self.color_hue = h;
        self.color_saturation = s;
        self.color_value = v;
    }

    pub fn close_color_picker(&mut self) {
        self.show_color_picker = false;
        self.active_modal = ModalType::None;
    }

    pub fn apply_color_picker(&mut self) {
        let color = self.color_picker_preview;
        if self.color_picker_target_fg {
            self.fg_color = color;
        } else {
            self.bg_color = color;
        }
        self.add_recent_color(color);
        self.close_color_picker();
        self.status_msg = "Color applied".into();
    }

    pub fn set_color_picker_hsv(&mut self, h: f32, s: f32, v: f32) {
        self.color_hue = h;
        self.color_saturation = s;
        self.color_value = v;
        let (r, g, b) = hsv_to_rgb(h, s, v);
        self.color_picker_preview =
            Color::rgb8((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
        self.status_msg = "Color preview updated".into();
    }

    pub fn cancel_color_picker(&mut self) {
        self.color_picker_preview = self.color_picker_original;
        self.close_color_picker();
        self.status_msg = "Color cancelled".into();
    }

    pub fn set_fg_color_hex(&mut self, hex: &str) {
        if let Some(color) = parse_hex_color(hex) {
            self.fg_color = color;
            self.add_recent_color(color);
        }
    }

    pub fn add_recent_color(&mut self, color: Color) {
        // Don't add duplicates
        if self
            .recent_colors
            .iter()
            .any(|c| c.to_u32() == color.to_u32())
        {
            return;
        }
        self.recent_colors.insert(0, color);
        if self.recent_colors.len() > 12 {
            self.recent_colors.pop();
        }
    }

    pub fn sample_color_at(&mut self, x: f64, y: f64) {
        let px = x as u32;
        let py = y as u32;
        if let Some(ref composited) = self.composited_image {
            let (r, g, b, _a) = composited.pixel(px, py);
            self.fg_color = Color::rgb8(r, g, b);
            self.add_recent_color(self.fg_color);
            self.status_msg = format!("Sampled color ({}, {}, {})", r, g, b);
        } else if let Some(layer) = self.active_layer() {
            let (r, g, b, _a) = layer.buffer.pixel(px, py);
            self.fg_color = Color::rgb8(r, g, b);
            self.add_recent_color(self.fg_color);
            self.status_msg = format!("Sampled color ({}, {}, {})", r, g, b);
        }
    }

    pub fn sample_next_foreground(&mut self) {
        // Open color picker instead of cycling
        self.open_color_picker(true);
    }

    pub fn swap_colors(&mut self) {
        std::mem::swap(&mut self.fg_color, &mut self.bg_color);
        self.status_msg = "Foreground/background swapped".into();
    }
}

// Color conversion helpers
pub(super) fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta < f32::EPSILON {
        0.0
    } else if (max - r).abs() < f32::EPSILON {
        60.0 * (((g - b) / delta) % 6.0)
    } else if (max - g).abs() < f32::EPSILON {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let h = if h < 0.0 { h + 360.0 } else { h };

    let s = if max < f32::EPSILON { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

pub(super) fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r + m, g + m, b + m)
}

pub(super) fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color::rgb8(r, g, b))
    } else {
        None
    }
}
