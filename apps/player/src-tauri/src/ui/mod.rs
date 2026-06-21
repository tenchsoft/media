//! Player app UI - PotPlayer-style media player.
//!
//! Uses the ClickRegion pattern from the View product:
//! - `register_click(rect, ClickAction)` during paint
//! - `click_action_at(x, y)` on pointer down

pub mod ai_panel;
mod app;
pub mod controls;
pub mod paint_controls;
pub mod paint_overlays;
pub mod paint_panels;
pub mod paint_video;
pub mod state;
pub mod theme;
pub mod video_surface;

pub use app::PlayerApp;
