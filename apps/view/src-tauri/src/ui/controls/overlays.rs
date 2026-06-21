use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::super::state::{ClickAction, ViewState};
use super::super::theme::{
    ACCENT_VIEW, BORDER_COLOR, BTN_BG, ERROR_COLOR, INPUT_BG, PANEL_BG, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};

// Delete Confirm

/// Draws the delete confirmation dialog.
/// Matches `.delete-confirm` CSS: centered, with error styling.
pub fn paint_delete_confirm(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Dim backdrop
    let backdrop = Rect::from_origin_size((0.0, 0.0), size);
    painter.fill_rect(backdrop, Color::rgba8(0, 0, 0, 128));

    let dialog_w = 360.0;
    let dialog_h = 60.0;
    let dialog_x = (size.width - dialog_w) / 2.0;
    let dialog_y = (size.height - dialog_h) / 2.0;

    let dialog_rect = Rect::new(dialog_x, dialog_y, dialog_x + dialog_w, dialog_y + dialog_h);

    // Error-tinted background
    painter.fill_rounded_rect(dialog_rect, Color::rgba8(0xF3, 0x8B, 0xA8, 38), 8.0);
    painter.stroke_rounded_rect(dialog_rect, Color::rgba8(0xF3, 0x8B, 0xA8, 128), 1.0, 8.0);

    // Title
    painter.draw_text_cached(
        text_cache,
        "Delete this image?",
        dialog_x + 16.0,
        dialog_y + dialog_h / 2.0 + 4.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Cancel button
    let cancel_x = dialog_x + dialog_w - 180.0;
    let cancel_rect = Rect::new(
        cancel_x,
        dialog_y + 14.0,
        cancel_x + 70.0,
        dialog_y + dialog_h - 14.0,
    );
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + 8.0,
        dialog_y + dialog_h / 2.0 + 4.0,
        TEXT_PRIMARY,
        12.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::DeleteCancel);

    // Delete button
    let delete_x = dialog_x + dialog_w - 90.0;
    let delete_rect = Rect::new(
        delete_x,
        dialog_y + 14.0,
        delete_x + 70.0,
        dialog_y + dialog_h - 14.0,
    );
    painter.fill_rounded_rect(delete_rect, ERROR_COLOR, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Delete",
        delete_x + 8.0,
        dialog_y + dialog_h / 2.0 + 4.0,
        Color::WHITE,
        12.0,
        FontWeight::BOLD,
        false,
        false,
    );
    state.register_click(delete_rect, ClickAction::DeleteConfirm);
}

// Edit Banner

