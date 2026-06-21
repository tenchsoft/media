use super::*;

/// Renders annotations on top of the image.
pub fn paint_annotations_overlay(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    use super::state::AnnotationTool;

    if state.annotations.is_empty() && !state.show_annotation_exit_confirm {
        return;
    }

    let mut painter = Painter::new(scene);

    // Get the image rect to map annotation coordinates
    let doc = match state.document.as_ref() {
        Some(d) => d,
        None => return,
    };
    let dims = doc.dimensions;

    let img_rect = super::image_stage::compute_image_rect(state, size, dims);
    let d = match dims {
        Some(d) => d,
        None => return,
    };
    let scale_x = img_rect.width() / d.width as f64;
    let scale_y = img_rect.height() / d.height as f64;

    for ann in &state.annotations {
        let ax = img_rect.x0 + ann.x * scale_x;
        let ay = img_rect.y0 + ann.y * scale_y;
        let aw = ann.w * scale_x;
        let ah = ann.h * scale_y;
        let lw = ann.line_width as f64;

        match ann.tool {
            AnnotationTool::Rectangle => {
                let rect = Rect::new(ax, ay, ax + aw, ay + ah);
                painter.stroke_rounded_rect(rect, ann.color, lw, 2.0);
            }
            AnnotationTool::Circle => {
                // Approximate ellipse with a rounded rect
                let cx = ax + aw / 2.0;
                let cy = ay + ah / 2.0;
                let rect = Rect::new(cx - aw / 2.0, cy - ah / 2.0, cx + aw / 2.0, cy + ah / 2.0);
                painter.stroke_rounded_rect(rect, ann.color, lw, aw.min(ah) / 2.0);
            }
            AnnotationTool::Arrow => {
                painter.draw_line(
                    Point::new(ax, ay),
                    Point::new(ax + aw, ay + ah),
                    ann.color,
                    lw,
                );
                let angle = (ah).atan2(aw);
                let head_len = 12.0;
                let end_x = ax + aw;
                let end_y = ay + ah;
                let a1 = angle + std::f64::consts::PI * 0.85;
                let a2 = angle - std::f64::consts::PI * 0.85;
                painter.draw_line(
                    Point::new(end_x, end_y),
                    Point::new(end_x + head_len * a1.cos(), end_y + head_len * a1.sin()),
                    ann.color,
                    lw,
                );
                painter.draw_line(
                    Point::new(end_x, end_y),
                    Point::new(end_x + head_len * a2.cos(), end_y + head_len * a2.sin()),
                    ann.color,
                    lw,
                );
            }
            AnnotationTool::Text => {
                if !ann.text.is_empty() {
                    let tw = text_cache.measure_text_width(&ann.text, 13.0, FontWeight::MEDIUM);
                    let text_bg = Rect::new(ax - 2.0, ay - 14.0, ax + tw + 4.0, ay + 4.0);
                    painter.fill_rounded_rect(text_bg, Color::rgba8(0, 0, 0, 180), 3.0);
                    painter.draw_text_cached(
                        text_cache,
                        &ann.text,
                        ax,
                        ay,
                        ann.color,
                        13.0,
                        FontWeight::MEDIUM,
                        false,
                        false,
                    );
                }
            }
            AnnotationTool::Freeform => {
                painter.draw_line(
                    Point::new(ax, ay),
                    Point::new(ax + aw, ay + ah),
                    ann.color,
                    lw.max(3.0),
                );
            }
            AnnotationTool::BlurMask => {
                let rect = Rect::new(ax, ay, ax + aw, ay + ah);
                painter.fill_rounded_rect(rect, Color::rgba8(0x80, 0x80, 0x80, 100), 0.0);
                painter.stroke_rounded_rect(rect, ann.color, lw, 0.0);
            }
            AnnotationTool::Eraser => {
                // Eraser annotations are not rendered (they represent deletions)
            }
        }
    }

    // Annotation exit confirmation dialog
    if state.show_annotation_exit_confirm {
        paint_annotation_exit_confirm(state, text_cache, size, scene);
    }
}

