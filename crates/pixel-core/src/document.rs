//! Document: a multi-layer pixel document with compositing.

use crate::blend::BlendMode;
use crate::buffer::PixelBuffer;
use crate::filter::Filter;
use crate::selection::Selection;

/// A single layer in a pixel document.
#[derive(Clone, Debug)]
pub struct DocumentLayer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub buffer: PixelBuffer,
    pub mask: Option<Selection>,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl DocumentLayer {
    /// Creates a new transparent layer.
    pub fn new(id: impl Into<String>, name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            buffer: PixelBuffer::new(width, height),
            mask: None,
            offset_x: 0,
            offset_y: 0,
        }
    }

    /// Creates a layer from an existing pixel buffer.
    pub fn from_buffer(
        id: impl Into<String>,
        name: impl Into<String>,
        buffer: PixelBuffer,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            buffer,
            mask: None,
            offset_x: 0,
            offset_y: 0,
        }
    }

    /// Applies a filter to this layer's buffer.
    pub fn apply_filter(&mut self, filter: &Filter) {
        filter.apply(&mut self.buffer);
    }
}

/// A multi-layer pixel document.
#[derive(Clone, Debug)]
pub struct Document {
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub layers: Vec<DocumentLayer>,
    pub active_layer_id: String,
    pub selection: Option<Selection>,
    pub file_path: Option<String>,
    pub dirty: bool,
}

impl Document {
    /// Creates a new empty document with a single background layer.
    pub fn new(width: u32, height: u32) -> Self {
        let bg = DocumentLayer::new("bg", "Background", width, height);
        Self {
            width,
            height,
            name: "untitled.png".to_string(),
            layers: vec![bg],
            active_layer_id: "bg".to_string(),
            selection: None,
            file_path: None,
            dirty: false,
        }
    }

    /// Creates a document from a loaded image, placing it as the background layer.
    pub fn from_image(image: image::DynamicImage, path: Option<String>) -> Self {
        let width = image.width();
        let height = image.height();
        let buffer = PixelBuffer::from_dynamic_image(&image);
        let bg = DocumentLayer::from_buffer("bg", "Background", buffer);
        let name = path
            .as_ref()
            .and_then(|p| {
                std::path::Path::new(p)
                    .file_name()?
                    .to_str()
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "untitled.png".to_string());
        Self {
            width,
            height,
            name,
            layers: vec![bg],
            active_layer_id: "bg".to_string(),
            selection: None,
            file_path: path,
            dirty: false,
        }
    }

    /// Returns the active layer index.
    pub fn active_layer_index(&self) -> usize {
        self.layers
            .iter()
            .position(|l| l.id == self.active_layer_id)
            .unwrap_or(0)
    }

    /// Returns a reference to the active layer.
    pub fn active_layer(&self) -> Option<&DocumentLayer> {
        self.layers.iter().find(|l| l.id == self.active_layer_id)
    }

    /// Returns a mutable reference to the active layer.
    pub fn active_layer_mut(&mut self) -> Option<&mut DocumentLayer> {
        self.layers
            .iter_mut()
            .find(|l| l.id == self.active_layer_id)
    }

    /// Adds a new empty layer on top.
    pub fn add_layer(&mut self, name: impl Into<String>) -> &DocumentLayer {
        let id = format!("layer_{}", self.layers.len());
        let layer = DocumentLayer::new(&id, name, self.width, self.height);
        self.layers.push(layer);
        self.active_layer_id = id.clone();
        self.dirty = true;
        self.layers.last().unwrap()
    }

    /// Removes a layer by ID. Cannot remove the last layer.
    pub fn remove_layer(&mut self, id: &str) -> bool {
        if self.layers.len() <= 1 {
            return false;
        }
        let idx = match self.layers.iter().position(|l| l.id == id) {
            Some(i) => i,
            None => return false,
        };
        self.layers.remove(idx);
        if self.active_layer_id == id {
            let new_idx = idx.min(self.layers.len() - 1);
            self.active_layer_id = self.layers[new_idx].id.clone();
        }
        self.dirty = true;
        true
    }

    /// Moves a layer up in the stack.
    pub fn move_layer_up(&mut self, id: &str) {
        let idx = match self.layers.iter().position(|l| l.id == id) {
            Some(i) => i,
            None => return,
        };
        if idx < self.layers.len() - 1 {
            self.layers.swap(idx, idx + 1);
            self.dirty = true;
        }
    }

    /// Moves a layer down in the stack.
    pub fn move_layer_down(&mut self, id: &str) {
        let idx = match self.layers.iter().position(|l| l.id == id) {
            Some(i) => i,
            None => return,
        };
        if idx > 0 {
            self.layers.swap(idx, idx - 1);
            self.dirty = true;
        }
    }

    /// Flattens all visible layers into a single buffer.
    pub fn flatten(&self) -> PixelBuffer {
        let mut result = PixelBuffer::new(self.width, self.height);
        for layer in &self.layers {
            if !layer.visible {
                continue;
            }
            blend_layer_on(&mut result, layer);
        }
        result
    }

    /// Flattens layers up to and including the given layer index.
    pub fn flatten_up_to(&self, layer_index: usize) -> PixelBuffer {
        let mut result = PixelBuffer::new(self.width, self.height);
        for layer in self.layers.iter().take(layer_index + 1) {
            if !layer.visible {
                continue;
            }
            blend_layer_on(&mut result, layer);
        }
        result
    }

    /// Resizes the document (all layers).
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        for layer in &mut self.layers {
            let img = layer.buffer.to_dynamic_image();
            let resized =
                img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
            layer.buffer = PixelBuffer::from_dynamic_image(&resized);
        }
        self.width = new_width;
        self.height = new_height;
        self.dirty = true;
    }

