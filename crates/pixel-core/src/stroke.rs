//! Brush stroke: recorded paint strokes for undo/redo.

/// A single point in a brush stroke.
#[derive(Clone, Debug, Copy)]
pub struct StrokePoint {
    pub x: f64,
    pub y: f64,
    pub pressure: f32,
    pub timestamp_ms: u64,
}

impl StrokePoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            pressure: 1.0,
            timestamp_ms: 0,
        }
    }

    pub fn with_pressure(mut self, pressure: f32) -> Self {
        self.pressure = pressure;
        self
    }

    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp_ms = ts;
        self
    }
}

/// A complete brush stroke.
#[derive(Clone, Debug)]
pub struct BrushStroke {
    pub points: Vec<StrokePoint>,
    pub color: (u8, u8, u8, u8),
    pub size: f64,
    pub opacity: f32,
    pub is_eraser: bool,
}

impl BrushStroke {
    /// Creates a new brush stroke.
    pub fn new(color: (u8, u8, u8, u8), size: f64, opacity: f32, is_eraser: bool) -> Self {
        Self {
            points: Vec::new(),
            color,
            size,
            opacity,
            is_eraser,
        }
    }

    /// Adds a point to the stroke.
    pub fn add_point(&mut self, point: StrokePoint) {
        self.points.push(point);
    }

    /// Rasterizes the stroke onto a pixel buffer.
    ///
    /// Draws filled circles at each point, interpolated between consecutive points.
    pub fn rasterize(&self, buf: &mut crate::buffer::PixelBuffer) {
        if self.points.is_empty() {
            return;
        }

        let radius = self.size / 2.0;

        for i in 0..self.points.len() {
            let point = &self.points[i];
            let effective_radius = radius * point.pressure as f64;

            // Draw a filled circle at this point
            draw_filled_circle(
                buf,
                point.x,
                point.y,
                effective_radius,
                self.color,
                self.opacity,
                self.is_eraser,
            );

            // Interpolate between consecutive points to fill gaps
            if i > 0 {
                let prev = &self.points[i - 1];
                let dx = point.x - prev.x;
                let dy = point.y - prev.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let steps = (dist / (effective_radius * 0.3)).ceil() as usize;

                for step in 1..steps {
                    let t = step as f64 / steps as f64;
                    let ix = prev.x + dx * t;
                    let iy = prev.y + dy * t;
                    draw_filled_circle(
                        buf,
                        ix,
                        iy,
                        effective_radius,
                        self.color,
                        self.opacity,
                        self.is_eraser,
                    );
                }
            }
        }
    }
}

/// Draws a filled circle with anti-aliasing.
fn draw_filled_circle(
    buf: &mut crate::buffer::PixelBuffer,
    cx: f64,
    cy: f64,
    radius: f64,
    color: (u8, u8, u8, u8),
    opacity: f32,
    is_eraser: bool,
) {
    let x0 = (cx - radius - 1.0).max(0.0) as u32;
    let y0 = (cy - radius - 1.0).max(0.0) as u32;
    let x1 = ((cx + radius + 1.0) as u32).min(buf.width - 1);
    let y1 = ((cy + radius + 1.0) as u32).min(buf.height - 1);

    let r_sq = radius * radius;

    for y in y0..=y1 {
        for x in x0..=x1 {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq <= r_sq {
                let alpha = if is_eraser {
                    0 // Erase to transparent
                } else {
                    (color.3 as f32 * opacity) as u8
                };

                if is_eraser {
                    // Erase: reduce alpha
                    let existing = buf.pixel(x, y);
                    let new_alpha = (existing.3 as f32 * (1.0 - opacity)).round() as u8;
                    buf.set_pixel(x, y, existing.0, existing.1, existing.2, new_alpha);
                } else {
                    buf.set_pixel(x, y, color.0, color.1, color.2, alpha);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::PixelBuffer;

    #[test]
    fn stroke_adds_points() {
        let mut stroke = BrushStroke::new((255, 0, 0, 255), 10.0, 1.0, false);
        stroke.add_point(StrokePoint::new(5.0, 5.0));
        stroke.add_point(StrokePoint::new(15.0, 5.0));
        assert_eq!(stroke.points.len(), 2);
    }

    #[test]
    fn rasterize_draws_pixels() {
        let mut buf = PixelBuffer::new(50, 50);
        let mut stroke = BrushStroke::new((255, 0, 0, 255), 10.0, 1.0, false);
        stroke.add_point(StrokePoint::new(25.0, 25.0));
        stroke.rasterize(&mut buf);

        let (r, _g, _b, a) = buf.pixel(25, 25);
        assert_eq!(r, 255);
        assert!(a > 0);
    }

    #[test]
    fn eraser_removes_pixels() {
        let mut buf = PixelBuffer::filled(50, 50, 255, 0, 0, 255);
        let mut stroke = BrushStroke::new((0, 0, 0, 0), 10.0, 1.0, true);
        stroke.add_point(StrokePoint::new(25.0, 25.0));
        stroke.rasterize(&mut buf);

        let (_, _, _, a) = buf.pixel(25, 25);
        assert_eq!(a, 0, "Eraser should make pixels transparent");
    }

    #[test]
    fn empty_stroke_does_nothing() {
        let mut buf = PixelBuffer::new(10, 10);
        let stroke = BrushStroke::new((255, 0, 0, 255), 5.0, 1.0, false);
        stroke.rasterize(&mut buf);
        assert_eq!(buf.pixel(5, 5), (0, 0, 0, 0));
    }
}
