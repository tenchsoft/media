use super::*;

// File Info Overlay

/// Draws the file info overlay.
/// Matches `.file-info-overlay` CSS: centered, bottom area.
pub fn paint_file_info_overlay(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let Some(ref doc) = state.document else {
        return;
    };
    let mut painter = Painter::new(scene);

    // Extract document data upfront to avoid borrow conflicts with register_click
    let (file_name, file_path, file_size, dimensions, format) = {
        (
            doc.file_name.clone(),
            doc.path.clone(),
            doc.file_size,
            doc.dimensions,
            doc.format.clone(),
        )
    };

    let panel_w = 360.0_f64.min(size.width - 40.0);
    let panel_h = 220.0;
    let panel_x = (size.width - panel_w) / 2.0;
    let panel_y = size.height - 100.0 - panel_h;

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
        "File Info",
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
    state.register_click(close_rect, ClickAction::ToggleFileInfo);

    y += 28.0;

    // Grid fields
    let fields: Vec<(&str, String)> = vec![
        ("Name", file_name),
        ("Path", file_path),
        ("Size", bytes_label(file_size)),
        (
            "Dimensions",
            dimensions.map_or("N/A".to_string(), |d| {
                format!("{} x {} px", d.width, d.height)
            }),
        ),
        ("Format", format.to_uppercase()),
    ];

    for (label, value) in &fields {
        painter.draw_text_cached(
            text_cache,
            label,
            panel_x + pad,
            y + 8.0,
            TEXT_MUTED,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        painter.draw_text_cached(
            text_cache,
            value,
            panel_x + pad + 80.0,
            y + 8.0,
            TEXT_PRIMARY,
            12.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        y += 24.0;
    }

    painter.pop_clip();
}
