//! Overlay rendering: GIF capture modal, toast, context menu, help, URL input,
//! subtitle style/search, GIF options, equalizer, add chapter, PiP indicator.

mod chapter;
mod context_menu;
mod equalizer;
mod gif_capture;
mod gif_options;
mod help;
mod modals;
mod pip;
mod subtitle_search;
mod subtitle_style;
mod toast;
mod url;

pub use context_menu::paint_context_menu;
pub use gif_capture::paint_gif_capture_modal;
pub use modals::paint_modals;
pub use pip::paint_pip_indicator;
pub use toast::paint_toast;
