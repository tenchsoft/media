use super::state::{PixelDesignState, Tool};
use super::theme;
use tench_ui::parley::FontWeight;
use tench_ui::peniko::{ImageAlphaType, ImageData, ImageFormat};
use tench_ui::prelude::*;

pub fn canvas_document_rect(state: &PixelDesignState, viewport: Rect) -> Rect {
    let zoom = state.zoom as f64 / 100.0;
    let mut doc_w = state.document.width as f64 * zoom;
    let mut doc_h = state.document.height as f64 * zoom;
    let max_w = (viewport.width() - 56.0).max(120.0);
    let max_h = (viewport.height() - 64.0).max(120.0);
    let fit = (max_w / doc_w).min(max_h / doc_h).min(1.0);
    doc_w *= fit;
    doc_h *= fit;
    let x0 = viewport.x0 + (viewport.width() - doc_w) * 0.5 + state.viewport_offset_x;
    let y0 = viewport.y0 + (viewport.height() - doc_h) * 0.5 + state.viewport_offset_y;
    Rect::new(x0, y0, x0 + doc_w, y0 + doc_h)
}

pub fn paint_canvas_viewport(
    state: &PixelDesignState,
    p: &mut Painter<'_>,
    theme_ui: &Theme,
    viewport: Rect,
    status_rect: Rect,
) {
    p.fill_rect(viewport, theme::CANVAS_BG);
    draw_checkerboard(p, viewport, 24.0);

    let doc = canvas_document_rect(state, viewport);
    p.stroke_rounded_rect(doc, theme_ui.disabled, 1.0, 2.0);

    // Render the composited document image
    if let Some(composited) = &state.composited_image {
        let img_data = ImageData {
            width: composited.width,
            height: composited.height,
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::AlphaPremultiplied,
            data: composited.data.clone().into(),
        };
        p.draw_image(&img_data, doc);
    } else {
        // No composited image yet — draw checkerboard inside doc area
        draw_checkerboard(p, doc, 12.0);
    }

    // Selection overlay
    if let Some(selection) = state.normalized_selection() {
        let x0 = doc.x0 + selection.start.x / state.document.width as f64 * doc.width();
        let y0 = doc.y0 + selection.start.y / state.document.height as f64 * doc.height();
        let x1 = doc.x0 + selection.end.x / state.document.width as f64 * doc.width();
        let y1 = doc.y0 + selection.end.y / state.document.height as f64 * doc.height();
        let rect = Rect::new(x0, y0, x1, y1);
        if state.active_tool == Tool::Crop || state.active_tool == Tool::Select {
            // Dim outside selection
            p.fill_rect(
                Rect::new(doc.x0, doc.y0, doc.x1, y0),
                Color::rgba8(0, 0, 0, 130),
            );
            p.fill_rect(
                Rect::new(doc.x0, y1, doc.x1, doc.y1),
                Color::rgba8(0, 0, 0, 130),
            );
            p.fill_rect(Rect::new(doc.x0, y0, x0, y1), Color::rgba8(0, 0, 0, 130));
            p.fill_rect(Rect::new(x1, y0, doc.x1, y1), Color::rgba8(0, 0, 0, 130));
        }
        p.stroke_rounded_rect(rect, theme::SELECTION_COLOR, 1.5, 0.0);
        // Marching ants cross-lines
        p.draw_line(
            Point::new(rect.x0, rect.y0),
            Point::new(rect.x1, rect.y1),
            Color::rgba8(255, 255, 255, 80),
            0.7,
        );
        p.draw_line(
            Point::new(rect.x1, rect.y0),
            Point::new(rect.x0, rect.y1),
            Color::rgba8(255, 255, 255, 80),
            0.7,
        );
    }

    // Text input overlay
    if state.show_text_input {
        let x = doc.x0 + state.text_pos.x / state.document.width as f64 * doc.width();
        let y = doc.y0 + state.text_pos.y / state.document.height as f64 * doc.height();
        let overlay = Rect::new(x, y - 20.0, x + 190.0, y + 14.0);
        p.fill_rounded_rect(overlay, theme_ui.surface, 4.0);
        p.stroke_rounded_rect(overlay, theme_ui.primary, 1.0, 4.0);
        let text = if state.text_input.is_empty() {
            "Type text..."
        } else {
            &state.text_input
        };
        p.draw_text(
            text,
            x + 8.0,
            y + 2.0,
            state.fg_color,
            theme_ui.font_size,
            FontWeight::NORMAL,
            false,
        );
    }

    // Rulers (Phase 7)
    if state.show_rulers {
        paint_rulers(p, theme_ui, viewport, doc);
    }

    // Grid overlay (Phase 7)
    if state.show_grid {
        paint_grid(
            p,
            viewport,
            doc,
            state.document.width,
            state.document.height,
        );
    }

    paint_canvas_status(state, p, theme_ui, status_rect);
}

fn draw_checkerboard(p: &mut Painter<'_>, rect: Rect, cell: f64) {
    let rows = (rect.height() / cell).ceil() as usize;
    let cols = (rect.width() / cell).ceil() as usize;
    for row in 0..rows {
        for col in 0..cols {
            let x = rect.x0 + col as f64 * cell;
            let y = rect.y0 + row as f64 * cell;
            let color = if (row + col) % 2 == 0 {
                theme::CHECKER_LIGHT
            } else {
                theme::CHECKER_DARK
            };
            p.fill_rect(
                Rect::new(x, y, (x + cell).min(rect.x1), (y + cell).min(rect.y1)),
                color,
            );
        }
    }
}