/// Draws the unsaved edit banner.
/// Matches `.edit-banner` CSS: centered, bottom area, accent border.
pub fn paint_edit_banner(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let banner_w = 320.0;
    let banner_h = 44.0;
    let banner_x = (size.width - banner_w) / 2.0;
    let banner_y = size.height - 80.0;

    let banner_rect = Rect::new(banner_x, banner_y, banner_x + banner_w, banner_y + banner_h);

    // Accent-tinted background
    painter.fill_rounded_rect(banner_rect, Color::rgba8(0x60, 0xA5, 0xFA, 38), 8.0);
    painter.stroke_rounded_rect(banner_rect, ACCENT_VIEW, 1.0, 8.0);

    painter.draw_text_cached(
        text_cache,
        "Unsaved edit",
        banner_x + 16.0,
        banner_y + banner_h / 2.0 + 4.0,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Save button
    let save_x = banner_x + banner_w - 150.0;
    let save_rect = Rect::new(
        save_x,
        banner_y + 8.0,
        save_x + 60.0,
        banner_y + banner_h - 8.0,
    );
    painter.fill_rounded_rect(save_rect, ACCENT_VIEW, 4.0);
    let tw = text_cache.measure_text_width("Save", 11.0, FontWeight::BOLD);
    painter.draw_text_cached(
        text_cache,
        "Save",
        save_x + (60.0 - tw) / 2.0,
        banner_y + banner_h / 2.0 + 4.0,
        Color::rgb8(0x0F, 0x0F, 0x0F),
        11.0,
        FontWeight::BOLD,
        false,
        false,
    );
    state.register_click(save_rect, ClickAction::EditSave);

    // Discard button
    let discard_x = banner_x + banner_w - 76.0;
    let discard_rect = Rect::new(
        discard_x,
        banner_y + 8.0,
        discard_x + 60.0,
        banner_y + banner_h - 8.0,
    );
    painter.fill_rounded_rect(discard_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(discard_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Discard",
        discard_x + 4.0,
        banner_y + banner_h / 2.0 + 4.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(discard_rect, ClickAction::EditDiscard);
}

// Loading Overlay

/// Draws the loading spinner overlay.
/// Matches `.loading-overlay` CSS: full-screen, centered spinner.
pub fn paint_loading_overlay(size: Size, scene: &mut Scene) {
    let mut painter = Painter::new(scene);

    // Semi-transparent backdrop
    let backdrop = Rect::from_origin_size((0.0, 0.0), size);
    painter.fill_rect(backdrop, Color::rgba8(0x0F, 0x0F, 0x0F, 153));

    // Spinner circle
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let r = 18.0;

    // Outer ring (track)
    let track_rect = Rect::new(cx - r, cy - r, cx + r, cy + r);
    painter.stroke_rounded_rect(track_rect, Color::rgb8(0x2A, 0x2A, 0x2A), 3.0, r);

    // Spinning arc (approximated with a filled arc segment)
    let arc_rect = Rect::new(cx - r, cy - r, cx + r, cy + r);
    painter.stroke_rounded_rect(arc_rect, ACCENT_VIEW, 3.0, r);
}

// Batch Trigger Button

/// Draws the small batch trigger button at bottom-left.
/// Matches `.batch-trigger` CSS.
pub fn paint_batch_trigger(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let label = "Batch";
    let btn_w = 52.0;
    let btn_h = 24.0;
    let x = 12.0;
    let y = size.height - 12.0 - btn_h;

    let btn_rect = Rect::new(x, y, x + btn_w, y + btn_h);
    painter.fill_rounded_rect(btn_rect, Color::rgba8(0x0F, 0x0F, 0x0F, 150), 4.0);
    let tw = text_cache.measure_text_width(label, 11.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        label,
        x + (btn_w - tw) / 2.0,
        y + btn_h / 2.0 + 3.0,
        Color::rgba8(0xCD, 0xD6, 0xF4, 150),
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    state.register_click(btn_rect, ClickAction::OpenBatch);
}

// Rename Dialog

/// Draws the rename dialog overlay.
/// A centered dialog with a text input field showing the current filename,
/// and Confirm/Cancel buttons.
pub fn paint_rename_dialog(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Dim backdrop
    let backdrop = Rect::from_origin_size((0.0, 0.0), size);
    painter.fill_rect(backdrop, Color::rgba8(0, 0, 0, 128));

    let dialog_w = 380.0_f64.min(size.width - 40.0);
    let dialog_h = 120.0;
    let dialog_x = (size.width - dialog_w) / 2.0;
    let dialog_y = (size.height - dialog_h) / 2.0;

    let dialog_rect = Rect::new(dialog_x, dialog_y, dialog_x + dialog_w, dialog_y + dialog_h);

    // Background
    painter.fill_rounded_rect(dialog_rect, PANEL_BG, 8.0);
    painter.stroke_rounded_rect(dialog_rect, BORDER_COLOR, 1.0, 8.0);

    // Clip to dialog bounds
    painter.push_clip(dialog_rect);

    // Title
    painter.draw_text_cached(
        text_cache,
        "Rename",
        dialog_x + 16.0,
        dialog_y + 20.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Text input field (shows current rename_input_text)
    let input_x = dialog_x + 16.0;
    let input_y = dialog_y + 32.0;
    let input_w = dialog_w - 32.0;
    let input_h = 30.0;
    let input_rect = Rect::new(input_x, input_y, input_x + input_w, input_y + input_h);
    painter.fill_rounded_rect(input_rect, INPUT_BG, 4.0);
    painter.stroke_rounded_rect(input_rect, ACCENT_VIEW, 1.0, 4.0);

    // Display text with cursor
    let display_text = if state.rename_input_text.is_empty() {
        ""
    } else {
        &state.rename_input_text
    };
    painter.draw_text_cached(
        text_cache,
        display_text,
        input_x + 8.0,
        input_y + input_h / 2.0 + 4.0,
        TEXT_PRIMARY,
        13.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Cursor blink indicator (a simple vertical bar after the text)
    let text_width = text_cache.measure_text_width(display_text, 13.0, FontWeight::NORMAL);
    let cursor_x = input_x + 8.0 + text_width + 2.0;
    painter.draw_line(
        Point::new(cursor_x, input_y + 6.0),
        Point::new(cursor_x, input_y + input_h - 6.0),
        TEXT_PRIMARY,
        1.0,
    );

    // Cancel button
    let cancel_x = dialog_x + dialog_w - 180.0;
    let cancel_y = dialog_y + dialog_h - 40.0;
    let cancel_rect = Rect::new(cancel_x, cancel_y, cancel_x + 70.0, cancel_y + 28.0);
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + 8.0,
        cancel_y + 18.0,
        TEXT_PRIMARY,
        12.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::RenameCancel);

    // Confirm button
    let confirm_x = dialog_x + dialog_w - 90.0;
    let confirm_rect = Rect::new(confirm_x, cancel_y, confirm_x + 70.0, cancel_y + 28.0);
    let is_valid = !state.rename_input_text.is_empty()
        && state.rename_input_text != state.rename_original_name;
    let btn_bg = if is_valid { ACCENT_VIEW } else { BTN_BG };
    let btn_tc = if is_valid {
        Color::rgb8(0x0F, 0x0F, 0x0F)
    } else {
        TEXT_MUTED
    };
    painter.fill_rounded_rect(confirm_rect, btn_bg, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Rename",
        confirm_x + 4.0,
        cancel_y + 18.0,
        btn_tc,
        12.0,
        FontWeight::BOLD,
        false,
        false,
    );
    if is_valid {
        state.register_click(confirm_rect, ClickAction::RenameConfirm);
    }

    painter.pop_clip();
}

/// Draws the hamburger menu overlay.
pub fn paint_hamburger_menu(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Backdrop
    let backdrop = Rect::from_origin_size((0.0, 0.0), size);
    painter.fill_rect(backdrop, Color::rgba8(0, 0, 0, 60));
    state.register_click(backdrop, ClickAction::DismissAll);

    let menu_w = 200.0;
    let menu_h = 320.0;
    let menu_x = 10.0;
    let menu_y = 50.0;
    let menu_rect = Rect::new(menu_x, menu_y, menu_x + menu_w, menu_y + menu_h);

    painter.fill_rounded_rect(menu_rect, PANEL_BG, 8.0);
    painter.stroke_rounded_rect(menu_rect, BORDER_COLOR, 1.0, 8.0);
    painter.push_clip(menu_rect);

    let items = [
        ("Open Image", ClickAction::OpenFileDialog),
        ("Open Folder", ClickAction::OpenFolderDialog),
        ("Open from URL", ClickAction::OpenFromUrl),
        (
            "Rename",
            ClickAction::ContextMenuAction("Rename".to_string()),
        ),
        ("Print", ClickAction::PrintImage),
        ("Share", ClickAction::ShareImage),
        ("Set as Wallpaper", ClickAction::SetWallpaperAction),
        ("Settings", ClickAction::ToggleSettings),
        ("Help", ClickAction::ShowHelp),
    ];

    let item_h = 32.0;
    let mut y = menu_y + 8.0;

    for (label, action) in &items {
        let item_rect = Rect::new(menu_x + 4.0, y, menu_x + menu_w - 4.0, y + item_h);
        painter.draw_text_cached(
            text_cache,
            label,
            menu_x + 18.0,
            y + item_h / 2.0 + 4.0,
            TEXT_PRIMARY,
            12.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        state.register_click(item_rect, action.clone());
        y += item_h;
    }

    painter.pop_clip();
}