    /// Crops the document to the given rectangle.
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) {
        for layer in &mut self.layers {
            let img = layer.buffer.to_dynamic_image();
            let cropped = img.crop_imm(x, y, w, h);
            layer.buffer = PixelBuffer::from_dynamic_image(&cropped);
        }
        self.width = w;
        self.height = h;
        self.dirty = true;
    }
}

/// Blends a single layer onto a destination buffer.
fn blend_layer_on(dst: &mut PixelBuffer, layer: &DocumentLayer) {
    let opacity = layer.opacity;
    let blend_mode = layer.blend_mode;
    let src = &layer.buffer;

    // Calculate the region to blend (handle layer offset)
    let src_x_start = if layer.offset_x > 0 {
        0u32
    } else {
        (-layer.offset_x) as u32
    };
    let src_y_start = if layer.offset_y > 0 {
        0u32
    } else {
        (-layer.offset_y) as u32
    };
    let dst_x_start = if layer.offset_x > 0 {
        layer.offset_x as u32
    } else {
        0u32
    };
    let dst_y_start = if layer.offset_y > 0 {
        layer.offset_y as u32
    } else {
        0u32
    };

    let copy_w = src
        .width
        .saturating_sub(src_x_start)
        .min(dst.width.saturating_sub(dst_x_start));
    let copy_h = src
        .height
        .saturating_sub(src_y_start)
        .min(dst.height.saturating_sub(dst_y_start));

    for dy in 0..copy_h {
        for dx in 0..copy_w {
            let sx = src_x_start + dx;
            let sy = src_y_start + dy;
            let dxp = dst_x_start + dx;
            let dyp = dst_y_start + dy;

            let src_pixel = src.pixel(sx, sy);
            let dst_pixel = dst.pixel(dxp, dyp);

            // Apply layer opacity
            let adjusted_src = (
                src_pixel.0,
                src_pixel.1,
                src_pixel.2,
                ((src_pixel.3 as f32 * opacity).round() as u8),
            );

            // Check mask
            if let Some(ref mask) = layer.mask {
                if !mask.is_selected(dxp, dyp) {
                    continue;
                }
            }

            let blended = blend_mode.blend_pixel(adjusted_src, dst_pixel);
            dst.set_pixel(dxp, dyp, blended.0, blended.1, blended.2, blended.3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document_has_background_layer() {
        let doc = Document::new(100, 100);
        assert_eq!(doc.layers.len(), 1);
        assert_eq!(doc.layers[0].name, "Background");
        assert_eq!(doc.width, 100);
        assert_eq!(doc.height, 100);
    }

    #[test]
    fn add_and_remove_layer() {
        let mut doc = Document::new(100, 100);
        doc.add_layer("Layer 1");
        assert_eq!(doc.layers.len(), 2);
        assert_eq!(doc.active_layer().unwrap().name, "Layer 1");

        let id = doc.active_layer_id.clone();
        assert!(doc.remove_layer(&id));
        assert_eq!(doc.layers.len(), 1);
    }

    #[test]
    fn cannot_remove_last_layer() {
        let mut doc = Document::new(100, 100);
        assert!(!doc.remove_layer("bg"));
    }

    #[test]
    fn flatten_two_layers() {
        let mut doc = Document::new(2, 2);
        doc.layers[0].buffer = PixelBuffer::filled(2, 2, 255, 0, 0, 255);

        doc.add_layer("Blue");
        let mut buf = PixelBuffer::new(2, 2);
        buf.set_pixel(0, 0, 0, 0, 255, 128); // Semi-transparent blue
                                             // We need to get the mutable layer back
        doc.active_layer_mut().unwrap().buffer = buf;

        let flat = doc.flatten();
        // Top-left should blend blue over red
        let (_r, _g, b, _a) = flat.pixel(0, 0);
        assert!(b > 0, "Blue channel should be present after blending");
        // Bottom-right should be red from background (top layer is transparent)
        let (r2, _g2, _b2, _a2) = flat.pixel(1, 1);
        assert_eq!(
            r2, 255,
            "Background red should show through transparent top layer"
        );
    }

    #[test]
    fn move_layer_order() {
        let mut doc = Document::new(10, 10);
        doc.add_layer("A");
        doc.add_layer("B");
        assert_eq!(doc.layers[1].name, "A");
        assert_eq!(doc.layers[2].name, "B");

        let layer_id = doc.layers[1].id.clone();
        doc.move_layer_up(&layer_id);
        assert_eq!(doc.layers[1].name, "B");
        assert_eq!(doc.layers[2].name, "A");
    }

    #[test]
    fn from_image_creates_document() {
        let img = image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            50,
            40,
            image::Rgba([255, 128, 0, 255]),
        ));
        let doc = Document::from_image(img, Some("test.png".into()));
        assert_eq!(doc.width, 50);
        assert_eq!(doc.height, 40);
        assert_eq!(doc.file_path, Some("test.png".into()));
        let (r, g, b, a) = doc.layers[0].buffer.pixel(0, 0);
        assert_eq!(r, 255);
        assert_eq!(g, 128);
        assert_eq!(b, 0);
        assert_eq!(a, 255);
    }
}
