use super::*;

// AI Panel

/// Draws the AI panel overlay.
/// Matches `.ai-panel` CSS: 300px wide, top-right, feature buttons.
pub fn paint_ai_panel(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let panel_w = 300.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;

    let features = AiFeature::all();
    let feature_h = 52.0;
    let panel_h = 50.0 + features.len() as f64 * (feature_h + 8.0) + 50.0;
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
        "AI Tools",
        panel_x + pad,
        y + 8.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );

    let close_x = panel_x + panel_w - pad - 40.0;
    let close_rect = Rect::new(close_x, y, close_x + 40.0, y + 28.0);
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(close_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        close_x + 2.0,
        y + 18.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ToggleAi);

    y += 30.0;

    // Hint
    painter.draw_text_cached(
        text_cache,
        "Select a feature to run on the current image",
        panel_x + pad,
        y + 4.0,
        TEXT_MUTED,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    y += 20.0;

    // Feature buttons
    for feature in features {
        let is_selected = state.ai_selected_feature == Some(*feature);
        let btn_rect = Rect::new(
            panel_x + pad,
            y,
            panel_x + panel_w - pad * 2.0,
            y + feature_h,
        );

        let bg = if is_selected {
            Color::rgba8(0x60, 0xA5, 0xFA, 38)
        } else {
            BTN_BG
        };
        painter.fill_rounded_rect(btn_rect, bg, 6.0);
        painter.stroke_rounded_rect(
            btn_rect,
            if is_selected {
                ACCENT_VIEW
            } else {
                BORDER_COLOR
            },
            1.0,
            6.0,
        );

        painter.draw_text_cached(
            text_cache,
            feature.label(),
            panel_x + pad + 10.0,
            y + 18.0,
            TEXT_PRIMARY,
            13.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        painter.draw_text_cached(
            text_cache,
            feature.description(),
            panel_x + pad + 10.0,
            y + 36.0,
            TEXT_MUTED,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        state.register_click(btn_rect, ClickAction::SelectAiFeature(*feature));

        y += feature_h + 8.0;
    }

    // Run button
    if state.ai_selected_feature.is_some() {
        let run_w = 80.0;
        let run_x = panel_x + panel_w - pad - run_w;
        let run_rect = Rect::new(run_x, y, run_x + run_w, y + 32.0);

        let run_label = if state.ai_running {
            "Running..."
        } else {
            "Run"
        };
        painter.fill_rounded_rect(run_rect, ACCENT_VIEW, 6.0);
        let tw = text_cache.measure_text_width(run_label, 12.0, FontWeight::BOLD);
        painter.draw_text_cached(
            text_cache,
            run_label,
            run_x + (run_w - tw) / 2.0,
            y + 20.0,
            Color::rgb8(0x0F, 0x0F, 0x0F),
            12.0,
            FontWeight::BOLD,
            false,
            false,
        );
        state.register_click(run_rect, ClickAction::RunAi);
    }

    // AI result text (for Tag/Describe)
    if let Some(ref result) = state.ai_result_text {
        y += 40.0;
        let result_rect = Rect::new(panel_x + pad, y, panel_x + panel_w - pad * 2.0, y + 48.0);
        painter.fill_rounded_rect(result_rect, Color::rgba8(0x1A, 0x1A, 0x1A, 60), 4.0);
        painter.stroke_rounded_rect(result_rect, BORDER_COLOR, 1.0, 4.0);
        // Truncate display to fit
        let display = if result.len() > 120 {
            format!("{}...", &result[..120])
        } else {
            result.clone()
        };
        painter.draw_text_cached(
            text_cache,
            &display,
            panel_x + pad + 8.0,
            y + 20.0,
            TEXT_PRIMARY,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    }

    painter.pop_clip();
}
