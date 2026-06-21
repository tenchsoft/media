use super::*;

impl PixelDesignState {
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + 25).min(3200);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = self.zoom.saturating_sub(25).max(10);
    }

    pub fn zoom_fit(&mut self) {
        self.zoom = 100;
        self.viewport_offset_x = 0.0;
        self.viewport_offset_y = 0.0;
        self.status_msg = "Fit to screen".into();
    }

    pub fn zoom_actual(&mut self) {
        self.zoom = 100;
        self.viewport_offset_x = 0.0;
        self.viewport_offset_y = 0.0;
        self.status_msg = "100% zoom".into();
    }

    // Phase 7: Flip/rotate
    pub fn flip_canvas_horizontal(&mut self) {
        self.canvas_flipped_h = !self.canvas_flipped_h;
        self.status_msg = "Canvas flipped horizontal".into();
    }

    pub fn flip_canvas_vertical(&mut self) {
        self.canvas_flipped_v = !self.canvas_flipped_v;
        self.status_msg = "Canvas flipped vertical".into();
    }

    pub fn rotate_canvas_90cw(&mut self) {
        self.canvas_rotation = (self.canvas_rotation + 90.0) % 360.0;
        self.status_msg = "Canvas rotated 90 CW".into();
    }

    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
        self.status_msg = if self.show_grid {
            "Grid on"
        } else {
            "Grid off"
        }
        .into();
    }

    pub fn toggle_rulers(&mut self) {
        self.show_rulers = !self.show_rulers;
        self.status_msg = if self.show_rulers {
            "Rulers on"
        } else {
            "Rulers off"
        }
        .into();
    }

    pub fn toggle_panels(&mut self) {
        self.panels_visible = !self.panels_visible;
    }
}
