// ---------------------------------------------------------------------------
// Extra semantic automation nodes
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, FitMode};
use crate::ui::{image_stage, overlays, state};

use super::automation_nodes::make_semantic_node;
use super::automation_status::push_automatic_status_nodes;

/// Builds semantic automation nodes for plan items that share a click action
/// or represent automatic UI behavior instead of a dedicated button action.
pub(super) fn extra_view_automation_nodes(
    state: &state::ViewState,
    size: Size,
) -> Vec<tench_ui::UiAutomationNode> {
    let mut nodes = Vec::new();

    if state.document.is_none() {
        for (action, rect) in image_stage::empty_state_button_rects(state, size) {
            let node = match action {
                ClickAction::OpenFileDialog => {
                    Some(("view.empty.open_image", "Open Image", "button", true))
                }
                ClickAction::OpenFolderDialog => {
                    Some(("view.empty.open_folder", "Open Folder", "button", true))
                }
                ClickAction::OpenArchiveDialog => {
                    Some(("view.empty.open_archive", "Open Archive", "button", true))
                }
                ClickAction::ToggleSearch => {
                    nodes.push(make_semantic_node(
                        "view.empty.search",
                        "button",
                        "Search Files",
                        None,
                        Rect::new(rect.x0, rect.y0, rect.x0 + 36.0, rect.y1),
                        true,
                    ));
                    None
                }
                _ => None,
            };
            if let Some((debug_id, label, role, enabled)) = node {
                nodes.push(make_semantic_node(
                    debug_id, role, label, None, rect, enabled,
                ));
            }
        }

        for (action, rect) in overlays::overlay_empty_button_rects(state, size) {
            let node = match action {
                ClickAction::OpenFileDialog => Some(("view.overlay.open_file", "Open File")),
                ClickAction::OpenFolderDialog => Some(("view.overlay.open_folder", "Open Folder")),
                _ => None,
            };
            if let Some((debug_id, label)) = node {
                nodes.push(make_semantic_node(
                    debug_id, "button", label, None, rect, true,
                ));
            }
        }
    }

    let overlay_h = if state.show_thumbnails && !state.sorted_entries.is_empty() {
        120.0
    } else {
        60.0
    };
    let ctrl_y = size.height - overlay_h + 10.0;
    let btn_w = 52.0;
    let btn_gap = 6.0;
    let controls = 12.0;
    let current_zoom_x =
        (size.width - controls * (btn_w + btn_gap)) / 2.0 + 3.0 * (btn_w + btn_gap);
    nodes.push(make_semantic_node(
        "view.bottom.current_zoom",
        "button",
        "Current Zoom",
        Some(match state.fit_mode {
            FitMode::Fit => "Fit".to_string(),
            FitMode::Actual => format!("{:.0}%", state.zoom * 100.0),
        }),
        Rect::new(
            current_zoom_x,
            ctrl_y,
            current_zoom_x + btn_w,
            ctrl_y + 28.0,
        ),
        true,
    ));

    if state.show_metadata {
        let pad = 18.0;
        let close_x = size.width - pad - 40.0;
        nodes.push(make_semantic_node(
            "view.metadata.close",
            "button",
            "Close Metadata",
            None,
            Rect::new(close_x, 20.0, close_x + 40.0, 48.0),
            true,
        ));
    }

    if state.show_file_info && state.document.is_some() {
        let panel_w = 360.0_f64.min(size.width - 40.0);
        let panel_x = (size.width - panel_w) / 2.0;
        let panel_y = size.height - 100.0 - 220.0;
        let pad = 16.0;
        let close_x = panel_x + panel_w - pad - 24.0;
        nodes.push(make_semantic_node(
            "view.file_info.close",
            "button",
            "Close File Info",
            None,
            Rect::new(close_x, panel_y + pad, close_x + 24.0, panel_y + pad + 24.0),
            true,
        ));
    }

    if state.show_quick_edit {
        let panel_w = 260.0_f64.min(size.width - 44.0);
        let panel_x = size.width - panel_w - 22.0;
        let panel_y = 80.0;
        let pad = 14.0;
        let close_x = panel_x + panel_w - pad - 40.0;
        nodes.push(make_semantic_node(
            "view.quick_edit.close",
            "button",
            "Close Quick Edit",
            None,
            Rect::new(close_x, panel_y + pad, close_x + 40.0, panel_y + pad + 28.0),
            true,
        ));

        if state.active_annotation_tool.is_some() {
            let y = panel_y
                + pad
                + 40.0
                + 6.0 * (36.0 + 6.0)
                + 4.0
                + 10.0
                + 36.0
                + 10.0
                + 10.0
                + 22.0
                + 32.0;
            let swatch_rect = Rect::new(
                panel_x + pad + 42.0,
                y + 2.0,
                panel_x + pad + 58.0,
                y + 18.0,
            );
            nodes.push(make_semantic_node(
                "view.quick_edit.color_swatch",
                "button",
                "Annotation Color Swatch",
                Some(format!(
                    "R:{} G:{} B:{}",
                    state.annotation_color.r(),
                    state.annotation_color.g(),
                    state.annotation_color.b()
                )),
                swatch_rect,
                true,
            ));
        }
    }

    if state.show_batch && state.batch_selected.is_empty() && !state.batch_running {
        let panel_w = 340.0_f64.min(size.width);
        let x = size.width - panel_w;
        let pad = 16.0;
        let apply_y = size.height - 52.0;
        nodes.push(make_semantic_node(
            "view.batch.apply",
            "button",
            "Apply Batch",
            Some("disabled".to_string()),
            Rect::new(x + pad, apply_y, x + panel_w - pad, apply_y + 36.0),
            false,
        ));
    }

    if state.show_filter {
        let panel_w = 280.0_f64.min(size.width - 44.0);
        let panel_x = size.width - panel_w - 22.0;
        let panel_y = 80.0;
        let pad = 16.0;
        let track_x = panel_x + pad + 80.0;
        let track_w = panel_w - pad * 2.0 - 128.0;
        for (idx, (debug_id, label, value)) in [
            (
                "view.filter.brightness",
                "Brightness",
                format!("{:.0}%", state.filter_brightness),
            ),
            (
                "view.filter.contrast",
                "Contrast",
                format!("{:.0}%", state.filter_contrast),
            ),
            (
                "view.filter.saturation",
                "Saturation",
                format!("{:.0}%", state.filter_saturation),
            ),
            (
                "view.filter.blur",
                "Blur",
                format!("{:.0}px", state.filter_blur),
            ),
            (
                "view.filter.hue_rotate",
                "Hue Rotate",
                format!("{:.0}", state.filter_hue_rotate),
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let track_y = panel_y + pad + 30.0 + idx as f64 * 40.0 + 4.0;
            nodes.push(make_semantic_node(
                debug_id,
                "slider",
                label,
                Some(value),
                Rect::new(track_x, track_y, track_x + track_w, track_y + 6.0),
                true,
            ));
        }
    }

    if state.show_context_menu {
        nodes.push(make_semantic_node(
            "view.ctx.dismiss",
            "button",
            "Dismiss Context Menu",
            None,
            Rect::new(16.0, size.height / 2.0, 96.0, size.height / 2.0 + 48.0),
            true,
        ));
    }

    // Settings panel nodes
    if state.show_settings {
        let panel_w = 400.0_f64.min(size.width - 40.0);
        let panel_x = (size.width - panel_w) / 2.0;
        let panel_y = 80.0;
        let pad = 16.0;

        // Close button
        let close_x = panel_x + panel_w - pad - 40.0;
        nodes.push(make_semantic_node(
            "view.settings.close",
            "button",
            "Close Settings",
            None,
            Rect::new(close_x, panel_y + pad, close_x + 40.0, panel_y + pad + 28.0),
            true,
        ));

        // Tab buttons
        let tab_w = (panel_w - pad * 2.0 - 6.0 * 3.0) / 4.0;
        let tab_y = panel_y + pad + 36.0;
        let tabs: &[(&str, &str)] = &[
            ("view.settings.tab.general", "General"),
            ("view.settings.tab.image", "Image"),
            ("view.settings.tab.slideshow", "Slideshow"),
            ("view.settings.tab.files", "Files"),
        ];
        for (i, (debug_id, label)) in tabs.iter().enumerate() {
            let tx = panel_x + pad + i as f64 * (tab_w + 6.0);
            nodes.push(make_semantic_node(
                *debug_id,
                "tab",
                *label,
                None,
                Rect::new(tx, tab_y, tx + tab_w, tab_y + 28.0),
                true,
            ));
        }
    }

    // Hamburger menu dismiss node
    if state.show_menu {
        let menu_btn_size = 28.0;
        let menu_btn_x = 6.0;
        let menu_btn_y = 18.0;
        let menu_w = 180.0;
        let menu_y = menu_btn_y + menu_btn_size + 4.0;
        let item_h = 28.0;
        let menu_items_count = 7usize;
        let menu_h = menu_items_count as f64 * item_h + 8.0;

        // Dismiss overlay (click outside menu to close)
        nodes.push(make_semantic_node(
            "view.dismiss",
            "button",
            "Dismiss Menu",
            None,
            Rect::new(menu_btn_x + menu_w, menu_y, size.width, menu_y + menu_h),
            true,
        ));
    }

    push_automatic_status_nodes(state, size, &mut nodes);

    nodes
}
