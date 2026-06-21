use super::*;

// Bottom Overlay

/// Draws the bottom overlay bar with zoom controls and filmstrip toggle.
/// Matches `.bottom-overlay` CSS.
pub fn paint_bottom_overlay(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let overlay_h = if state.show_thumbnails && !state.sorted_entries.is_empty() {
        120.0
    } else {
        60.0
    };
    let overlay_y = size.height - overlay_h;
    let overlay_rect = Rect::new(0.0, overlay_y, size.width, size.height);

    // Gradient background
    painter.fill_rect(overlay_rect, OVERLAY_BG);

    // Clip to overlay bounds
    painter.push_clip(overlay_rect);

    // View controls row
    let ctrl_y = overlay_y + 10.0;
    let ctrl_h = 28.0;
    let btn_w = 52.0;
    let btn_gap = 6.0;

    let zoom_label = match state.fit_mode {
        FitMode::Fit => "Fit".to_string(),
        FitMode::Actual => format!("{:.0}%", state.zoom * 100.0),
    };

    let bg_label = match state.bg_color {
        super::state::BgColor::Black => "BG: Black",
        super::state::BgColor::Gray => "BG: Gray",
        super::state::BgColor::White => "BG: White",
    };

    let controls: Vec<(&str, ClickAction)> = vec![
        ("Fit", ClickAction::ZoomFit),
        ("100%", ClickAction::ZoomActual),
        ("-", ClickAction::ZoomOut),
        (&zoom_label, ClickAction::ZoomFit),
        ("+", ClickAction::ZoomIn),
        ("Film", ClickAction::ToggleThumbnails),
        ("Rotate", ClickAction::Rotate),
        (bg_label, ClickAction::CycleBgColor),
        ("Full", ClickAction::ToggleFullscreen),
        ("Share", ClickAction::ShareImage),
        ("Wallpaper", ClickAction::SetWallpaperAction),
        ("Del", ClickAction::DeleteFromToolbar),
    ];

    let total_w = controls.len() as f64 * (btn_w + btn_gap);
    let mut x = (size.width - total_w) / 2.0;

    for (label, action) in &controls {
        let btn_rect = Rect::new(x, ctrl_y, x + btn_w, ctrl_y + ctrl_h);
        painter.fill_rounded_rect(btn_rect, BTN_BG, 4.0);
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);

        let tw = text_cache.measure_text_width(label, 11.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            label,
            x + (btn_w - tw) / 2.0,
            ctrl_y + ctrl_h / 2.0 + 4.0,
            TEXT_PRIMARY,
            11.0,
            FontWeight::MEDIUM,
            false,
            false,
        );

        // Register click region
        state.register_click(btn_rect, action.clone());

        x += btn_w + btn_gap;
    }

    // Pixel info on the left side
    if let Some(ref info) = state.pixel_info {
        let swatch_size = 14.0;
        let swatch_x = 20.0;
        let swatch_y = ctrl_y + ctrl_h + 6.0;
        let swatch_rect = Rect::new(
            swatch_x,
            swatch_y,
            swatch_x + swatch_size,
            swatch_y + swatch_size,
        );
        let pixel_color = Color::rgba8(info.r, info.g, info.b, info.a);
        painter.fill_rounded_rect(swatch_rect, pixel_color, 2.0);
        painter.stroke_rounded_rect(swatch_rect, BORDER_COLOR, 1.0, 2.0);

        let hex = format!("#{:02X}{:02X}{:02X}", info.r, info.g, info.b);
        let pixel_text = format!(
            "{},{}  {}  rgba({},{},{},{})",
            info.x, info.y, hex, info.r, info.g, info.b, info.a
        );
        painter.draw_text_cached(
            text_cache,
            &pixel_text,
            swatch_x + swatch_size + 6.0,
            swatch_y + 11.0,
            TEXT_MUTED,
            10.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    }

    // Status message on the right
    if let Some(ref doc) = state.document {
        let status = format!(
            "{} \u{00B7} {}",
            bytes_label(doc.file_size),
            &state.status_message
        );
        let tw = text_cache.measure_text_width(&status, 11.0, FontWeight::NORMAL);
        painter.draw_text_cached(
            text_cache,
            &status,
            size.width - tw - 20.0,
            ctrl_y + ctrl_h + 16.0,
            TEXT_MUTED,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    }

    // Filmstrip area
    if state.show_thumbnails && !state.sorted_entries.is_empty() {
        paint_filmstrip(state, text_cache, size, overlay_y + 46.0, &mut painter);
    }

    painter.pop_clip();
}

/// Draws the horizontal filmstrip of thumbnails with virtual scrolling.
/// Matches `.filmstrip` CSS.
fn paint_filmstrip(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    y_start: f64,
    painter: &mut Painter,
) {
    let thumb_size = 48.0;
    let thumb_gap = 8.0;
    let item_w = thumb_size + thumb_gap;
    let max_visible = ((size.width - 20.0) / item_w).floor() as usize;
    let total_count = state.sorted_entries.len();
    let count = total_count.min(max_visible);

    // Virtual scrolling: center the visible window around the selected item
    let selected_idx = state.selected_index().unwrap_or(0);
    let half = count / 2;
    let vis_start = selected_idx.saturating_sub(half);
    let vis_end = (vis_start + count).min(total_count);
    let vis_start = vis_end.saturating_sub(count);

    let total_w = count as f64 * item_w;
    let start_x = (size.width - total_w) / 2.0;

    for vi in 0..count {
        let i = vis_start + vi;
        if i >= total_count {
            break;
        }
        let x = start_x + vi as f64 * item_w;
        let entry = &state.sorted_entries[i];
        let is_active = selected_idx == i;

        let thumb_rect = Rect::new(x, y_start, x + thumb_size, y_start + thumb_size);

        // Background
        painter.fill_rounded_rect(thumb_rect, BTN_BG, 4.0);

        // Draw real thumbnail image if available
        if let Some(thumb_data) = state.thumbnail_cache.get(&entry.path) {
            painter.draw_image(thumb_data, thumb_rect);
        } else {
            // Fallback: draw file initials
            let name = &entry.file_name;
            let initials: String = name
                .split(|c: char| c.is_whitespace() || c == '.' || c == '_' || c == '-')
                .filter_map(|s| s.chars().next())
                .take(2)
                .collect::<String>()
                .to_uppercase();

            let tw = text_cache.measure_text_width(&initials, 11.0, FontWeight::MEDIUM);
            painter.draw_text_cached(
                text_cache,
                &initials,
                x + (thumb_size - tw) / 2.0,
                y_start + thumb_size / 2.0 + 4.0,
                TEXT_SECONDARY,
                11.0,
                FontWeight::MEDIUM,
                false,
                false,
            );
        }

        // Selected highlight: accent border
        if is_active {
            painter.stroke_rounded_rect(thumb_rect, ACCENT_VIEW, 2.5, 4.0);
        } else {
            painter.stroke_rounded_rect(thumb_rect, BORDER_COLOR, 1.0, 4.0);
        }

        // Register click region for filmstrip navigation
        state.register_click(thumb_rect, ClickAction::NavigateToIndex(i));
    }
}
