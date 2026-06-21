use super::*;

// Compare Panel

/// Draws the compare panel (before/after).
/// Matches `.compare-panel` CSS: full-screen overlay with split slider.
pub fn paint_compare_panel(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Full-screen overlay
    let bg = Color::rgb8(0x0F, 0x0F, 0x0F);
    painter.fill_background(size, bg);

    // Header bar
    let header_rect = Rect::new(0.0, 0.0, size.width, 44.0);
    painter.fill_rect(header_rect, Color::rgba8(0x0F, 0x0F, 0x0F, 230));

    // Clip header
    painter.push_clip(header_rect);

    painter.draw_text_cached(
        text_cache,
        "Compare",
        16.0,
        26.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Labels
    let left_label = "Original";
    let right_label = "Current";
    painter.draw_text_cached(
        text_cache,
        left_label,
        100.0,
        26.0,
        TEXT_MUTED,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    let tw = text_cache.measure_text_width(right_label, 12.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        right_label,
        size.width - 100.0 - tw,
        26.0,
        TEXT_MUTED,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Close button
    let close_x = size.width - 60.0;
    let close_rect = Rect::new(close_x, 8.0, close_x + 50.0, 36.0);
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(close_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Close",
        close_x + 4.0,
        26.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ToggleCompare);

    // Mode button
    let mode_label = match state.compare_mode {
        CompareMode::Split => "Split",
        CompareMode::SideBySide => "Side",
        CompareMode::Difference => "Diff",
    };
    let mode_x = close_x - 70.0;
    let mode_rect = Rect::new(mode_x, 8.0, mode_x + 60.0, 36.0);
    painter.fill_rounded_rect(mode_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(mode_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width(mode_label, 11.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        mode_label,
        mode_x + (60.0 - tw) / 2.0,
        26.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(mode_rect, ClickAction::CycleCompareMode);

    painter.pop_clip();

    // Render based on compare mode
    let mode = state.compare_mode;
    match mode {
        CompareMode::Split => paint_compare_split(state, text_cache, size, &mut painter),
        CompareMode::SideBySide => {
            paint_compare_side_by_side(state, text_cache, size, &mut painter)
        }
        CompareMode::Difference => paint_compare_diff(state, text_cache, size, &mut painter),
    }
}

fn paint_compare_split(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    painter: &mut Painter,
) {
    let split_x = size.width * state.compare_split / 100.0;
    let viewport_y = 44.0;
    let viewport_h = size.height - viewport_y;

    // Draw original image on the left side (clipped)
    if let Some(ref original_data) = state.original_image_data {
        let img_w = original_data.width as f64;
        let img_h = original_data.height as f64;
        let scale_x = split_x / img_w;
        let scale_y = viewport_h / img_h;
        let scale = scale_x.min(scale_y).min(1.0);
        let display_w = img_w * scale;
        let display_h = img_h * scale;
        let img_x = (split_x - display_w) / 2.0;
        let img_y = viewport_y + (viewport_h - display_h) / 2.0;

        let left_clip = Rect::new(0.0, viewport_y, split_x, size.height);
        painter.push_clip(left_clip);
        let img_rect = Rect::new(img_x, img_y, img_x + display_w, img_y + display_h);
        painter.draw_image(original_data, img_rect);
        painter.pop_clip();
    } else {
        // No original data - just show dimmed background
        let left_rect = Rect::new(0.0, viewport_y, split_x, size.height);
        painter.fill_rect(left_rect, Color::rgb8(0x1A, 0x1A, 0x1A));
    }

    // Draw current image on the right side (clipped)
    if let Some(ref current_data) = state.current_image_data {
        let right_w = size.width - split_x;
        let img_w = current_data.width as f64;
        let img_h = current_data.height as f64;
        let scale_x = right_w / img_w;
        let scale_y = viewport_h / img_h;
        let scale = scale_x.min(scale_y).min(1.0);
        let display_w = img_w * scale;
        let display_h = img_h * scale;
        let img_x = split_x + (right_w - display_w) / 2.0;
        let img_y = viewport_y + (viewport_h - display_h) / 2.0;

        let right_clip = Rect::new(split_x, viewport_y, size.width, size.height);
        painter.push_clip(right_clip);
        let img_rect = Rect::new(img_x, img_y, img_x + display_w, img_y + display_h);
        painter.draw_image(current_data, img_rect);
        painter.pop_clip();
    } else {
        // No current data - just show dimmed background
        let right_rect = Rect::new(split_x, viewport_y, size.width, size.height);
        painter.fill_rect(right_rect, Color::rgb8(0x1E, 0x1E, 0x1E));
    }

    // Labels on each side
    painter.draw_text_cached(
        text_cache,
        "Original",
        20.0,
        viewport_y + 30.0,
        TEXT_MUTED,
        24.0,
        FontWeight::BOLD,
        false,
        false,
    );
    painter.draw_text_cached(
        text_cache,
        "Current",
        split_x + 20.0,
        viewport_y + 30.0,
        TEXT_MUTED,
        24.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Slider line
    painter.draw_line(
        Point::new(split_x, viewport_y),
        Point::new(split_x, size.height),
        ACCENT_VIEW,
        3.0,
    );

    // Slider handle
    let handle_y = viewport_y + viewport_h / 2.0;
    let handle_r = 12.0;
    let handle_rect = Rect::new(
        split_x - handle_r,
        handle_y - handle_r,
        split_x + handle_r,
        handle_y + handle_r,
    );
    painter.fill_rounded_rect(handle_rect, ACCENT_VIEW, handle_r);
    painter.stroke_rounded_rect(handle_rect, TEXT_PRIMARY, 2.0, handle_r);

    // Register a wider click area around the split line for dragging
    let drag_hit_area = Rect::new(split_x - 20.0, viewport_y, split_x + 20.0, size.height);
    state.register_click(drag_hit_area, ClickAction::CompareDragStart);
}

fn paint_compare_side_by_side(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    painter: &mut Painter,
) {
    let viewport_y = 44.0;
    let viewport_h = size.height - viewport_y;
    let half_w = size.width / 2.0;

    // Left: Original
    if let Some(ref original_data) = state.original_image_data {
        let img_w = original_data.width as f64;
        let img_h = original_data.height as f64;
        let scale_x = (half_w - 20.0) / img_w;
        let scale_y = (viewport_h - 40.0) / img_h;
        let scale = scale_x.min(scale_y).min(1.0);
        let display_w = img_w * scale;
        let display_h = img_h * scale;
        let img_x = (half_w - display_w) / 2.0;
        let img_y = viewport_y + (viewport_h - display_h) / 2.0;
        let img_rect = Rect::new(img_x, img_y, img_x + display_w, img_y + display_h);
        painter.draw_image(original_data, img_rect);
    } else {
        painter.fill_rect(
            Rect::new(0.0, viewport_y, half_w, size.height),
            Color::rgb8(0x1A, 0x1A, 0x1A),
        );
    }

    // Divider line
    painter.draw_line(
        Point::new(half_w, viewport_y),
        Point::new(half_w, size.height),
        BORDER_COLOR,
        1.0,
    );

    // Right: Current
    if let Some(ref current_data) = state.current_image_data {
        let img_w = current_data.width as f64;
        let img_h = current_data.height as f64;
        let scale_x = (half_w - 20.0) / img_w;
        let scale_y = (viewport_h - 40.0) / img_h;
        let scale = scale_x.min(scale_y).min(1.0);
        let display_w = img_w * scale;
        let display_h = img_h * scale;
        let img_x = half_w + (half_w - display_w) / 2.0;
        let img_y = viewport_y + (viewport_h - display_h) / 2.0;
        let img_rect = Rect::new(img_x, img_y, img_x + display_w, img_y + display_h);
        painter.draw_image(current_data, img_rect);
    } else {
        painter.fill_rect(
            Rect::new(half_w, viewport_y, size.width, size.height),
            Color::rgb8(0x1E, 0x1E, 0x1E),
        );
    }

    // Labels
    painter.draw_text_cached(
        text_cache,
        "Original",
        20.0,
        viewport_y + 30.0,
        TEXT_MUTED,
        24.0,
        FontWeight::BOLD,
        false,
        false,
    );
    painter.draw_text_cached(
        text_cache,
        "Current",
        half_w + 20.0,
        viewport_y + 30.0,
        TEXT_MUTED,
        24.0,
        FontWeight::BOLD,
        false,
        false,
    );
}

fn paint_compare_diff(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    painter: &mut Painter,
) {
    let viewport_y = 44.0;
    let viewport_h = size.height - viewport_y;

    // Show current image
    if let Some(ref current_data) = state.current_image_data {
        let img_w = current_data.width as f64;
        let img_h = current_data.height as f64;
        let scale_x = size.width / img_w;
        let scale_y = viewport_h / img_h;
        let scale = scale_x.min(scale_y).min(1.0);
        let display_w = img_w * scale;
        let display_h = img_h * scale;
        let img_x = (size.width - display_w) / 2.0;
        let img_y = viewport_y + (viewport_h - display_h) / 2.0;
        let img_rect = Rect::new(img_x, img_y, img_x + display_w, img_y + display_h);
        painter.draw_image(current_data, img_rect);

        // Overlay a red-tinted diff indicator
        if state.original_image_data.is_some() {
            let diff_rect = Rect::new(img_x, img_y, img_x + display_w, img_y + display_h);
            painter.fill_rounded_rect(diff_rect, Color::rgba8(0xFF, 0x00, 0x00, 30), 0.0);
        }
    }

    // Label
    painter.draw_text_cached(
        text_cache,
        "Difference View",
        20.0,
        viewport_y + 30.0,
        TEXT_MUTED,
        24.0,
        FontWeight::BOLD,
        false,
        false,
    );
}
