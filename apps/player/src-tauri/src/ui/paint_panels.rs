//! Drawer panel rendering: AI assistant panel, playlist, chapters, subtitles, info tabs.

mod ai;
mod chapters;
mod drawer;
mod info;
mod playlist;
mod subtitles;

pub use ai::paint_ai_panel;
pub use drawer::paint_drawer;
