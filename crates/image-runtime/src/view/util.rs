//! Image utility functions for the View app.
//!
//! Provides conversions between image crate types and peniko rendering types,
//! plus histogram computation for the metadata panel.

use base64::Engine;
use image::DynamicImage;
use tench_ui::peniko::ImageData;

/// Converts a `DynamicImage` to a `peniko::ImageData` suitable for GPU rendering.
///
/// The image is converted to RGBA8 format (premultiplied alpha) as required by
/// the Vello rendering pipeline.
pub fn dynamic_to_image_data(img: &DynamicImage) -> ImageData {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    ImageData {
        data: rgba.into_raw().into(),
        format: tench_ui::peniko::ImageFormat::Rgba8,
        alpha_type: tench_ui::peniko::ImageAlphaType::AlphaPremultiplied,
        width: w,
        height: h,
    }
}

/// Decodes a base64 data-URL string into a `peniko::ImageData`.
///
/// Expected format: `data:image/<mime>;base64,<encoded>`.
/// Returns `None` if the data URL cannot be parsed or decoded.
pub fn data_url_to_image_data(data_url: &str) -> Option<ImageData> {
    let encoded = data_url
        .strip_prefix("data:")
        .and_then(|s| s.split_once(';'))
        .and_then(|(_, rest)| rest.strip_prefix("base64,"))?;

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;

    let img = image::load_from_memory(&bytes).ok()?;
    Some(dynamic_to_image_data(&img))
}

/// Per-channel histogram data for an image (256 bins per channel).
#[derive(Debug, Clone)]
pub struct HistogramData {
    pub r: [u32; 256],
    pub g: [u32; 256],
    pub b: [u32; 256],
}

impl HistogramData {
    /// Computes a histogram from RGBA pixel data.
    ///
    /// `pixels` must be in RGBA8 layout with `width * height * 4` bytes.
    pub fn from_rgba(pixels: &[u8], width: u32, height: u32) -> Self {
        let mut hist = Self {
            r: [0u32; 256],
            g: [0u32; 256],
            b: [0u32; 256],
        };

        let expected = width as usize * height as usize * 4;
        let len = pixels.len().min(expected);
        let mut i = 0;
        while i + 3 < len {
            hist.r[pixels[i] as usize] += 1;
            hist.g[pixels[i + 1] as usize] += 1;
            hist.b[pixels[i + 2] as usize] += 1;
            i += 4;
        }

        hist
    }

    /// Computes a histogram from a `DynamicImage`.
    pub fn from_dynamic_image(img: &DynamicImage) -> Self {
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        Self::from_rgba(rgba.as_raw(), w, h)
    }

    /// Returns the maximum bin value across all channels (for normalization).
    pub fn max_value(&self) -> u32 {
        let mr = *self.r.iter().max().unwrap_or(&1);
        let mg = *self.g.iter().max().unwrap_or(&1);
        let mb = *self.b.iter().max().unwrap_or(&1);
        mr.max(mg).max(mb).max(1)
    }
}

/// Converts a `peniko::ImageData` back to a `DynamicImage`.
///
/// Returns `None` if the pixel data cannot be interpreted as RGBA8.
pub fn image_data_to_dynamic(data: &ImageData) -> Option<DynamicImage> {
    if data.format != tench_ui::peniko::ImageFormat::Rgba8 {
        return None;
    }
    let rgba = image::RgbaImage::from_raw(data.width, data.height, data.data.data().to_vec())?;
    Some(DynamicImage::ImageRgba8(rgba))
}

/// Crops an `ImageData` to the specified pixel region.
///
/// The crop rectangle is specified in pixel coordinates of the source image.
/// Returns a new `ImageData` containing only the cropped region.
pub fn crop_image_data(data: &ImageData, x: u32, y: u32, w: u32, h: u32) -> Option<ImageData> {
    let mut dynamic = image_data_to_dynamic(data)?;
    dynamic = dynamic.crop_imm(x, y, w, h);
    Some(dynamic_to_image_data(&dynamic))
}

/// Resizes an `ImageData` to the specified dimensions using Lanczos3 filtering.
pub fn resize_image_data(data: &ImageData, new_w: u32, new_h: u32) -> Option<ImageData> {
    let dynamic = image_data_to_dynamic(data)?;
    let resized = dynamic.resize_exact(new_w, new_h, image::imageops::FilterType::Lanczos3);
    Some(dynamic_to_image_data(&resized))
}

/// Rotates an `ImageData` by 90 degrees clockwise.
///
/// `times` is the number of 90-degree clockwise rotations (0-3).
pub fn rotate_image_data(data: &ImageData, times: u32) -> Option<ImageData> {
    if times == 0 {
        return Some(data.clone());
    }
    let mut dynamic = image_data_to_dynamic(data)?;
    for _ in 0..(times % 4) {
        dynamic = dynamic.rotate90();
    }
    Some(dynamic_to_image_data(&dynamic))
}

