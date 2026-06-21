//! Side panels - metadata drawer, histogram, filter panel, AI panel.
//!
//! Matches the React `MetadataDrawer`, `Histogram`, `FilterPanel`, `AiPanel` components.

use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::state::{bytes_label, AiFeature, AnnotationTool, ClickAction, CompareMode, ViewState};
use super::theme::{
    ACCENT_VIEW, BORDER_COLOR, BTN_BG, ERROR_COLOR, INPUT_BG, PANEL_BG, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};

mod ai;
mod compare;
mod file_info;
mod filter;
mod help;
mod metadata;
mod quick_edit;
mod rects;
mod settings;

pub use ai::paint_ai_panel;
pub use compare::paint_compare_panel;
pub use file_info::paint_file_info_overlay;
pub use filter::paint_filter_panel;
pub use help::paint_help_overlay;
pub use metadata::paint_metadata_drawer;
pub use quick_edit::paint_quick_edit_overlay;
pub use rects::{
    ai_panel_button_rects, compare_panel_button_rects, file_info_overlay_button_rects,
    filter_panel_button_rects, help_overlay_button_rects, metadata_drawer_button_rects,
    quick_edit_overlay_button_rects,
};
pub use settings::paint_settings_panel;
