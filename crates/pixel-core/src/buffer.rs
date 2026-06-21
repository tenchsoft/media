//! Pixel buffer: RGBA8 image data with direct pixel access.

/// A 2D RGBA8 pixel buffer.
///
/// Stores pixels in row-major order with 4 bytes per pixel (R, G, B, A).
#[derive(Clone, Debug)]
pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl PixelBuffer {
    /// Creates a new buffer filled with transparent black.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0u8; (width * height * 4) as usize],
        }
    }

    /// Creates a buffer filled with a solid RGBA color.
    pub fn filled(width: u32, height: u32, r: u8, g: u8, b: u8, a: u8) -> Self {
        let pixel = [r, g, b, a];
        let mut data = Vec::with_capacity((width * height * 4) as usize);
        for _ in 0..(width * height) {
            data.extend_from_slice(&pixel);
        }
        Self {
            width,
            height,
            data,
        }
    }

    /// Creates a buffer from raw RGBA8 bytes.
    ///
    /// Panics if `data.len() != width * height * 4`.
    pub fn from_raw(width: u32, height: u32, data: Vec<u8>) -> Self {
        assert_eq!(
            data.len(),
            (width * height * 4) as usize,
            "PixelBuffer::from_raw: data length mismatch"
        );
        Self {
            width,
            height,
            data,
        }
    }

    /// Creates a buffer from a `DynamicImage`, converting to RGBA8.
    pub fn from_dynamic_image(img: &image::DynamicImage) -> Self {
        let rgba = img.to_rgba8();
        let width = rgba.width();
        let height = rgba.height();
        Self {
            width,
            height,
            data: rgba.into_raw(),
        }
    }

    /// Returns the pixel at (x, y) as (R, G, B, A).
    ///
    /// Returns `(0, 0, 0, 0)` for out-of-bounds coordinates.
    pub fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        if x >= self.width || y >= self.height {
            return (0, 0, 0, 0);
        }
        let offset = (y * self.width + x) as usize * 4;
        (
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        )
    }

    /// Sets the pixel at (x, y). No-op for out-of-bounds coordinates.
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.width + x) as usize * 4;
        self.data[offset] = r;
        self.data[offset + 1] = g;
        self.data[offset + 2] = b;
        self.data[offset + 3] = a;
    }

    /// Returns the raw RGBA8 byte slice.
    pub fn as_rgba8(&self) -> &[u8] {
        &self.data
    }

    /// Returns the raw RGBA8 byte slice mutably.
    pub fn as_rgba8_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Converts to a `DynamicImage` (RGBA8).
    pub fn to_dynamic_image(&self) -> image::DynamicImage {
        let rgba = image::RgbaImage::from_raw(self.width, self.height, self.data.clone())
            .expect("PixelBuffer data should be valid RGBA8");
        image::DynamicImage::ImageRgba8(rgba)
    }

    /// Clears the buffer to transparent black.
    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    /// Returns the total number of pixels.
    pub fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    /// Returns the byte size of the buffer.
    pub fn byte_size(&self) -> usize {
        self.data.len()
    }

    /// Extracts a rectangular region as a new buffer.
    ///
    /// Out-of-bounds regions are filled with transparent black.
    pub fn extract_rect(&self, x: u32, y: u32, w: u32, h: u32) -> PixelBuffer {
        let mut result = PixelBuffer::new(w, h);
        for dy in 0..h {
            for dx in 0..w {
                let (r, g, b, a) = self.pixel(x + dx, y + dy);
                result.set_pixel(dx, dy, r, g, b, a);
            }
        }
        result
    }

    /// Pastes another buffer at the given offset.
    pub fn paste(&mut self, src: &PixelBuffer, x: u32, y: u32) {
        for dy in 0..src.height {
            for dx in 0..src.width {
                let (r, g, b, a) = src.pixel(dx, dy);
                self.set_pixel(x + dx, y + dy, r, g, b, a);
            }
        }
    }

    /// Flood fills from (start_x, start_y) with the given RGBA color.
    ///
    /// Uses a scanline flood fill algorithm. The `tolerance` parameter controls
    /// how much the color can differ from the starting pixel (0 = exact match,
    /// 255 = fill everything).
    // Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
    #[allow(clippy::too_many_arguments)]
    pub fn flood_fill(
        &mut self,
        start_x: u32,
        start_y: u32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        tolerance: u8,
    ) {
        if start_x >= self.width || start_y >= self.height {
            return;
        }

        let (sr, sg, sb, sa) = self.pixel(start_x, start_y);
        let fill_color = (r, g, b, a);

        // Don't fill if the target color is the same as fill color
        if sr == r && sg == g && sb == b && sa == a {
            return;
        }

        let tol = tolerance as i32;
        let w = self.width as usize;
        let h = self.height as usize;
        let mut visited = vec![false; w * h];
        let mut stack = vec![(start_x as usize, start_y as usize)];

        while let Some((x, y)) = stack.pop() {
            if visited[y * w + x] {
                continue;
            }

            let (pr, pg, pb, pa) = self.pixel(x as u32, y as u32);
            if !color_within_tolerance(pr, pg, pb, pa, sr, sg, sb, sa, tol) {
                continue;
            }

            // Scan left
            let mut left = x;
            while left > 0 {
                let (lr, lg, lb, la) = self.pixel((left - 1) as u32, y as u32);
                if !color_within_tolerance(lr, lg, lb, la, sr, sg, sb, sa, tol) {
                    break;
                }
                left -= 1;
            }

            // Scan right
            let mut right = x;
            while right + 1 < w {
                let (rr, rg, rb, ra) = self.pixel((right + 1) as u32, y as u32);
                if !color_within_tolerance(rr, rg, rb, ra, sr, sg, sb, sa, tol) {
                    break;
                }
                right += 1;
            }

            // Fill the scanline and check rows above/below
            for fx in left..=right {
                self.set_pixel(
                    fx as u32,
                    y as u32,
                    fill_color.0,
                    fill_color.1,
                    fill_color.2,
                    fill_color.3,
                );
                visited[y * w + fx] = true;

                if y > 0 && !visited[(y - 1) * w + fx] {
                    stack.push((fx, y - 1));
                }
                if y + 1 < h && !visited[(y + 1) * w + fx] {
                    stack.push((fx, y + 1));
                }
            }
        }
    }

    /// Draws a filled rectangle with the given color.
    // Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_pixel(x + dx, y + dy, r, g, b, a);
            }
        }
    }

    /// Draws a rectangle outline with the given color and stroke width.
    // Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke_rect(
        &mut self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        stroke: u32,
    ) {
        for s in 0..stroke {
            // Top edge
            for dx in 0..w {
                self.set_pixel(x + dx, y + s, r, g, b, a);
            }
            // Bottom edge
            for dx in 0..w {
                self.set_pixel(x + dx, y + h - 1 - s, r, g, b, a);
            }
            // Left edge
            for dy in 0..h {
                self.set_pixel(x + s, y + dy, r, g, b, a);
            }
            // Right edge
            for dy in 0..h {
                self.set_pixel(x + w - 1 - s, y + dy, r, g, b, a);
            }
        }
    }

    /// Draws a filled ellipse within the given bounding rectangle.
    // Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, r: u8, g: u8, b: u8, a: u8) {
        if rx <= 0.0 || ry <= 0.0 {
            return;
        }
        let x0 = (cx - rx - 1.0).max(0.0) as u32;
        let y0 = (cy - ry - 1.0).max(0.0) as u32;
        let x1 = ((cx + rx + 1.0) as u32).min(self.width.saturating_sub(1));
        let y1 = ((cy + ry + 1.0) as u32).min(self.height.saturating_sub(1));
        let rx_sq = rx * rx;
        let ry_sq = ry * ry;

        for py in y0..=y1 {
            for px in x0..=x1 {
                let dx = px as f64 - cx;
                let dy = py as f64 - cy;
                if (dx * dx) / rx_sq + (dy * dy) / ry_sq <= 1.0 {
                    self.set_pixel(px, py, r, g, b, a);
                }
            }
        }
    }

    /// Draws a line between two points using Bresenham's algorithm.
    // Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_line(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        thickness: u32,
    ) {
        let half = (thickness as f64 / 2.0).ceil() as i32;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let (mut cx, mut cy) = (x0, y0);

        loop {
            for oy in -half..=half {
                for ox in -half..=half {
                    let px = cx + ox;
                    let py = cy + oy;
                    if px >= 0 && py >= 0 {
                        self.set_pixel(px as u32, py as u32, r, g, b, a);
                    }
                }
            }
            if cx == x1 && cy == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                cx += sx;
            }
            if e2 < dx {
                err += dx;
                cy += sy;
            }
        }
    }

    /// Renders text onto the buffer using a simple bitmap approach.
    /// For each character, draws a simple pixel representation.
    // Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
    #[allow(clippy::too_many_arguments)]
    pub fn render_text(&mut self, text: &str, x: u32, y: u32, r: u8, g: u8, b: u8, font_size: u32) {
        let mut cx = x;
        for ch in text.chars() {
            if ch == '\n' {
                cx = x;
                continue;
            }
            let glyph = simple_glyph(ch, font_size);
            for (row_idx, row) in glyph.iter().enumerate() {
                for (col_idx, &filled) in row.iter().enumerate() {
                    if filled {
                        self.set_pixel(cx + col_idx as u32, y + row_idx as u32, r, g, b, 255);
                    }
                }
            }
            let glyph_width = glyph.first().map_or(font_size as usize, |r| r.len());
            cx += glyph_width as u32 + 1;
        }
    }
}