/// Draws a basic color picker for annotation tools.
///
/// Shows a grid of preset colors and a current-color indicator.
pub fn paint_annotation_color_picker(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    if !state.show_annotation_color_picker {
        return;
    }

    let mut painter = Painter::new(scene);

    let picker_w = 240.0;
    let picker_h = 180.0;
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let picker_rect = Rect::new(
        cx - picker_w / 2.0,
        cy - picker_h / 2.0,
        cx + picker_w / 2.0,
        cy + picker_h / 2.0,
    );

    // Background
    painter.fill_rounded_rect(picker_rect, Color::rgb8(0x1E, 0x1E, 0x1E), 10.0);
    painter.stroke_rounded_rect(picker_rect, BORDER_COLOR, 1.0, 10.0);

    let pad = 16.0;
    let mut y = picker_rect.y0 + pad;

    // Title
    painter.draw_text_cached(
        text_cache,
        "Annotation Color",
        picker_rect.x0 + pad,
        y + 8.0,
        TEXT_PRIMARY,
        13.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 28.0;

    // Current color preview
    let preview_rect = Rect::new(
        picker_rect.x0 + pad,
        y,
        picker_rect.x0 + pad + 28.0,
        y + 28.0,
    );
    painter.fill_rounded_rect(preview_rect, state.annotation_color, 4.0);
    painter.stroke_rounded_rect(preview_rect, BORDER_COLOR, 1.0, 4.0);

    // Current color label
    let color_label = format!(
        "R:{} G:{} B:{}",
        state.annotation_color.r(),
        state.annotation_color.g(),
        state.annotation_color.b()
    );
    painter.draw_text_cached(
        text_cache,
        &color_label,
        picker_rect.x0 + pad + 36.0,
        y + 18.0,
        TEXT_SECONDARY,
        10.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    y += 40.0;

    // Color grid - preset colors
    let preset_colors: [(u8, u8, u8); 16] = [
        (0xFF, 0x00, 0x00), // Red
        (0xFF, 0x80, 0x00), // Orange
        (0xFF, 0xFF, 0x00), // Yellow
        (0x00, 0xFF, 0x00), // Green
        (0x00, 0xFF, 0xFF), // Cyan
        (0x00, 0x80, 0xFF), // Light Blue
        (0x00, 0x00, 0xFF), // Blue
        (0x80, 0x00, 0xFF), // Purple
        (0xFF, 0x00, 0xFF), // Magenta
        (0xFF, 0x80, 0x80), // Light Red
        (0xFF, 0xFF, 0xFF), // White
        (0xC0, 0xC0, 0xC0), // Light Gray
        (0x80, 0x80, 0x80), // Gray
        (0x40, 0x40, 0x40), // Dark Gray
        (0x00, 0x00, 0x00), // Black
        (0x80, 0x40, 0x00), // Brown
    ];

    let swatch_size = 24.0;
    let gap = 6.0;
    let cols = 8;
    for (i, &(r, g, b)) in preset_colors.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let sx = picker_rect.x0 + pad + col as f64 * (swatch_size + gap);
        let sy = y + row as f64 * (swatch_size + gap);
        let swatch_rect = Rect::new(sx, sy, sx + swatch_size, sy + swatch_size);

        let color = Color::rgb8(r, g, b);
        painter.fill_rounded_rect(swatch_rect, color, 3.0);
        painter.stroke_rounded_rect(swatch_rect, BORDER_COLOR, 1.0, 3.0);

        state.register_click(swatch_rect, ClickAction::SetAnnotationColor(color));
    }

    // Close button
    let close_w = 60.0;
    let close_x = cx - close_w / 2.0;
    let close_y = picker_rect.y1 - 36.0;
    let close_rect = Rect::new(close_x, close_y, close_x + close_w, close_y + 24.0);
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(close_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Close", 10.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Close",
        close_x + (close_w - tw) / 2.0,
        close_y + 16.0,
        TEXT_PRIMARY,
        10.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ToggleAnnotationColorPicker);
}

/// Draws the annotation exit confirmation dialog.
fn paint_annotation_exit_confirm(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Dim overlay
    painter.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );

    let dlg_w = 280.0;
    let dlg_h = 140.0;
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let dlg_rect = Rect::new(
        cx - dlg_w / 2.0,
        cy - dlg_h / 2.0,
        cx + dlg_w / 2.0,
        cy + dlg_h / 2.0,
    );

    painter.fill_rounded_rect(dlg_rect, Color::rgb8(0x1E, 0x1E, 0x1E), 10.0);
    painter.stroke_rounded_rect(dlg_rect, BORDER_COLOR, 1.0, 10.0);

    let pad = 16.0;

    // Title
    painter.draw_text_cached(
        text_cache,
        "Discard Annotations?",
        dlg_rect.x0 + pad,
        dlg_rect.y0 + 28.0,
        TEXT_PRIMARY,
        13.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Message
    painter.draw_text_cached(
        text_cache,
        "Unsaved annotations will be lost.",
        dlg_rect.x0 + pad,
        dlg_rect.y0 + 52.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Buttons
    let btn_w = 90.0;
    let btn_h = 28.0;
    let btn_y = dlg_rect.y1 - pad - btn_h;
    let gap = 12.0;
    let total_w = btn_w * 2.0 + gap;
    let start_x = cx - total_w / 2.0;

    // Discard button
    let discard_rect = Rect::new(start_x, btn_y, start_x + btn_w, btn_y + btn_h);
    painter.fill_rounded_rect(discard_rect, Color::rgba8(0xF3, 0x8B, 0xA8, 40), 4.0);
    painter.stroke_rounded_rect(discard_rect, Color::rgba8(0xF3, 0x8B, 0xA8, 100), 1.0, 4.0);
    let tw = text_cache.measure_text_width("Discard", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Discard",
        start_x + (btn_w - tw) / 2.0,
        btn_y + 18.0,
        Color::rgb8(0xEF, 0x44, 0x44),
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(discard_rect, ClickAction::AnnotationExitConfirm);

    // Cancel button
    let cancel_x = start_x + btn_w + gap;
    let cancel_rect = Rect::new(cancel_x, btn_y, cancel_x + btn_w, btn_y + btn_h);
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Cancel", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + (btn_w - tw) / 2.0,
        btn_y + 18.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::AnnotationExitCancel);
}
