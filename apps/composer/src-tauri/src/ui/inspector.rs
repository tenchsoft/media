//! Inspector, tab, and media-bin helpers for Composer.

use tench_ui::prelude::{Point, Rect};

/// Inspector tabs in order matching ComposerMode: [Edit, Color, Audio, Deliver].
pub const TABS: [&str; 4] = ["Edit", "Color", "Audio", "Deliver"];
pub const TAB_W: f64 = 55.0;

pub fn media_icon(media: &str) -> &'static str {
    if media.ends_with(".mp4") || media.ends_with(".mov") || media.ends_with(".mkv") {
        "[V]"
    } else if media.ends_with(".mp3") || media.ends_with(".wav") || media.ends_with(".flac") {
        "[A]"
    } else {
        "[I]"
    }
}

pub fn tab_rect(panel_left: f64, tab_y: f64, spacing: f64, index: usize) -> Rect {
    let x = panel_left + spacing + index as f64 * (TAB_W + spacing);
    Rect::new(x, tab_y - 8.0, x + TAB_W, tab_y + 8.0)
}

pub fn hit_test_tab(point: Point, panel_left: f64, tab_y: f64, spacing: f64) -> Option<usize> {
    TABS.iter().enumerate().find_map(|(idx, _)| {
        tab_rect(panel_left, tab_y, spacing, idx)
            .contains(point)
            .then_some(idx)
    })
}

pub fn tab_label(index: usize) -> &'static str {
    TABS.get(index).copied().unwrap_or("Edit")
}

/// Mode-specific empty state messages for the inspector.
pub fn empty_state_message(tab_index: usize) -> &'static str {
    match tab_index {
        0 => "No clip selected. Select a clip to edit properties.",
        1 => "No clip selected. Select a clip to access color grading tools.",
        2 => "No clip selected. Select a clip to adjust audio levels.",
        3 => "Configure export settings below.",
        _ => "No clip selected.",
    }
}
