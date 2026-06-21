//! Selection: rectangular, elliptical, and mask-based selections.

/// A selection defines which pixels are affected by an operation.
#[derive(Clone, Debug)]
pub enum Selection {
    /// No selection (everything is selected).
    None,
    /// Rectangular selection.
    Rect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    /// Elliptical selection within a bounding rect.
    Ellipse { cx: f64, cy: f64, rx: f64, ry: f64 },
    /// Pixel mask where each byte is 0 (unselected) or 255 (selected).
    Mask {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
}

impl Selection {
    /// Returns true if the pixel at (x, y) is selected.
    pub fn is_selected(&self, x: u32, y: u32) -> bool {
        match self {
            Selection::None => true,
            Selection::Rect {
                x: rx,
                y: ry,
                width,
                height,
            } => x >= *rx && y >= *ry && x < rx + width && y < ry + height,
            Selection::Ellipse { cx, cy, rx, ry } => {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry) <= 1.0
            }
            Selection::Mask { width, data, .. } => {
                if x >= *width {
                    return false;
                }
                let idx = (y * width + x) as usize;
                data.get(idx).is_some_and(|&v| v > 0)
            }
        }
    }

    /// Returns the bounding rectangle of the selection.
    pub fn bounding_rect(&self) -> Option<(u32, u32, u32, u32)> {
        match self {
            Selection::None => None,
            Selection::Rect {
                x,
                y,
                width,
                height,
            } => Some((*x, *y, *width, *height)),
            Selection::Ellipse { cx, cy, rx, ry } => {
                let x = (*cx - *rx).max(0.0) as u32;
                let y = (*cy - *ry).max(0.0) as u32;
                let w = (*cx + *rx).ceil() as u32 - x;
                let h = (*cy + *ry).ceil() as u32 - y;
                Some((x, y, w, h))
            }
            Selection::Mask { width, height, .. } => Some((0, 0, *width, *height)),
        }
    }

    /// Inverts the selection.
    pub fn invert(&mut self) {
        if let Selection::Mask { data, .. } = self {
            for byte in data.iter_mut() {
                *byte = 255 - *byte;
            }
        }
        // For Rect/Ellipse, inversion would require a mask — leave as-is for now.
    }

    /// Creates a rectangular selection.
    pub fn rect(x: u32, y: u32, width: u32, height: u32) -> Self {
        Selection::Rect {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates an elliptical selection.
    pub fn ellipse(cx: f64, cy: f64, rx: f64, ry: f64) -> Self {
        Selection::Ellipse { cx, cy, rx, ry }
    }
}

/// Alias for Selection when used as a layer mask.
pub type Mask = Selection;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_selects_everything() {
        let sel = Selection::None;
        assert!(sel.is_selected(0, 0));
        assert!(sel.is_selected(999, 999));
    }

    #[test]
    fn rect_selection() {
        let sel = Selection::rect(10, 20, 30, 40);
        assert!(sel.is_selected(10, 20));
        assert!(sel.is_selected(39, 59));
        assert!(!sel.is_selected(9, 20));
        assert!(!sel.is_selected(40, 20));
    }

    #[test]
    fn ellipse_selection() {
        let sel = Selection::ellipse(50.0, 50.0, 20.0, 20.0);
        assert!(sel.is_selected(50, 50)); // center
        assert!(!sel.is_selected(80, 50)); // outside
    }

    #[test]
    fn mask_selection() {
        let mut data = vec![0u8; 4];
        data[0] = 255;
        let sel = Selection::Mask {
            width: 2,
            height: 2,
            data,
        };
        assert!(sel.is_selected(0, 0));
        assert!(!sel.is_selected(1, 0));
    }

    #[test]
    fn bounding_rect() {
        let sel = Selection::rect(5, 10, 20, 30);
        assert_eq!(sel.bounding_rect(), Some((5, 10, 20, 30)));
    }
}
