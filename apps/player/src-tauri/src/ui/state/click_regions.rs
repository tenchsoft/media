use super::*;
use tench_ui::prelude::{Point, Rect};

impl PlayerState {
    // ── Click region management ──

    /// Clear all click regions (call at start of each paint).
    pub fn clear_click_regions(&mut self) {
        self.click_regions.clear();
    }

    /// Register a clickable region with the given action.
    pub fn register_click(&mut self, rect: Rect, action: ClickAction) {
        self.click_regions.push(ClickRegion { rect, action });
    }

    /// Find the action for a click at the given position.
    /// Returns the last (topmost-painted) matching region.
    pub fn click_action_at(&self, x: f64, y: f64) -> Option<&ClickAction> {
        for region in self.click_regions.iter().rev() {
            if region.rect.contains(Point::new(x, y)) {
                return Some(&region.action);
            }
        }
        None
    }
}