/// Checks if two colors are within the given tolerance of each other.
// Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
#[allow(clippy::too_many_arguments)]
fn color_within_tolerance(
    r1: u8,
    g1: u8,
    b1: u8,
    a1: u8,
    r2: u8,
    g2: u8,
    b2: u8,
    a2: u8,
    tolerance: i32,
) -> bool {
    let dr = (r1 as i32 - r2 as i32).abs();
    let dg = (g1 as i32 - g2 as i32).abs();
    let db = (b1 as i32 - b2 as i32).abs();
    let da = (a1 as i32 - a2 as i32).abs();
    // Use max channel difference for stricter matching
    dr <= tolerance && dg <= tolerance && db <= tolerance && da <= tolerance
}

/// Returns a simple bitmap glyph for ASCII characters.
fn simple_glyph(ch: char, size: u32) -> Vec<Vec<bool>> {
    let s = size.max(6) as usize;
    let mut grid = vec![vec![false; s]; s];

    match ch {
        'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' => {
            // Simple block representation for basic characters
            if ch != ' ' {
                let margin = if s > 8 { 1 } else { 0 };
                for row in grid.iter_mut().take(s.saturating_sub(margin)).skip(margin) {
                    for col in row.iter_mut().take(s.saturating_sub(margin)).skip(margin) {
                        *col = true;
                    }
                }
                // Add some shape differentiation
                if s >= 8 {
                    // Hollow out center for some characters
                    if matches!(
                        ch,
                        'A' | 'D'
                            | 'O'
                            | 'P'
                            | 'Q'
                            | 'R'
                            | '0'
                            | '4'
                            | '6'
                            | '8'
                            | '9'
                            | 'a'
                            | 'b'
                            | 'd'
                            | 'e'
                            | 'g'
                            | 'o'
                            | 'p'
                            | 'q'
                    ) {
                        for row in grid.iter_mut().take(s.saturating_sub(2)).skip(2) {
                            for col in row.iter_mut().take(s.saturating_sub(2)).skip(2) {
                                *col = false;
                            }
                        }
                    }
                }
            }
        }
        _ => {
            // Default: small filled block
            for row in grid.iter_mut().take(s.saturating_sub(1)).skip(1) {
                for col in row.iter_mut().take(s.saturating_sub(1)).skip(1) {
                    *col = true;
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_is_transparent_black() {
        let buf = PixelBuffer::new(4, 4);
        assert_eq!(buf.pixel(0, 0), (0, 0, 0, 0));
        assert_eq!(buf.pixel(3, 3), (0, 0, 0, 0));
        assert_eq!(buf.data.len(), 64);
    }

    #[test]
    fn filled_buffer_has_correct_color() {
        let buf = PixelBuffer::filled(2, 2, 255, 128, 0, 255);
        assert_eq!(buf.pixel(0, 0), (255, 128, 0, 255));
        assert_eq!(buf.pixel(1, 1), (255, 128, 0, 255));
    }

    #[test]
    fn set_and_get_pixel() {
        let mut buf = PixelBuffer::new(4, 4);
        buf.set_pixel(2, 3, 10, 20, 30, 40);
        assert_eq!(buf.pixel(2, 3), (10, 20, 30, 40));
    }

    #[test]
    fn out_of_bounds_returns_transparent() {
        let buf = PixelBuffer::new(4, 4);
        assert_eq!(buf.pixel(10, 10), (0, 0, 0, 0));
    }

    #[test]
    fn extract_and_paste() {
        let src = PixelBuffer::filled(4, 4, 255, 0, 0, 255);
        let region = src.extract_rect(1, 1, 2, 2);
        assert_eq!(region.pixel(0, 0), (255, 0, 0, 255));

        let mut dst = PixelBuffer::new(4, 4);
        dst.paste(&region, 0, 0);
        assert_eq!(dst.pixel(0, 0), (255, 0, 0, 255));
        assert_eq!(dst.pixel(2, 2), (0, 0, 0, 0));
    }

    #[test]
    fn clear_resets_to_transparent() {
        let mut buf = PixelBuffer::filled(2, 2, 255, 255, 255, 255);
        buf.clear();
        assert_eq!(buf.pixel(0, 0), (0, 0, 0, 0));
    }

    #[test]
    fn flood_fill_fills_connected_region() {
        let mut buf = PixelBuffer::new(10, 10);
        // Fill from center — entire buffer is same color (transparent) so all fills
        buf.flood_fill(5, 5, 255, 0, 0, 255, 0);
        assert_eq!(buf.pixel(5, 5), (255, 0, 0, 255));
        assert_eq!(buf.pixel(0, 0), (255, 0, 0, 255));
    }

    #[test]
    fn flood_fill_respects_tolerance() {
        let mut buf = PixelBuffer::new(10, 10);
        // Fill entire buffer with a base color
        for y in 0..10 {
            for x in 0..10 {
                buf.set_pixel(x, y, 10, 10, 10, 255);
            }
        }
        // Set a pixel that is slightly different — within tolerance
        buf.set_pixel(5, 5, 30, 10, 10, 255);
        // Tolerance 32 should cover (30,10,10) vs (10,10,10)
        buf.flood_fill(0, 0, 255, 0, 0, 255, 32);
        assert_eq!(buf.pixel(5, 5), (255, 0, 0, 255));
    }

    #[test]
    fn draw_rect_fills_area() {
        let mut buf = PixelBuffer::new(20, 20);
        buf.draw_rect(5, 5, 10, 10, 255, 128, 0, 255);
        assert_eq!(buf.pixel(5, 5), (255, 128, 0, 255));
        assert_eq!(buf.pixel(14, 14), (255, 128, 0, 255));
        assert_eq!(buf.pixel(4, 5), (0, 0, 0, 0)); // Outside
    }

    #[test]
    fn draw_ellipse_fills_region() {
        let mut buf = PixelBuffer::new(20, 20);
        buf.draw_ellipse(10.0, 10.0, 5.0, 5.0, 255, 0, 0, 255);
        assert_eq!(buf.pixel(10, 10), (255, 0, 0, 255)); // Center
        assert_eq!(buf.pixel(0, 0), (0, 0, 0, 0)); // Outside
    }

    #[test]
    fn draw_line_connects_points() {
        let mut buf = PixelBuffer::new(20, 20);
        buf.draw_line(0, 0, 19, 0, 255, 255, 255, 255, 1);
        assert_eq!(buf.pixel(10, 0), (255, 255, 255, 255));
    }

    #[test]
    fn render_text_draws_pixels() {
        let mut buf = PixelBuffer::new(100, 20);
        buf.render_text("Hi", 0, 0, 255, 255, 255, 8);
        // Should have some non-zero pixels
        let has_pixels = buf.data.iter().any(|&b| b > 0);
        assert!(has_pixels);
    }
}
