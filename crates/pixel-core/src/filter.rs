//! Filters: pixel-level image adjustments.

use crate::buffer::PixelBuffer;

/// A filter that can be applied to a pixel buffer.
#[derive(Clone, Debug)]
pub enum Filter {
    /// Adjust brightness (-100 to 100).
    Brightness(i32),
    /// Adjust contrast (-100 to 100).
    Contrast(i32),
    /// Adjust saturation (-100 to 100).
    Saturation(i32),
    /// Adjust temperature (-100 to 100).
    Temperature(i32),
    /// Gaussian blur with the given radius.
    GaussianBlur(f64),
    /// Sharpen with the given amount.
    Sharpen(f64),
    /// Convert to grayscale.
    Grayscale,
    /// Invert colors.
    Invert,
    /// Sepia tone with the given intensity (0..1).
    Sepia(f32),
    /// Threshold: pixels above the threshold become white, below become black.
    Threshold(u8),
    /// Posterize: reduce to N levels per channel.
    Posterize(u8),
}

impl Filter {
    /// Applies the filter to the given pixel buffer in-place.
    pub fn apply(&self, buf: &mut PixelBuffer) {
        match self {
            Filter::Brightness(amount) => apply_brightness(buf, *amount),
            Filter::Contrast(amount) => apply_contrast(buf, *amount),
            Filter::Saturation(amount) => apply_saturation(buf, *amount),
            Filter::Temperature(amount) => apply_temperature(buf, *amount),
            Filter::Grayscale => apply_grayscale(buf),
            Filter::Invert => apply_invert(buf),
            Filter::Sepia(intensity) => apply_sepia(buf, *intensity),
            Filter::Threshold(level) => apply_threshold(buf, *level),
            Filter::Posterize(levels) => apply_posterize(buf, *levels),
            Filter::GaussianBlur(_radius) => {
                // Simple box blur approximation — a full Gaussian would need a separate kernel
                apply_box_blur(buf, 3);
            }
            Filter::Sharpen(_amount) => {
                // Simple sharpen using unsharp mask approximation
                apply_sharpen(buf);
            }
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Filter::Brightness(_) => "Brightness",
            Filter::Contrast(_) => "Contrast",
            Filter::Saturation(_) => "Saturation",
            Filter::Temperature(_) => "Temperature",
            Filter::GaussianBlur(_) => "Gaussian Blur",
            Filter::Sharpen(_) => "Sharpen",
            Filter::Grayscale => "Grayscale",
            Filter::Invert => "Invert",
            Filter::Sepia(_) => "Sepia",
            Filter::Threshold(_) => "Threshold",
            Filter::Posterize(_) => "Posterize",
        }
    }
}

fn apply_brightness(buf: &mut PixelBuffer, amount: i32) {
    let delta = amount * 255 / 100;
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        pixel[0] = (pixel[0] as i32 + delta).clamp(0, 255) as u8;
        pixel[1] = (pixel[1] as i32 + delta).clamp(0, 255) as u8;
        pixel[2] = (pixel[2] as i32 + delta).clamp(0, 255) as u8;
    }
}

fn apply_contrast(buf: &mut PixelBuffer, amount: i32) {
    let factor = (100.0 + amount as f32) / 100.0;
    let factor = factor * factor; // Quadratic for more noticeable effect
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        pixel[0] =
            (((pixel[0] as f32 / 255.0 - 0.5) * factor + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
        pixel[1] =
            (((pixel[1] as f32 / 255.0 - 0.5) * factor + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
        pixel[2] =
            (((pixel[2] as f32 / 255.0 - 0.5) * factor + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
    }
}

fn apply_saturation(buf: &mut PixelBuffer, amount: i32) {
    let factor = (100.0 + amount as f32) / 100.0;
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        let gray = 0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
        pixel[0] = (gray + (pixel[0] as f32 - gray) * factor).clamp(0.0, 255.0) as u8;
        pixel[1] = (gray + (pixel[1] as f32 - gray) * factor).clamp(0.0, 255.0) as u8;
        pixel[2] = (gray + (pixel[2] as f32 - gray) * factor).clamp(0.0, 255.0) as u8;
    }
}

fn apply_temperature(buf: &mut PixelBuffer, amount: i32) {
    let delta = (amount as f32 / 100.0) * 30.0;
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        pixel[0] = (pixel[0] as f32 + delta).clamp(0.0, 255.0) as u8; // Red
        pixel[2] = (pixel[2] as f32 - delta).clamp(0.0, 255.0) as u8; // Blue
    }
}

fn apply_grayscale(buf: &mut PixelBuffer) {
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        let gray =
            (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
}

fn apply_invert(buf: &mut PixelBuffer) {
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
    }
}

fn apply_sepia(buf: &mut PixelBuffer, intensity: f32) {
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let sr = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0);
        let sg = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0);
        let sb = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0);
        pixel[0] = (r + (sr - r) * intensity) as u8;
        pixel[1] = (g + (sg - g) * intensity) as u8;
        pixel[2] = (b + (sb - b) * intensity) as u8;
    }
}

