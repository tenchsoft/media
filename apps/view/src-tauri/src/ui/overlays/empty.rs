use super::*;

/// Draws the empty state view with a drop zone and recent files.
/// Computes overlay empty-state button rects without rendering.
/// Returns a list of (ClickAction, Rect) for Open File, Open Folder,
/// and recent file slots.
pub fn overlay_empty_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    let mut rects = Vec::new();

    if state.document.is_some() {
        return rects;
    }

    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let dz_h = 160.0;

    let btn_w = 80.0;
    let btn_h = 28.0;
    let btn_gap = 12.0;
    let total_btn_w = btn_w * 2.0 + btn_gap;
    let btn_y = cy - dz_h / 2.0 + 70.0;

    // Open File button
    let file_btn_x = cx - total_btn_w / 2.0;
    rects.push((
        ClickAction::OpenFileDialog,
        Rect::new(file_btn_x, btn_y, file_btn_x + btn_w, btn_y + btn_h),
    ));

    // Open Folder button
    let folder_btn_x = file_btn_x + btn_w + btn_gap;
    rects.push((
        ClickAction::OpenFolderDialog,
        Rect::new(folder_btn_x, btn_y, folder_btn_x + btn_w, btn_y + btn_h),
    ));

    // Recent file slots
    let max_recent = 5.min(state.recent_files.len());
    let recent_y = cy + dz_h / 2.0 - 20.0;
    let recent_start_y = recent_y + 20.0;
    let item_h = 24.0;

    for i in 0..max_recent {
        let ry = recent_start_y + i as f64 * item_h;
        rects.push((
            ClickAction::OpenRecentFromEmpty(i),
            Rect::new(cx - 140.0, ry, cx + 140.0, ry + item_h),
        ));
    }

    rects
}

