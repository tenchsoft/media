use tench_ui::prelude::*;

use super::super::state::{ClickAction, ViewState};

// ---------------------------------------------------------------------------
// Rect computation helpers (automation fallback)
// ---------------------------------------------------------------------------

/// Computes batch panel button rects without rendering.
pub fn batch_panel_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_batch {
        return Vec::new();
    }
    let mut rects = Vec::new();
    let panel_w = 340.0_f64.min(size.width);
    let x = size.width - panel_w;
    let pad = 16.0;

    // Close button
    let close_x = size.width - pad - 40.0;
    rects.push((
        ClickAction::ToggleBatch,
        Rect::new(close_x, 16.0, close_x + 40.0, 44.0),
    ));

    let mut y = 56.0;
    let mode_w = 80.0;
    let mode_gap = 8.0;

    // Resize mode button
    rects.push((
        ClickAction::BatchModeResize,
        Rect::new(x + pad, y, x + pad + mode_w, y + 28.0),
    ));
    // Convert mode button
    rects.push((
        ClickAction::BatchModeConvert,
        Rect::new(
            x + pad + mode_w + mode_gap,
            y,
            x + pad + mode_w * 2.0 + mode_gap,
            y + 28.0,
        ),
    ));

    y += 40.0;

    // Format buttons (convert mode only)
    if !state.batch_mode_resize {
        let formats = ["png", "jpg", "webp"];
        let mut fx = x + pad;
        for fmt in &formats {
            rects.push((
                ClickAction::BatchSelectFormat(fmt.to_string()),
                Rect::new(fx, y, fx + 48.0, y + 28.0),
            ));
            fx += 56.0;
        }
    }

    y += 56.0;

    // Select all
    rects.push((
        ClickAction::BatchToggleSelectAll,
        Rect::new(x + pad, y, x + panel_w - pad, y + 28.0),
    ));

    y += 36.0;

    // File entries
    let visible_count = state.sorted_entries.len().min(20);
    for i in 0..visible_count {
        rects.push((
            ClickAction::BatchToggleFile(i),
            Rect::new(x + pad, y, x + panel_w - pad, y + 22.0),
        ));
        y += 22.0;
    }

    // Output browse
    let apply_y = size.height - 52.0;
    let output_y = apply_y - 38.0;
    let browse_x = x + pad + 160.0;
    rects.push((
        ClickAction::BatchBrowseOutput,
        Rect::new(browse_x, output_y, browse_x + 50.0, output_y + 20.0),
    ));

    // Apply button
    let apply_w = panel_w - pad * 2.0;
    rects.push((
        ClickAction::BatchApply,
        Rect::new(x + pad, apply_y, x + pad + apply_w, apply_y + 36.0),
    ));

    rects
}

/// Computes delete confirm dialog button rects without rendering.
pub fn delete_confirm_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_delete_confirm {
        return Vec::new();
    }
    let dialog_w = 360.0;
    let dialog_h = 60.0;
    let dialog_x = (size.width - dialog_w) / 2.0;
    let dialog_y = (size.height - dialog_h) / 2.0;

    let cancel_x = dialog_x + dialog_w - 180.0;
    let delete_x = dialog_x + dialog_w - 90.0;

    vec![
        (
            ClickAction::DeleteCancel,
            Rect::new(
                cancel_x,
                dialog_y + 14.0,
                cancel_x + 70.0,
                dialog_y + dialog_h - 14.0,
            ),
        ),
        (
            ClickAction::DeleteConfirm,
            Rect::new(
                delete_x,
                dialog_y + 14.0,
                delete_x + 70.0,
                dialog_y + dialog_h - 14.0,
            ),
        ),
    ]
}

/// Computes edit banner button rects without rendering.
pub fn edit_banner_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.has_edited_image {
        return Vec::new();
    }
    let banner_w = 320.0;
    let banner_h = 44.0;
    let banner_x = (size.width - banner_w) / 2.0;
    let banner_y = size.height - 80.0;

    let save_x = banner_x + banner_w - 150.0;
    let discard_x = banner_x + banner_w - 76.0;

    vec![
        (
            ClickAction::EditSave,
            Rect::new(
                save_x,
                banner_y + 8.0,
                save_x + 60.0,
                banner_y + banner_h - 8.0,
            ),
        ),
        (
            ClickAction::EditDiscard,
            Rect::new(
                discard_x,
                banner_y + 8.0,
                discard_x + 60.0,
                banner_y + banner_h - 8.0,
            ),
        ),
    ]
}

/// Computes rename dialog button rects without rendering.
pub fn rename_dialog_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_rename {
        return Vec::new();
    }
    let dialog_w = 380.0_f64.min(size.width - 40.0);
    let dialog_h = 120.0;
    let dialog_x = (size.width - dialog_w) / 2.0;
    let dialog_y = (size.height - dialog_h) / 2.0;

    let cancel_x = dialog_x + dialog_w - 180.0;
    let confirm_x = dialog_x + dialog_w - 90.0;
    let btn_y = dialog_y + dialog_h - 40.0;

    let mut rects = vec![(
        ClickAction::RenameCancel,
        Rect::new(cancel_x, btn_y, cancel_x + 70.0, btn_y + 28.0),
    )];

    // Confirm only registered when valid
    if !state.rename_input_text.is_empty() && state.rename_input_text != state.rename_original_name
    {
        rects.push((
            ClickAction::RenameConfirm,
            Rect::new(confirm_x, btn_y, confirm_x + 70.0, btn_y + 28.0),
        ));
    }

    rects
}

/// Computes batch trigger button rect without rendering.
pub fn batch_trigger_button_rect(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if state.sorted_entries.len() <= 1 || state.show_batch {
        return Vec::new();
    }
    let btn_w = 52.0;
    let btn_h = 24.0;
    let x = 12.0;
    let y = size.height - 12.0 - btn_h;
    vec![(
        ClickAction::OpenBatch,
        Rect::new(x, y, x + btn_w, y + btn_h),
    )]
}
