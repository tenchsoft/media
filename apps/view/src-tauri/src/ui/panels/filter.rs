use super::*;

// Filter Panel

/// Draws the filter panel overlay.
/// Matches `.filter-panel` CSS: 280px wide, top-right, with sliders.
pub fn paint_filter_panel(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let panel_w = 280.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;

    let sliders = [
        ("Brightness", state.filter_brightness, 0.0, 200.0, "%"),
        ("Contrast", state.filter_contrast, 0.0, 200.0, "%"),
        ("Saturation", state.filter_saturation, 0.0, 200.0, "%"),
        ("Blur", state.filter_blur, 0.0, 20.0, "px"),
        (
            "Hue Rotate",
            state.filter_hue_rotate,
            0.0,
            360.0,
            "\u{00B0}",
        ),
    ];

    let panel_h = 50.0 + sliders.len() as f64 * 40.0 + 50.0;
    let panel_rect = Rect::new(panel_x, panel_y, panel_x + panel_w, panel_y + panel_h);

    // Background
    painter.fill_rounded_rect(panel_rect, PANEL_BG, 8.0);
    painter.stroke_rounded_rect(panel_rect, BORDER_COLOR, 1.0, 8.0);

    // Clip to panel bounds
    painter.push_clip(panel_rect);

    let pad = 16.0;
    let mut y = panel_y + pad;

    // Header
    painter.draw_text_cached(
        text_cache,
        "Filters",
        panel_x + pad,
        y + 8.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Close button
    let close_x = panel_x + panel_w - pad - 24.0;
    let close_rect = Rect::new(close_x, y, close_x + 24.0, y + 24.0);
    painter.draw_text_cached(
        text_cache,
        "x",
        close_x,
        y + 8.0,
        TEXT_SECONDARY,
        14.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ToggleFilter);

    y += 30.0;

    // Sliders
    for (label, value, min, max, unit) in &sliders {
        painter.draw_text_cached(
            text_cache,
            label,
            panel_x + pad,
            y + 8.0,
            TEXT_SECONDARY,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Slider track
        let track_x = panel_x + pad + 80.0;
        let track_w = panel_w - pad * 2.0 - 128.0;
        let track_y = y + 4.0;
        let track_rect = Rect::new(track_x, track_y, track_x + track_w, track_y + 6.0);
        painter.fill_rounded_rect(track_rect, Color::rgb8(0x3A, 0x3A, 0x3A), 3.0);

        // Active track
        let norm = (*value - min) / (max - min);
        let active_w = track_w * norm;
        if active_w > 0.0 {
            let active_rect = Rect::new(track_x, track_y, track_x + active_w, track_y + 6.0);
            painter.fill_rounded_rect(active_rect, ACCENT_VIEW, 3.0);
        }

        // Thumb
        let thumb_x = track_x + active_w;
        let thumb_rect = Rect::new(thumb_x - 5.0, track_y - 2.0, thumb_x + 5.0, track_y + 8.0);
        painter.fill_rounded_rect(thumb_rect, ACCENT_VIEW, 5.0);

        // Value label
        let val_label = if *unit == "%" {
            format!("{:.0}%", value)
        } else {
            format!("{:.0}{}", value, unit)
        };
        painter.draw_text_cached(
            text_cache,
            &val_label,
            panel_x + panel_w - pad - 40.0,
            y + 8.0,
            TEXT_MUTED,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        y += 40.0;
    }

    // Action buttons
    let reset_x = panel_x + panel_w - pad - 140.0;
    let apply_x = panel_x + panel_w - pad - 60.0;
    let btn_y = y + 4.0;
    let btn_h = 28.0;

    // Reset button
    let reset_rect = Rect::new(reset_x, btn_y, reset_x + 60.0, btn_y + btn_h);
    painter.fill_rounded_rect(reset_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(reset_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Reset", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Reset",
        reset_x + (60.0 - tw) / 2.0,
        btn_y + 18.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(reset_rect, ClickAction::FilterReset);

    // Apply button
    let apply_rect = Rect::new(apply_x, btn_y, apply_x + 60.0, btn_y + btn_h);
    painter.fill_rounded_rect(apply_rect, ACCENT_VIEW, 4.0);
    let tw = text_cache.measure_text_width("Apply", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Apply",
        apply_x + (60.0 - tw) / 2.0,
        btn_y + 18.0,
        Color::rgb8(0x0F, 0x0F, 0x0F),
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(apply_rect, ClickAction::FilterApply);

    painter.pop_clip();
}