pub fn paint_empty_state(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    if state.document.is_some() {
        return;
    }

    let mut painter = Painter::new(scene);
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;

    // Drop zone icon (dashed border rectangle)
    let dz_w = 300.0_f64.min(size.width - 60.0);
    let dz_h = 160.0;
    let dz_rect = Rect::new(
        cx - dz_w / 2.0,
        cy - dz_h / 2.0 - 40.0,
        cx + dz_w / 2.0,
        cy + dz_h / 2.0 - 40.0,
    );

    // Dashed border approximation using multiple small lines
    let dash_len = 8.0;
    let gap_len = 4.0;
    let border_color = Color::rgba8(0x60, 0x60, 0x60, 180);

    // Top edge
    let mut x = dz_rect.x0;
    while x < dz_rect.x1 {
        let end = (x + dash_len).min(dz_rect.x1);
        painter.draw_line(
            Point::new(x, dz_rect.y0),
            Point::new(end, dz_rect.y0),
            border_color,
            2.0,
        );
        x = end + gap_len;
    }
    // Bottom edge
    let mut x = dz_rect.x0;
    while x < dz_rect.x1 {
        let end = (x + dash_len).min(dz_rect.x1);
        painter.draw_line(
            Point::new(x, dz_rect.y1),
            Point::new(end, dz_rect.y1),
            border_color,
            2.0,
        );
        x = end + gap_len;
    }
    // Left edge
    let mut y = dz_rect.y0;
    while y < dz_rect.y1 {
        let end = (y + dash_len).min(dz_rect.y1);
        painter.draw_line(
            Point::new(dz_rect.x0, y),
            Point::new(dz_rect.x0, end),
            border_color,
            2.0,
        );
        y = end + gap_len;
    }
    // Right edge
    let mut y = dz_rect.y0;
    while y < dz_rect.y1 {
        let end = (y + dash_len).min(dz_rect.y1);
        painter.draw_line(
            Point::new(dz_rect.x1, y),
            Point::new(dz_rect.x1, end),
            border_color,
            2.0,
        );
        y = end + gap_len;
    }

    // Center icon (image icon using text)
    painter.draw_text_cached(
        text_cache,
        "\u{1F5BC}", // camera/image emoji as fallback
        cx - 12.0,
        cy - dz_h / 2.0 - 10.0,
        TEXT_MUTED,
        28.0,
        FontWeight::NORMAL,
        true,
        false,
    );

    // "Drop an image here" text
    painter.draw_text_cached(
        text_cache,
        "Drop an image here or press Ctrl+O",
        cx,
        cy - dz_h / 2.0 + 30.0,
        TEXT_SECONDARY,
        14.0,
        FontWeight::NORMAL,
        true,
        false,
    );

    // Subtitle
    painter.draw_text_cached(
        text_cache,
        "Ctrl+Shift+O to open a folder",
        cx,
        cy - dz_h / 2.0 + 52.0,
        TEXT_MUTED,
        11.0,
        FontWeight::NORMAL,
        true,
        false,
    );

    // Open buttons
    let btn_w = 80.0;
    let btn_h = 28.0;
    let btn_gap = 12.0;
    let total_btn_w = btn_w * 2.0 + btn_gap;
    let btn_y = cy - dz_h / 2.0 + 70.0;

    // Open File button
    let file_btn_x = cx - total_btn_w / 2.0;
    let file_btn_rect = Rect::new(file_btn_x, btn_y, file_btn_x + btn_w, btn_y + btn_h);
    painter.fill_rounded_rect(file_btn_rect, ACCENT_VIEW, 4.0);
    let tw = text_cache.measure_text_width("Open File", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Open File",
        file_btn_x + (btn_w - tw) / 2.0,
        btn_y + 18.0,
        Color::rgb8(0x0F, 0x0F, 0x0F),
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(file_btn_rect, ClickAction::OpenFileDialog);

    // Open Folder button
    let folder_btn_x = file_btn_x + btn_w + btn_gap;
    let folder_btn_rect = Rect::new(folder_btn_x, btn_y, folder_btn_x + btn_w, btn_y + btn_h);
    painter.fill_rounded_rect(folder_btn_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(folder_btn_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Open Folder", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Open Folder",
        folder_btn_x + (btn_w - tw) / 2.0,
        btn_y + 18.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(folder_btn_rect, ClickAction::OpenFolderDialog);

    // Recent files section (below drop zone)
    if !state.recent_files.is_empty() {
        let recent_y = cy + dz_h / 2.0 - 20.0;

        painter.draw_text_cached(
            text_cache,
            "Recent Files",
            cx,
            recent_y,
            TEXT_SECONDARY,
            12.0,
            FontWeight::BOLD,
            true,
            false,
        );

        let recent_start_y = recent_y + 20.0;
        let max_recent = 5.min(state.recent_files.len());
        let item_h = 24.0;

        for i in 0..max_recent {
            let path = &state.recent_files[i];
            let ry = recent_start_y + i as f64 * item_h;

            // Truncate path for display
            let display_name: String = path.rsplit('/').next().unwrap_or(path).to_string();

            let item_rect = Rect::new(cx - 140.0, ry, cx + 140.0, ry + item_h);
            painter.fill_rounded_rect(item_rect, Color::rgba8(0x1A, 0x1A, 0x1A, 120), 3.0);

            // File name
            painter.draw_text_cached(
                text_cache,
                &display_name,
                cx - 130.0,
                ry + 16.0,
                TEXT_PRIMARY,
                11.0,
                FontWeight::NORMAL,
                false,
                false,
            );

            // Truncated path
            let short_path: String = path
                .split('/')
                .filter(|s| !s.is_empty())
                .rev()
                .take(2)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("/");
            painter.draw_text_cached(
                text_cache,
                &short_path,
                cx + 130.0,
                ry + 16.0,
                TEXT_MUTED,
                9.0,
                FontWeight::NORMAL,
                false,
                false,
            );

            state.register_click(item_rect, ClickAction::OpenRecentFromEmpty(i));
        }
    }
}