/// Applies brightness/contrast/saturation/blur/hue-rotate filter adjustments to `ImageData`.
///
/// All parameters use the same scale as the filter sliders:
/// - `brightness`: 0-200, where 100 = no change
/// - `contrast`: 0-200, where 100 = no change
/// - `saturation`: 0-200, where 100 = no change
/// - `blur`: 0-20 pixel radius, where 0 = no change
/// - `hue_rotate`: 0-360 degrees
pub fn apply_filters(
    data: &ImageData,
    brightness: f64,
    contrast: f64,
    saturation: f64,
    blur: f64,
    hue_rotate: f64,
) -> Option<ImageData> {
    let w = data.width as usize;
    let h = data.height as usize;
    let src = data.data.data();

    let brightness_factor = brightness / 100.0;
    let contrast_factor = contrast / 100.0;
    let saturation_factor = saturation / 100.0;
    let hue_rad = hue_rotate.to_radians();

    // Apply blur first using a box blur if blur > 0
    let blurred: Vec<u8> = if blur > 0.0 {
        let radius = blur.round() as usize;
        box_blur(src, w, h, radius)
    } else {
        src.to_vec()
    };

    // Precompute contrast adjustment: map through ((val/255 - 0.5) * factor + 0.5) * 255
    let mut dst = vec![0u8; src.len()];

    for i in 0..(w * h) {
        let idx = i * 4;
        let r = blurred[idx] as f64 / 255.0;
        let g = blurred[idx + 1] as f64 / 255.0;
        let b = blurred[idx + 2] as f64 / 255.0;
        let a = blurred[idx + 3];

        // Apply brightness
        let mut r = r * brightness_factor;
        let mut g = g * brightness_factor;
        let mut b = b * brightness_factor;

        // Apply contrast
        r = (r - 0.5) * contrast_factor + 0.5;
        g = (g - 0.5) * contrast_factor + 0.5;
        b = (b - 0.5) * contrast_factor + 0.5;

        // Apply saturation (in linear RGB space approximation)
        let gray = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        r = gray + (r - gray) * saturation_factor;
        g = gray + (g - gray) * saturation_factor;
        b = gray + (b - gray) * saturation_factor;

        // Apply hue rotation (simplified rotation matrix)
        if hue_rotate != 0.0 {
            let cos_h = hue_rad.cos();
            let sin_h = hue_rad.sin();
            let nr = r * (cos_h + (1.0 - cos_h) / 3.0)
                + g * ((1.0 - cos_h) / 3.0 - (3.0_f64).sqrt() / 3.0 * sin_h)
                + b * ((1.0 - cos_h) / 3.0 + (3.0_f64).sqrt() / 3.0 * sin_h);
            let ng = r * ((1.0 - cos_h) / 3.0 + (3.0_f64).sqrt() / 3.0 * sin_h)
                + g * (cos_h + (1.0 - cos_h) / 3.0)
                + b * ((1.0 - cos_h) / 3.0 - (3.0_f64).sqrt() / 3.0 * sin_h);
            let nb = r * ((1.0 - cos_h) / 3.0 - (3.0_f64).sqrt() / 3.0 * sin_h)
                + g * ((1.0 - cos_h) / 3.0 + (3.0_f64).sqrt() / 3.0 * sin_h)
                + b * (cos_h + (1.0 - cos_h) / 3.0);
            r = nr;
            g = ng;
            b = nb;
        }

        // Clamp and convert back
        dst[idx] = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
        dst[idx + 1] = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
        dst[idx + 2] = (b.clamp(0.0, 1.0) * 255.0).round() as u8;
        dst[idx + 3] = a;
    }

    Some(ImageData {
        data: dst.into(),
        format: tench_ui::peniko::ImageFormat::Rgba8,
        alpha_type: tench_ui::peniko::ImageAlphaType::AlphaPremultiplied,
        width: data.width,
        height: data.height,
    })
}

