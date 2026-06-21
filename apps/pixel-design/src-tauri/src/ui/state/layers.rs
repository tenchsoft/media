use super::*;
use tench_pixel_core::{DocumentLayer, PixelBuffer};

impl PixelDesignState {
    pub fn add_layer(&mut self) {
        let name = format!("Layer {}", self.document.layers.len() + 1);
        self.document.add_layer(&name);
        self.push_history("Add layer");
        self.status_msg = "Layer added".into();
    }

    pub fn delete_layer(&mut self) {
        if self.document.layers.len() <= 1 {
            self.status_msg = "At least one layer is required".into();
            return;
        }
        let id = self.document.active_layer_id.clone();
        if self.document.remove_layer(&id) {
            self.push_history("Delete layer");
            self.status_msg = "Layer deleted".into();
        }
    }

    pub fn toggle_layer_visibility(&mut self, idx: usize) {
        if let Some(layer) = self.document.layers.get_mut(idx) {
            layer.visible = !layer.visible;
            self.status_msg = format!(
                "{} {}",
                layer.name,
                if layer.visible { "visible" } else { "hidden" }
            );
        }
    }

    pub fn toggle_layer_lock(&mut self, idx: usize) {
        if let Some(layer) = self.document.layers.get_mut(idx) {
            layer.locked = !layer.locked;
            self.status_msg = format!(
                "{} {}",
                layer.name,
                if layer.locked { "locked" } else { "unlocked" }
            );
        }
    }

    pub fn move_layer_up(&mut self, idx: usize) {
        if idx + 1 < self.document.layers.len() {
            let id = self.document.layers[idx].id.clone();
            self.document.move_layer_up(&id);
            self.push_history("Reorder layer");
            self.status_msg = "Layer moved up".into();
        }
    }

    pub fn move_layer_down(&mut self, idx: usize) {
        if idx > 0 {
            let id = self.document.layers[idx].id.clone();
            self.document.move_layer_down(&id);
            self.push_history("Reorder layer");
            self.status_msg = "Layer moved down".into();
        }
    }

    pub fn duplicate_layer(&mut self, idx: usize) {
        if let Some(layer) = self.document.layers.get(idx) {
            let new_name = format!("{} copy", layer.name);
            let new_id = format!("layer_{}", self.document.layers.len());
            let mut new_layer = layer.clone();
            new_layer.id = new_id.clone();
            new_layer.name = new_name;
            self.document.layers.insert(idx + 1, new_layer);
            self.document.active_layer_id = new_id;
            self.document.dirty = true;
            self.push_history("Duplicate layer");
            self.status_msg = "Layer duplicated".into();
        }
    }

    pub fn merge_layer_down(&mut self, idx: usize) {
        if idx == 0 || idx >= self.document.layers.len() {
            return;
        }
        // Get the upper layer's buffer and blend onto lower
        let mut upper = self.document.layers[idx].clone();
        let lower = &mut self.document.layers[idx - 1];
        upper.buffer.paste(
            &lower.buffer,
            upper.offset_x.unsigned_abs(),
            upper.offset_y.unsigned_abs(),
        );
        let upper_id = upper.id.clone();
        self.document.layers.remove(idx);
        if self.document.active_layer_id == upper_id {
            self.document.active_layer_id = self.document.layers[idx - 1].id.clone();
        }
        self.push_history("Merge down");
        self.status_msg = "Layers merged".into();
    }

    pub fn flatten_layers(&mut self) {
        let flat = self.document.flatten();
        let new_layer = DocumentLayer::from_buffer("flat", "Background", flat);
        self.document.layers.clear();
        self.document.layers.push(new_layer);
        self.document.active_layer_id = "flat".into();
        self.document.dirty = true;
        self.push_history("Flatten");
        self.status_msg = "Image flattened".into();
    }

    pub fn rename_layer(&mut self, idx: usize, new_name: &str) {
        if let Some(layer) = self.document.layers.get_mut(idx) {
            layer.name = new_name.to_string();
            self.status_msg = format!("Layer renamed to {}", new_name);
        }
        self.renaming_layer_idx = None;
    }

    pub fn nudge_active_layer_opacity(&mut self, delta: i32) {
        if let Some(layer) = self.document.active_layer_mut() {
            let new_opacity = ((layer.opacity * 100.0) as i32 + delta).clamp(0, 100);
            layer.opacity = new_opacity as f32 / 100.0;
            self.status_msg = format!("Layer opacity: {new_opacity}%");
        }
    }

    pub fn set_layer_opacity(&mut self, opacity: f32) {
        if let Some(layer) = self.document.active_layer_mut() {
            layer.opacity = opacity.clamp(0.0, 1.0);
        }
    }

    /// Generate a thumbnail for the given layer.
    pub fn generate_thumbnail(&mut self, layer_id: &str) {
        let thumb_size = 22u32;
        if let Some(layer) = self.document.layers.iter().find(|l| l.id == layer_id) {
            let buf = &layer.buffer;
            let mut thumb = PixelBuffer::new(thumb_size, thumb_size);
            let sx = buf.width as f64 / thumb_size as f64;
            let sy = buf.height as f64 / thumb_size as f64;
            for ty in 0..thumb_size {
                for tx in 0..thumb_size {
                    let src_x = (tx as f64 * sx) as u32;
                    let src_y = (ty as f64 * sy) as u32;
                    let (r, g, b, a) = buf.pixel(src_x, src_y);
                    thumb.set_pixel(tx, ty, r, g, b, a);
                }
            }
            self.layer_thumbnails.insert(layer_id.to_string(), thumb);
        }
    }

    /// Update thumbnails for all layers.
    pub fn update_all_thumbnails(&mut self) {
        let layer_ids: Vec<String> = self.document.layers.iter().map(|l| l.id.clone()).collect();
        for id in layer_ids {
            self.generate_thumbnail(&id);
        }
    }
}