fn paint_rulers(p: &mut Painter<'_>, theme_ui: &Theme, viewport: Rect, doc: Rect) {
    let ruler_w = 20.0;
    // Horizontal ruler background
    p.fill_rect(
        Rect::new(viewport.x0, viewport.y0, viewport.x1, viewport.y0 + ruler_w),
        theme_ui.surface,
    );
    // Vertical ruler background
    p.fill_rect(
        Rect::new(viewport.x0, viewport.y0, viewport.x0 + ruler_w, viewport.y1),
        theme_ui.surface,
    );

    let zoom = doc.width() / state_document_width(doc, viewport);
    let step = if zoom > 2.0 {
        10.0
    } else if zoom > 0.5 {
        50.0
    } else {
        100.0
    };

    // Horizontal ruler ticks
    let mut x = doc.x0;
    let mut px = 0u32;
    while x < doc.x1 && x < viewport.x1 {
        if x >= viewport.x0 + ruler_w {
            p.draw_line(
                Point::new(x, viewport.y0),
                Point::new(x, viewport.y0 + 8.0),
                theme_ui.disabled,
                1.0,
            );
            if px.is_multiple_of(step as u32 * 5) {
                p.draw_text(
                    &px.to_string(),
                    x + 2.0,
                    viewport.y0 + 16.0,
                    theme_ui.disabled,
                    8.0,
                    FontWeight::NORMAL,
                    false,
                );
            }
        }
        x += step * zoom;
        px += step as u32;
    }
}

fn state_document_width(doc: Rect, viewport: Rect) -> f64 {
    (doc.width() / viewport.width() * 800.0).max(1.0)
}

fn paint_grid(p: &mut Painter<'_>, viewport: Rect, doc: Rect, doc_w: u32, _doc_h: u32) {
    let zoom = doc.width() / doc_w as f64;
    let grid_step = if zoom > 4.0 {
        10.0
    } else if zoom > 1.0 {
        50.0
    } else {
        100.0
    };
    let grid_color = theme::GRID_COLOR;

    let mut x = doc.x0;
    while x < doc.x1 {
        if x >= viewport.x0 && x <= viewport.x1 {
            p.draw_line(
                Point::new(x, doc.y0.max(viewport.y0)),
                Point::new(x, doc.y1.min(viewport.y1)),
                grid_color,
                0.5,
            );
        }
        x += grid_step * zoom;
    }
    let mut y = doc.y0;
    while y < doc.y1 {
        if y >= viewport.y0 && y <= viewport.y1 {
            p.draw_line(
                Point::new(doc.x0.max(viewport.x0), y),
                Point::new(doc.x1.min(viewport.x1), y),
                grid_color,
                0.5,
            );
        }
        y += grid_step * zoom;
    }
}

fn paint_canvas_status(
    state: &PixelDesignState,
    p: &mut Painter<'_>,
    theme_ui: &Theme,
    status_rect: Rect,
) {
    p.fill_rect(status_rect, theme_ui.surface);
    p.draw_line(
        Point::new(status_rect.x0, status_rect.y0),
        Point::new(status_rect.x1, status_rect.y0),
        theme::BORDER,
        1.0,
    );

    let y = status_rect.y0 + 21.0;
    p.draw_text(
        &state.status_msg,
        status_rect.x0 + 14.0,
        y,
        theme_ui.on_surface,
        theme_ui.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    p.draw_text(
        &format!("{} x {}", state.document.width, state.document.height),
        status_rect.x0 + 220.0,
        y,
        theme_ui.secondary,
        theme_ui.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    p.draw_text(
        &format!(
            "x: {}  y: {}",
            state.mouse_pos.x.round(),
            state.mouse_pos.y.round()
        ),
        status_rect.x0 + 348.0,
        y,
        theme_ui.secondary,
        theme_ui.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    let zoom_x = status_rect.x1 - 142.0;
    p.fill_rounded_rect(
        Rect::new(
            zoom_x,
            status_rect.y0 + 5.0,
            zoom_x + 28.0,
            status_rect.y1 - 5.0,
        ),
        theme::BG_BUTTON,
        4.0,
    );
    p.draw_text(
        "-",
        zoom_x + 11.0,
        y,
        theme_ui.on_surface,
        theme_ui.font_size,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &format!("{}%", state.zoom),
        zoom_x + 44.0,
        y,
        theme_ui.on_surface,
        theme_ui.font_size_small,
        FontWeight::MEDIUM,
        false,
    );
    p.fill_rounded_rect(
        Rect::new(
            zoom_x + 100.0,
            status_rect.y0 + 5.0,
            zoom_x + 128.0,
            status_rect.y1 - 5.0,
        ),
        theme::BG_BUTTON,
        4.0,
    );
    p.draw_text(
        "+",
        zoom_x + 109.0,
        y,
        theme_ui.on_surface,
        theme_ui.font_size,
        FontWeight::BOLD,
        false,
    );
}
