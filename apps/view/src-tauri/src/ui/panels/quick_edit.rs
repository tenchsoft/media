use super::*;

// Quick Edit Overlay

/// Draws the quick edit overlay.
/// Matches `.quick-edit-overlay` CSS: 260px wide, top-right.
pub fn paint_quick_edit_overlay(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let panel_w = 260.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;

    // Action buttons with click handlers
    let action_clicks: [(&str, &str, ClickAction); 6] = [
        (
            "\u{2B1C}",
            "Crop",
            ClickAction::ContextMenuAction("Crop".to_string()),
        ),
        (
            "\u{2194}",
            "Resize",
            ClickAction::ContextMenuAction("Resize".to_string()),
        ),
        ("\u{21BB}", "Rotate", ClickAction::Rotate),
        (
            "\u{27F3}",
            "Convert",
            ClickAction::ContextMenuAction("Convert".to_string()),
        ),
        ("\u{270F}", "Markup", ClickAction::QuickEditMarkup),
        ("\u{1F4CB}", "Copy", ClickAction::CopyImage),
    ];

    let action_h = 36.0;
    let annotation_section_h = if state.active_annotation_tool.is_some() {
        260.0
    } else {
        0.0
    };
    let panel_h =
        40.0 + action_clicks.len() as f64 * (action_h + 6.0) + 40.0 + 40.0 + annotation_section_h;
    let panel_rect = Rect::new(panel_x, panel_y, panel_x + panel_w, panel_y + panel_h);

    // Background
    painter.fill_rounded_rect(panel_rect, PANEL_BG, 8.0);
    painter.stroke_rounded_rect(panel_rect, BORDER_COLOR, 1.0, 8.0);

    // Clip to panel bounds
    painter.push_clip(panel_rect);

    let pad = 14.0;
    let mut y = panel_y + pad;

    // Header
    painter.draw_text_cached(
        text_cache,
        "Quick Edit",
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
        "Close",
        close_x + 2.0,
        y + 18.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ToggleQuickEdit);

    y += 40.0;

    for (icon, label, action) in &action_clicks {
        let btn_rect = Rect::new(
            panel_x + pad,
            y,
            panel_x + panel_w - pad * 2.0,
            y + action_h,
        );
        painter.fill_rounded_rect(btn_rect, BTN_BG, 4.0);
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);

        // Icon
        painter.draw_text_cached(
            text_cache,
            icon,
            panel_x + pad + 10.0,
            y + action_h / 2.0 + 4.0,
            TEXT_SECONDARY,
            14.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Label
        painter.draw_text_cached(
            text_cache,
            label,
            panel_x + pad + 36.0,
            y + action_h / 2.0 + 4.0,
            TEXT_PRIMARY,
            12.0,
            FontWeight::MEDIUM,
            false,
            false,
        );

        state.register_click(btn_rect, action.clone());
        y += action_h + 6.0;
    }

    // Divider
    y += 4.0;
    painter.draw_line(
        Point::new(panel_x + pad, y),
        Point::new(panel_x + panel_w - pad, y),
        BORDER_COLOR,
        1.0,
    );
    y += 10.0;

    // Delete button
    let delete_rect = Rect::new(
        panel_x + pad,
        y,
        panel_x + panel_w - pad * 2.0,
        y + action_h,
    );
    let error_color = ERROR_COLOR;
    painter.fill_rounded_rect(delete_rect, Color::rgba8(0xF3, 0x8B, 0xA8, 30), 4.0);
    painter.stroke_rounded_rect(delete_rect, Color::rgba8(0xF3, 0x8B, 0xA8, 100), 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Delete",
        panel_x + pad + 10.0,
        y + action_h / 2.0 + 4.0,
        error_color,
        12.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(
        delete_rect,
        ClickAction::ContextMenuAction("Delete".to_string()),
    );

    // Annotation tools section (shown when an annotation tool is active)
    if state.active_annotation_tool.is_some() {
        y += action_h + 10.0;
        painter.draw_line(
            Point::new(panel_x + pad, y),
            Point::new(panel_x + panel_w - pad, y),
            BORDER_COLOR,
            1.0,
        );
        y += 10.0;

        painter.draw_text_cached(
            text_cache,
            "Annotation Tools",
            panel_x + pad,
            y + 8.0,
            TEXT_SECONDARY,
            11.0,
            FontWeight::BOLD,
            false,
            false,
        );
        y += 22.0;

        let tools: [(&str, AnnotationTool); 7] = [
            ("Arrow", AnnotationTool::Arrow),
            ("Rect", AnnotationTool::Rectangle),
            ("Circle", AnnotationTool::Circle),
            ("Text", AnnotationTool::Text),
            ("Draw", AnnotationTool::Freeform),
            ("Blur", AnnotationTool::BlurMask),
            ("Erase", AnnotationTool::Eraser),
        ];
        let tool_btn_w = (panel_w - pad * 2.0 - 6.0 * 4.0) / 7.0;
        for (i, (label, tool)) in tools.iter().enumerate() {
            let tx = panel_x + pad + i as f64 * (tool_btn_w + 4.0);
            let tool_rect = Rect::new(tx, y, tx + tool_btn_w, y + 24.0);
            let is_active = state.active_annotation_tool == Some(*tool);
            let bg = if is_active { ACCENT_VIEW } else { BTN_BG };
            let text_c = if is_active {
                TEXT_PRIMARY
            } else {
                TEXT_SECONDARY
            };
            painter.fill_rounded_rect(tool_rect, bg, 3.0);
            painter.stroke_rounded_rect(tool_rect, BORDER_COLOR, 1.0, 3.0);
            let tw = text_cache.measure_text_width(label, 9.0, FontWeight::MEDIUM);
            painter.draw_text_cached(
                text_cache,
                label,
                tx + (tool_btn_w - tw) / 2.0,
                y + 16.0,
                text_c,
                9.0,
                FontWeight::MEDIUM,
                false,
                false,
            );
            state.register_click(tool_rect, ClickAction::SelectAnnotationTool(*tool));
        }

        y += 32.0;

        // Color swatch
        painter.draw_text_cached(
            text_cache,
            "Color:",
            panel_x + pad,
            y + 12.0,
            TEXT_SECONDARY,
            10.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        let swatch_rect = Rect::new(
            panel_x + pad + 42.0,
            y + 2.0,
            panel_x + pad + 58.0,
            y + 18.0,
        );
        painter.fill_rounded_rect(swatch_rect, state.annotation_color, 3.0);
        painter.stroke_rounded_rect(swatch_rect, BORDER_COLOR, 1.0, 3.0);
        state.register_click(swatch_rect, ClickAction::ToggleAnnotationColorPicker);

        y += 28.0;

        // Line width selector
        painter.draw_text_cached(
            text_cache,
            "Width:",
            panel_x + pad,
            y + 12.0,
            TEXT_SECONDARY,
            10.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        let line_widths: [(&str, f32); 3] = [("Thin", 1.0), ("Med", 2.0), ("Thick", 4.0)];
        let lw_btn_w = 36.0;
        let lw_start_x = panel_x + pad + 42.0;
        for (i, (lbl, w)) in line_widths.iter().enumerate() {
            let bx = lw_start_x + i as f64 * (lw_btn_w + 4.0);
            let bw_rect = Rect::new(bx, y + 2.0, bx + lw_btn_w, y + 18.0);
            let is_active = (state.annotation_line_width - *w).abs() < 0.1;
            let bg = if is_active { ACCENT_VIEW } else { BTN_BG };
            let tc = if is_active {
                TEXT_PRIMARY
            } else {
                TEXT_SECONDARY
            };
            painter.fill_rounded_rect(bw_rect, bg, 3.0);
            painter.stroke_rounded_rect(bw_rect, BORDER_COLOR, 1.0, 3.0);
            let tw = text_cache.measure_text_width(lbl, 9.0, FontWeight::MEDIUM);
            painter.draw_text_cached(
                text_cache,
                lbl,
                bx + (lw_btn_w - tw) / 2.0,
                y + 13.0,
                tc,
                9.0,
                FontWeight::MEDIUM,
                false,
                false,
            );
            state.register_click(bw_rect, ClickAction::AnnotationSetLineWidth(*w));
        }

        y += 28.0;

        // Undo / Redo buttons
        let undo_redo_w = (panel_w - pad * 2.0 - 4.0) / 2.0;
        let undo_rect = Rect::new(panel_x + pad, y, panel_x + pad + undo_redo_w, y + 24.0);
        painter.fill_rounded_rect(undo_rect, BTN_BG, 3.0);
        painter.stroke_rounded_rect(undo_rect, BORDER_COLOR, 1.0, 3.0);
        let tw = text_cache.measure_text_width("Undo", 10.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            "Undo",
            panel_x + pad + (undo_redo_w - tw) / 2.0,
            y + 16.0,
            TEXT_SECONDARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(undo_rect, ClickAction::AnnotationUndo);

        let redo_x = panel_x + pad + undo_redo_w + 4.0;
        let redo_rect = Rect::new(redo_x, y, redo_x + undo_redo_w, y + 24.0);
        painter.fill_rounded_rect(redo_rect, BTN_BG, 3.0);
        painter.stroke_rounded_rect(redo_rect, BORDER_COLOR, 1.0, 3.0);
        let tw = text_cache.measure_text_width("Redo", 10.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            "Redo",
            redo_x + (undo_redo_w - tw) / 2.0,
            y + 16.0,
            TEXT_SECONDARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(redo_rect, ClickAction::AnnotationRedo);

        y += 32.0;

        // Save / Exit buttons
        let save_exit_w = (panel_w - pad * 2.0 - 4.0) / 2.0;
        let save_rect = Rect::new(panel_x + pad, y, panel_x + pad + save_exit_w, y + 24.0);
        painter.fill_rounded_rect(save_rect, ACCENT_VIEW, 3.0);
        painter.stroke_rounded_rect(save_rect, BORDER_COLOR, 1.0, 3.0);
        let tw = text_cache.measure_text_width("Save", 10.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            "Save",
            panel_x + pad + (save_exit_w - tw) / 2.0,
            y + 16.0,
            TEXT_PRIMARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(save_rect, ClickAction::AnnotationSave);

        let exit_x = panel_x + pad + save_exit_w + 4.0;
        let exit_rect = Rect::new(exit_x, y, exit_x + save_exit_w, y + 24.0);
        painter.fill_rounded_rect(exit_rect, BTN_BG, 3.0);
        painter.stroke_rounded_rect(exit_rect, BORDER_COLOR, 1.0, 3.0);
        let tw = text_cache.measure_text_width("Exit", 10.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            "Exit",
            exit_x + (save_exit_w - tw) / 2.0,
            y + 16.0,
            TEXT_SECONDARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(exit_rect, ClickAction::AnnotationExit);

        y += 32.0;

        // Clear annotations button
        let clear_rect = Rect::new(panel_x + pad, y, panel_x + panel_w - pad, y + 24.0);
        painter.fill_rounded_rect(clear_rect, BTN_BG, 3.0);
        painter.stroke_rounded_rect(clear_rect, BORDER_COLOR, 1.0, 3.0);
        let tw = text_cache.measure_text_width("Clear All", 10.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            "Clear All",
            panel_x + pad + (panel_w - pad * 2.0 - tw) / 2.0,
            y + 16.0,
            TEXT_SECONDARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(clear_rect, ClickAction::ClearAnnotations);
    }

    painter.pop_clip();
}
