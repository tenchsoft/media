use super::*;

// Help Overlay

/// Draws the keyboard shortcuts help overlay.
/// Shows all available shortcuts in a centered modal dialog.
pub fn paint_help_overlay(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Dimmed backdrop
    painter.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 160),
    );

    let dialog_w = 420.0_f64.min(size.width - 40.0);
    let dialog_h = 480.0_f64.min(size.height - 60.0);
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let dialog_rect = Rect::new(
        cx - dialog_w / 2.0,
        cy - dialog_h / 2.0,
        cx + dialog_w / 2.0,
        cy + dialog_h / 2.0,
    );

    // Dialog background
    painter.fill_rounded_rect(dialog_rect, Color::rgb8(0x1E, 0x1E, 0x1E), 10.0);
    painter.stroke_rounded_rect(dialog_rect, BORDER_COLOR, 1.0, 10.0);

    painter.push_clip(dialog_rect);

    let pad = 20.0;
    let mut y = dialog_rect.y0 + pad;

    // Title
    painter.draw_text_cached(
        text_cache,
        "Keyboard Shortcuts",
        dialog_rect.x0 + pad,
        y + 10.0,
        TEXT_PRIMARY,
        16.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Close button
    let close_x = dialog_rect.x1 - pad - 40.0;
    let close_rect = Rect::new(close_x, y, close_x + 40.0, y + 28.0);
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(close_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Close",
        close_x + 2.0,
        y + 18.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ShowHelp);

    y += 40.0;

    // Shortcuts list
    let shortcuts: &[(&str, &str)] = &[
        ("\u{2190} / \u{2192}", "Navigate images"),
        ("+ / -", "Zoom in / out"),
        ("0", "Fit to window"),
        ("1", "Actual size (100%)"),
        ("R", "Rotate 90\u{00B0}"),
        ("B", "Cycle background color"),
        ("T", "Toggle filmstrip"),
        ("M / I", "Toggle metadata"),
        ("E", "Quick edit panel"),
        ("F", "Filter panel"),
        ("A", "AI tools panel"),
        ("Q", "File info overlay"),
        ("D", "Compare mode"),
        ("S", "Toggle slideshow"),
        ("P", "Toggle animated playback"),
        ("Ctrl+O", "Open file dialog"),
        ("Ctrl+S", "Save image"),
        ("Ctrl+Z", "Undo"),
        ("Ctrl+Y", "Redo"),
        ("Delete", "Delete image"),
        ("Escape", "Dismiss overlays"),
        ("?", "Show this help"),
    ];

    for (key, desc) in shortcuts {
        // Key column
        painter.draw_text_cached(
            text_cache,
            key,
            dialog_rect.x0 + pad,
            y + 10.0,
            ACCENT_VIEW,
            12.0,
            FontWeight::MEDIUM,
            false,
            false,
        );

        // Description column
        painter.draw_text_cached(
            text_cache,
            desc,
            dialog_rect.x0 + pad + 120.0,
            y + 10.0,
            TEXT_SECONDARY,
            12.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        y += 20.0;
    }

    painter.pop_clip();
}