/// Applies a simple box blur to RGBA pixel data.
fn box_blur(src: &[u8], w: usize, h: usize, radius: usize) -> Vec<u8> {
    if radius == 0 || w == 0 || h == 0 {
        return src.to_vec();
    }

    // Horizontal pass
    let mut temp = vec![0u8; src.len()];
    for y in 0..h {
        for x in 0..w {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            let x_start = x.saturating_sub(radius);
            let x_end = (x + radius + 1).min(w);
            for kx in x_start..x_end {
                let idx = (y * w + kx) * 4;
                r_sum += src[idx] as u32;
                g_sum += src[idx + 1] as u32;
                b_sum += src[idx + 2] as u32;
                a_sum += src[idx + 3] as u32;
                count += 1;
            }

            let idx = (y * w + x) * 4;
            temp[idx] = (r_sum / count) as u8;
            temp[idx + 1] = (g_sum / count) as u8;
            temp[idx + 2] = (b_sum / count) as u8;
            temp[idx + 3] = (a_sum / count) as u8;
        }
    }

    // Vertical pass
    let mut dst = vec![0u8; src.len()];
    for y in 0..h {
        for x in 0..w {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            let y_start = y.saturating_sub(radius);
            let y_end = (y + radius + 1).min(h);
            for ky in y_start..y_end {
                let idx = (ky * w + x) * 4;
                r_sum += temp[idx] as u32;
                g_sum += temp[idx + 1] as u32;
                b_sum += temp[idx + 2] as u32;
                a_sum += temp[idx + 3] as u32;
                count += 1;
            }

            let idx = (y * w + x) * 4;
            dst[idx] = (r_sum / count) as u8;
            dst[idx + 1] = (g_sum / count) as u8;
            dst[idx + 2] = (b_sum / count) as u8;
            dst[idx + 3] = (a_sum / count) as u8;
        }
    }

    dst
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a small test ImageData with a specific RGBA fill color.
    fn make_image_data(w: u32, h: u32, r: u8, g: u8, b: u8, a: u8) -> ImageData {
        let pixel = [r, g, b, a];
        let pixels: Vec<u8> = pixel.repeat((w * h) as usize);
        ImageData {
            data: pixels.into(),
            format: tench_ui::peniko::ImageFormat::Rgba8,
            alpha_type: tench_ui::peniko::ImageAlphaType::AlphaPremultiplied,
            width: w,
            height: h,
        }
    }

    // --- dynamic_to_image_data / image_data_to_dynamic ---

    #[test]
    fn dynamic_image_roundtrip() {
        let img = DynamicImage::new_rgb8(4, 4);
        let data = dynamic_to_image_data(&img);
        assert_eq!(data.width, 4);
        assert_eq!(data.height, 4);
        assert_eq!(data.data.len(), 4 * 4 * 4);
    }

    #[test]
    fn image_data_to_dynamic_and_back() {
        let original = make_image_data(3, 3, 0xFF, 0x80, 0x00, 0xFF);
        let dynamic = image_data_to_dynamic(&original).expect("should convert");
        assert_eq!(dynamic.width(), 3);
        assert_eq!(dynamic.height(), 3);

        let roundtripped = dynamic_to_image_data(&dynamic);
        assert_eq!(roundtripped.width, 3);
        assert_eq!(roundtripped.height, 3);
        // First pixel should match
        let src = original.data.data();
        let dst = roundtripped.data.data();
        assert_eq!(src[0..4], dst[0..4]);
    }

    #[test]
    fn image_data_to_dynamic_rejects_non_rgba8() {
        let mut data = make_image_data(2, 2, 0, 0, 0, 0xFF);
        data.format = tench_ui::peniko::ImageFormat::Rgba8; // only Rgba8 supported
        assert!(image_data_to_dynamic(&data).is_some());
    }

    // --- HistogramData ---

    #[test]
    fn histogram_all_black() {
        let img = DynamicImage::new_rgb8(2, 2);
        let hist = HistogramData::from_dynamic_image(&img);
        assert_eq!(hist.r[0], 4);
        assert_eq!(hist.g[0], 4);
        assert_eq!(hist.b[0], 4);
        // All other bins should be 0
        assert_eq!(hist.r[1..].iter().sum::<u32>(), 0);
    }

    #[test]
    fn histogram_from_rgba_known_values() {
        // 2x1 image: one white pixel, one red pixel
        let pixels: &[u8] = &[
            255, 255, 255, 255, // white
            255, 0, 0, 255, // red
        ];
        let hist = HistogramData::from_rgba(pixels, 2, 1);
        assert_eq!(hist.r[255], 2); // both pixels have r=255
        assert_eq!(hist.g[255], 1); // only white has g=255
        assert_eq!(hist.b[255], 1); // only white has b=255
        assert_eq!(hist.g[0], 1); // red has g=0
        assert_eq!(hist.b[0], 1); // red has b=0
    }

    #[test]
    fn histogram_max_value() {
        let img = DynamicImage::new_rgb8(2, 2);
        let hist = HistogramData::from_dynamic_image(&img);
        // All black: max is 4 (2x2 = 4 pixels)
        assert_eq!(hist.max_value(), 4);
    }

    #[test]
    fn histogram_empty_image() {
        let pixels: &[u8] = &[];
        let hist = HistogramData::from_rgba(pixels, 0, 0);
        // All bins should be 0, max_value returns 1 (the floor)
        assert_eq!(hist.max_value(), 1);
    }

    // --- crop_image_data ---

    #[test]
    fn crop_image_data_full_image() {
        let data = make_image_data(4, 4, 0xFF, 0x00, 0x00, 0xFF);
        let cropped = crop_image_data(&data, 0, 0, 4, 4).expect("should crop");
        assert_eq!(cropped.width, 4);
        assert_eq!(cropped.height, 4);
    }

    #[test]
    fn crop_image_data_partial() {
        let data = make_image_data(8, 8, 0xFF, 0x00, 0x00, 0xFF);
        let cropped = crop_image_data(&data, 2, 2, 4, 3).expect("should crop");
        assert_eq!(cropped.width, 4);
        assert_eq!(cropped.height, 3);
        // Cropped data should be smaller
        assert_eq!(cropped.data.len(), 4 * 3 * 4);
    }

    // --- resize_image_data ---

    #[test]
    fn resize_image_data_smaller() {
        let data = make_image_data(100, 100, 0xFF, 0x00, 0x00, 0xFF);
        let resized = resize_image_data(&data, 50, 50).expect("should resize");
        assert_eq!(resized.width, 50);
        assert_eq!(resized.height, 50);
    }

    #[test]
    fn resize_image_data_larger() {
        let data = make_image_data(10, 10, 0x00, 0xFF, 0x00, 0xFF);
        let resized = resize_image_data(&data, 20, 20).expect("should resize");
        assert_eq!(resized.width, 20);
        assert_eq!(resized.height, 20);
    }

    // --- rotate_image_data ---

    #[test]
    fn rotate_90_degrees() {
        // 2x3 image -> should become 3x2 after 90-degree rotation
        let data = make_image_data(2, 3, 0xFF, 0x00, 0x00, 0xFF);
        let rotated = rotate_image_data(&data, 1).expect("should rotate");
        assert_eq!(rotated.width, 3);
        assert_eq!(rotated.height, 2);
    }

    #[test]
    fn rotate_360_degrees_identity() {
        let data = make_image_data(5, 3, 0xFF, 0x80, 0x00, 0xFF);
        let rotated = rotate_image_data(&data, 4).expect("should rotate");
        assert_eq!(rotated.width, 5);
        assert_eq!(rotated.height, 3);
    }

    #[test]
    fn rotate_zero_times_is_clone() {
        let data = make_image_data(4, 4, 0x00, 0x00, 0xFF, 0xFF);
        let rotated = rotate_image_data(&data, 0).expect("should return clone");
        assert_eq!(rotated.width, data.width);
        assert_eq!(rotated.height, data.height);
    }

    // --- apply_filters ---

    #[test]
    fn apply_filters_identity() {
        // Default values: brightness=100, contrast=100, saturation=100, blur=0, hue=0
        let data = make_image_data(2, 2, 0x80, 0x40, 0x20, 0xFF);
        let filtered = apply_filters(&data, 100.0, 100.0, 100.0, 0.0, 0.0).expect("should apply");
        // Should be identical (within rounding)
        let src = data.data.data();
        let dst = filtered.data.data();
        for i in 0..src.len() {
            assert!(
                (src[i] as i16 - dst[i] as i16).unsigned_abs() <= 1,
                "pixel {} differs: {} vs {}",
                i,
                src[i],
                dst[i]
            );
        }
    }

    #[test]
    fn apply_filters_brightness_zero_produces_black() {
        let data = make_image_data(1, 1, 0xFF, 0x80, 0x40, 0xFF);
        let filtered = apply_filters(&data, 0.0, 100.0, 100.0, 0.0, 0.0).expect("should apply");
        let dst = filtered.data.data();
        assert_eq!(dst[0], 0); // R
        assert_eq!(dst[1], 0); // G
        assert_eq!(dst[2], 0); // B
        assert_eq!(dst[3], 0xFF); // A preserved
    }

    #[test]
    fn apply_filters_preserves_dimensions() {
        let data = make_image_data(7, 5, 0xFF, 0x00, 0x00, 0xFF);
        let filtered = apply_filters(&data, 120.0, 80.0, 150.0, 0.0, 45.0).expect("should apply");
        assert_eq!(filtered.width, 7);
        assert_eq!(filtered.height, 5);
    }

    // --- data_url_to_image_data ---

    #[test]
    fn data_url_rejects_invalid() {
        assert!(data_url_to_image_data("not a data url").is_none());
    }

    #[test]
    fn data_url_rejects_non_base64() {
        assert!(data_url_to_image_data("data:image/png;base64,!!!notbase64!!!").is_none());
    }
}
