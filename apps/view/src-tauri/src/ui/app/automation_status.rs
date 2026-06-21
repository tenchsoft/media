// ---------------------------------------------------------------------------
// Automatic behavior status automation nodes
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state;

use super::automation_nodes::make_semantic_node;

pub(super) fn push_automatic_status_nodes(
    state: &state::ViewState,
    size: Size,
    nodes: &mut Vec<tench_ui::UiAutomationNode>,
) {
    let full = Rect::from_origin_size((0.0, 0.0), size);
    for (debug_id, label, value) in [
        (
            "view.automatic.loading_overlay",
            "Loading Overlay",
            Some(state.is_loading.to_string()),
        ),
        (
            "view.automatic.slideshow_auto_advance",
            "Slideshow Auto Advance",
            Some(state.slideshow_playing.to_string()),
        ),
        (
            "view.automatic.slideshow_transition",
            "Slideshow Transition",
            Some(state.slideshow_transition.label().to_string()),
        ),
        (
            "view.automatic.adjacent_prefetch",
            "Adjacent Image Prefetch",
            Some(state.sorted_entries.len().to_string()),
        ),
        (
            "view.automatic.batch_progress",
            "Batch Progress Bar",
            state
                .batch_progress
                .map(|(done, total)| format!("{done}/{total}")),
        ),
        (
            "view.automatic.pixel_hover_info",
            "Pixel Hover Info",
            state.pixel_info.map(|info| {
                format!(
                    "{},{} #{:02X}{:02X}{:02X}",
                    info.x, info.y, info.r, info.g, info.b
                )
            }),
        ),
        (
            "view.automatic.drag_and_drop_open",
            "Drag And Drop Open",
            state.document.as_ref().map(|doc| doc.path.clone()),
        ),
        (
            "view.automatic.image_placeholder",
            "Image Placeholder",
            Some((state.document.is_some() && state.current_image_data.is_none()).to_string()),
        ),
        (
            "view.automatic.annotations_overlay",
            "Annotations Overlay",
            Some(state.annotations.len().to_string()),
        ),
        (
            "view.automatic.status_message_lifecycle",
            "Status Message Lifecycle",
            Some(state.status_message.clone()),
        ),
        (
            "view.automatic.chrome_visibility",
            "Viewer Chrome Visibility",
            Some(state.show_chrome.to_string()),
        ),
        (
            "view.automatic.thumbnail_generation",
            "Thumbnail Generation",
            Some(state.thumbnail_cache.len().to_string()),
        ),
        (
            "view.automatic.thumbnail_virtual_window",
            "Thumbnail Virtual Window",
            Some(state.thumbnail_scroll_offset.to_string()),
        ),
        (
            "view.automatic.image_layout",
            "Image Layout Recompute",
            Some(format!("{:.3}", state.effective_zoom())),
        ),
    ] {
        nodes.push(make_semantic_node(
            debug_id, "status", label, value, full, false,
        ));
    }
}