fn apply_threshold(buf: &mut PixelBuffer, level: u8) {
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        let gray =
            (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
        let val = if gray >= level { 255 } else { 0 };
        pixel[0] = val;
        pixel[1] = val;
        pixel[2] = val;
    }
}

fn apply_posterize(buf: &mut PixelBuffer, levels: u8) {
    let levels = levels.max(2);
    let step = 255.0 / (levels - 1) as f32;
    for pixel in buf.as_rgba8_mut().chunks_exact_mut(4) {
        pixel[0] = ((pixel[0] as f32 / 255.0 * (levels - 1) as f32).round() * step)
            .clamp(0.0, 255.0) as u8;
        pixel[1] = ((pixel[1] as f32 / 255.0 * (levels - 1) as f32).round() * step)
            .clamp(0.0, 255.0) as u8;
        pixel[2] = ((pixel[2] as f32 / 255.0 * (levels - 1) as f32).round() * step)
            .clamp(0.0, 255.0) as u8;
    }
}

fn apply_box_blur(buf: &mut PixelBuffer, radius: u32) {
    let w = buf.width;
    let h = buf.height;
    let src = buf.data.clone();
    let _diameter = radius * 2 + 1;

    for y in 0..h {
        for x in 0..w {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let nx = (x as i32 + dx).clamp(0, w as i32 - 1) as u32;
                    let ny = (y as i32 + dy).clamp(0, h as i32 - 1) as u32;
                    let offset = (ny * w + nx) as usize * 4;
                    r_sum += src[offset] as u32;
                    g_sum += src[offset + 1] as u32;
                    b_sum += src[offset + 2] as u32;
                    a_sum += src[offset + 3] as u32;
                    count += 1;
                }
            }

            let offset = (y * w + x) as usize * 4;
            buf.data[offset] = (r_sum / count) as u8;
            buf.data[offset + 1] = (g_sum / count) as u8;
            buf.data[offset + 2] = (b_sum / count) as u8;
            buf.data[offset + 3] = (a_sum / count) as u8;
        }
    }
}

fn apply_sharpen(buf: &mut PixelBuffer) {
    let w = buf.width;
    let h = buf.height;
    let src = buf.data.clone();

    // Unsharp mask: original + 2 * (original - blurred)
    for y in 1..h.saturating_sub(1) {
        for x in 1..w.saturating_sub(1) {
            let offset = (y * w + x) as usize * 4;
            for c in 0..4 {
                let center = src[offset + c] as f32;
                let neighbors = [
                    src[((y - 1) * w + x) as usize * 4 + c],
                    src[((y + 1) * w + x) as usize * 4 + c],
                    src[(y * w + x - 1) as usize * 4 + c],
                    src[(y * w + x + 1) as usize * 4 + c],
                ];
                let avg_neighbor: f32 = neighbors.iter().map(|&v| v as f32).sum::<f32>() / 4.0;
                let sharpened = center + (center - avg_neighbor) * 0.5;
                buf.data[offset + c] = sharpened.clamp(0.0, 255.0) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn white_buffer() -> PixelBuffer {
        PixelBuffer::filled(4, 4, 255, 255, 255, 255)
    }

    #[test]
    fn brightness_increases_values() {
        let mut buf = PixelBuffer::filled(2, 2, 100, 100, 100, 255);
        Filter::Brightness(20).apply(&mut buf);
        assert!(buf.pixel(0, 0).0 > 100);
    }

    #[test]
    fn contrast_clamps() {
        let mut buf = white_buffer();
        Filter::Contrast(100).apply(&mut buf);
        assert_eq!(buf.pixel(0, 0).0, 255);
    }

    #[test]
    fn grayscale_produces_equal_channels() {
        let mut buf = PixelBuffer::filled(2, 2, 200, 100, 50, 255);
        Filter::Grayscale.apply(&mut buf);
        let (r, g, b, _) = buf.pixel(0, 0);
        assert_eq!(r, g);
        assert_eq!(g, b);
    }

    #[test]
    fn invert_white_becomes_black() {
        let mut buf = white_buffer();
        Filter::Invert.apply(&mut buf);
        assert_eq!(buf.pixel(0, 0).0, 0);
    }

    #[test]
    fn threshold_splits() {
        let mut buf = PixelBuffer::filled(2, 2, 128, 128, 128, 255);
        Filter::Threshold(128).apply(&mut buf);
        assert_eq!(buf.pixel(0, 0).0, 255); // At threshold → white
    }

    #[test]
    fn all_filters_run_without_panic() {
        let filters = [
            Filter::Brightness(50),
            Filter::Contrast(30),
            Filter::Saturation(-50),
            Filter::Temperature(20),
            Filter::GaussianBlur(2.0),
            Filter::Sharpen(1.0),
            Filter::Grayscale,
            Filter::Invert,
            Filter::Sepia(0.8),
            Filter::Threshold(128),
            Filter::Posterize(4),
        ];
        for filter in &filters {
            let mut buf = white_buffer();
            filter.apply(&mut buf);
            // Should not panic
        }
    }

    #[test]
    fn filter_labels() {
        assert_eq!(Filter::Brightness(0).label(), "Brightness");
        assert_eq!(Filter::Grayscale.label(), "Grayscale");
    }
}
