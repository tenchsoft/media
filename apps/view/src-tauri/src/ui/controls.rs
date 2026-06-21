//! Controls - context menu, slideshow controls, batch trigger.
//!
//! Matches the React `ContextMenu`, `SlideshowControls`, `BatchPanel` trigger components.

mod batch;
mod context_menu;
mod hit_test;
mod overlays;
mod slideshow;

pub use batch::paint_batch_panel;
pub use context_menu::{context_menu_item_rects, paint_context_menu};
pub use hit_test::{
    batch_panel_button_rects, batch_trigger_button_rect, delete_confirm_button_rects,
    edit_banner_button_rects, rename_dialog_button_rects,
};
pub use overlays::{
    paint_batch_trigger, paint_delete_confirm, paint_edit_banner, paint_hamburger_menu,
    paint_loading_overlay, paint_rename_dialog,
};
pub use slideshow::{paint_slideshow_controls, slideshow_controls_button_rects};
