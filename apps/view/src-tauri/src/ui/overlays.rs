//! Top and bottom overlays - toolbar and status bar.
//!
//! Matches the React `TopOverlay` and `BottomOverlay` components.

use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::state::{bytes_label, ClickAction, FitMode, ViewState};
use super::theme::{
    ACCENT_VIEW, BORDER_COLOR, BTN_BG, OVERLAY_BG, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

use super::image_stage;
use super::state;

mod annotations;
mod bottom;
mod controls;
mod dialogs;
mod empty;
mod nav;
mod rects;

pub use annotations::{paint_annotation_color_picker, paint_annotations_overlay};
pub use bottom::paint_bottom_overlay;
pub use controls::{bottom_overlay_button_rects, paint_top_overlay, top_overlay_button_rects};
pub use dialogs::{paint_print_dialog, paint_url_dialog};
pub use empty::{overlay_empty_button_rects, paint_empty_state};
pub use nav::{nav_edge_button_rects, paint_nav_edges};
pub use rects::{print_dialog_button_rects, url_dialog_button_rects};
