use super::*;

/// Draws the URL open dialog overlay.
pub fn paint_url_dialog(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    if !state.show_url_dialog {
        return;
    }

    let mut painter = Painter::new(scene);

    // Dimmed backdrop
    painter.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 160),
    );

    let dialog_w = 420.0_f64.min(size.width - 40.0);
    let dialog_h = 160.0;
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

    let pad = 20.0;
    let mut y = dialog_rect.y0 + pad;

    // Title
    let title = "Open from URL";
    painter.draw_text_cached(
        text_cache,
        title,
        dialog_rect.x0 + pad,
        y + 10.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 30.0;

    // URL input field (display current text)
    let input_rect = Rect::new(
        dialog_rect.x0 + pad,
        y,
        dialog_rect.x0 + dialog_w - pad - 80.0,
        y + 28.0,
    );
    painter.fill_rounded_rect(input_rect, Color::rgb8(0x0A, 0x0A, 0x0A), 4.0);
    painter.stroke_rounded_rect(input_rect, BORDER_COLOR, 1.0, 4.0);

    let display_text = if state.url_input_text.is_empty() {
        "https://example.com/image.png"
    } else {
        &state.url_input_text
    };
    let text_color = if state.url_input_text.is_empty() {
        TEXT_MUTED
    } else {
        TEXT_PRIMARY
    };
    painter.draw_text_cached(
        text_cache,
        display_text,
        input_rect.x0 + 8.0,
        input_rect.y0 + 18.0,
        text_color,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Load button
    let load_x = dialog_rect.x0 + dialog_w - pad - 70.0;
    let load_rect = Rect::new(load_x, y, load_x + 70.0, y + 28.0);
    painter.fill_rounded_rect(load_rect, ACCENT_VIEW, 4.0);
    let tw = text_cache.measure_text_width("Load", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Load",
        load_x + (70.0 - tw) / 2.0,
        y + 18.0,
        Color::rgb8(0x0F, 0x0F, 0x0F),
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(load_rect, ClickAction::LoadFromUrl);

    y += 40.0;

    // Cancel button
    let cancel_w = 60.0;
    let cancel_x = cx - cancel_w / 2.0;
    let cancel_rect = Rect::new(cancel_x, y, cancel_x + cancel_w, y + 24.0);
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Cancel", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + (cancel_w - tw) / 2.0,
        y + 16.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::UrlCancel);
}

/// Draws the print dialog overlay.
pub fn paint_print_dialog(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    if !state.show_print_dialog {
        return;
    }

    let mut painter = Painter::new(scene);

    // Dimmed backdrop
    painter.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 160),
    );

    let dialog_w = 380.0_f64.min(size.width - 40.0);
    let dialog_h = 320.0;
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

    let pad = 20.0;
    let mut y = dialog_rect.y0 + pad;

    // Title
    painter.draw_text_cached(
        text_cache,
        "Print",
        dialog_rect.x0 + pad,
        y + 10.0,
        TEXT_PRIMARY,
        16.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 36.0;

    // Paper size options
    painter.draw_text_cached(
        text_cache,
        "Paper Size",
        dialog_rect.x0 + pad,
        y + 8.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 22.0;

    let paper_sizes = ["A4", "A3", "Letter", "Legal", "4x6\"", "5x7\""];
    let mut px = dialog_rect.x0 + pad;
    for paper in &paper_sizes {
        let tw = text_cache.measure_text_width(paper, 10.0, FontWeight::MEDIUM);
        let btn_w = tw + 14.0;
        let btn_rect = Rect::new(px, y, px + btn_w, y + 22.0);
        painter.fill_rounded_rect(
            btn_rect,
            if state.print_paper_size == *paper {
                ACCENT_VIEW
            } else {
                BTN_BG
            },
            4.0,
        );
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);
        painter.draw_text_cached(
            text_cache,
            paper,
            px + 7.0,
            y + 14.0,
            TEXT_PRIMARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(
            btn_rect,
            ClickAction::PrintSelectPaper((*paper).to_string()),
        );
        px += btn_w + 6.0;
    }
    y += 34.0;

    // Orientation
    painter.draw_text_cached(
        text_cache,
        "Orientation",
        dialog_rect.x0 + pad,
        y + 8.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 22.0;

    let orientations = ["Portrait", "Landscape"];
    let mut ox = dialog_rect.x0 + pad;
    for orient in &orientations {
        let tw = text_cache.measure_text_width(orient, 10.0, FontWeight::MEDIUM);
        let btn_w = tw + 14.0;
        let btn_rect = Rect::new(ox, y, ox + btn_w, y + 22.0);
        painter.fill_rounded_rect(
            btn_rect,
            if state.print_orientation == *orient {
                ACCENT_VIEW
            } else {
                BTN_BG
            },
            4.0,
        );
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);
        painter.draw_text_cached(
            text_cache,
            orient,
            ox + 7.0,
            y + 14.0,
            TEXT_PRIMARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(
            btn_rect,
            ClickAction::PrintSelectOrientation((*orient).to_string()),
        );
        ox += btn_w + 6.0;
    }
    y += 34.0;

    // Fit options
    painter.draw_text_cached(
        text_cache,
        "Scaling",
        dialog_rect.x0 + pad,
        y + 8.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 22.0;

    let scales = ["Fit to page", "Fill page", "Actual size", "50%", "25%"];
    let mut sx = dialog_rect.x0 + pad;
    for scale in &scales {
        let tw = text_cache.measure_text_width(scale, 10.0, FontWeight::MEDIUM);
        let btn_w = tw + 14.0;
        if sx + btn_w > dialog_rect.x1 - pad {
            sx = dialog_rect.x0 + pad;
            y += 26.0;
        }
        let btn_rect = Rect::new(sx, y, sx + btn_w, y + 22.0);
        painter.fill_rounded_rect(
            btn_rect,
            if state.print_scaling == *scale {
                ACCENT_VIEW
            } else {
                BTN_BG
            },
            4.0,
        );
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);
        painter.draw_text_cached(
            text_cache,
            scale,
            sx + 7.0,
            y + 14.0,
            TEXT_PRIMARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(
            btn_rect,
            ClickAction::PrintSelectScaling((*scale).to_string()),
        );
        sx += btn_w + 6.0;
    }
    y += 40.0;

    // Print button
    let print_w = 80.0;
    let print_x = cx - print_w / 2.0 - 50.0;
    let print_rect = Rect::new(print_x, y, print_x + print_w, y + 28.0);
    painter.fill_rounded_rect(print_rect, ACCENT_VIEW, 4.0);
    let tw = text_cache.measure_text_width("Print", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Print",
        print_x + (print_w - tw) / 2.0,
        y + 18.0,
        Color::rgb8(0x0F, 0x0F, 0x0F),
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(print_rect, ClickAction::PrintImage);

    // Cancel button
    let cancel_w = 70.0;
    let cancel_x = cx + 20.0;
    let cancel_rect = Rect::new(cancel_x, y, cancel_x + cancel_w, y + 28.0);
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Cancel", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + (cancel_w - tw) / 2.0,
        y + 18.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::PrintCancel);
}
