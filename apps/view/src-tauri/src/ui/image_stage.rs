//! Image stage - displays the loaded image with zoom/pan support.
//!
//! Matches the React `ImageStage` component and the `.image-stage` / `.main-image` CSS.

use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::state::{ClickAction, FitMode, ViewState};

/// Computes empty-state button rects without rendering.
/// Returns a list of (ClickAction, Rect) for Open Image, Open Folder, Open Archive,
/// recent file slots, and the search field.
pub fn empty_state_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    let mut rects = Vec::new();

    // Only compute when no document is loaded
    if state.document.is_some() {
        return rects;
    }

    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let box_w = 400.0_f64.min(size.width - 24.0);

    // Main buttons row: Open Image, Open Folder, Open Archive
    let buttons: Vec<ClickAction> = vec![
        ClickAction::OpenFileDialog,
        ClickAction::OpenFolderDialog,
        ClickAction::OpenArchiveDialog,
    ];
    let btn_w = 100.0;
    let btn_gap = 12.0;
    let total_w = buttons.len() as f64 * btn_w + (buttons.len() - 1) as f64 * btn_gap;
    let start_x = cx - total_w / 2.0;

    for (i, action) in buttons.into_iter().enumerate() {
        let bx = start_x + i as f64 * (btn_w + btn_gap);
        let by = cy + 10.0;
        rects.push((action, Rect::new(bx, by, bx + btn_w, by + 32.0)));
    }

    // Recent file slots
    let max_recent = 5.min(state.recent_files.len());
    let item_h = 20.0;
    let item_w = box_w - 40.0;
    let start_y = cy + 72.0;

    for i in 0..max_recent {
        let iy = start_y + i as f64 * (item_h + 2.0);
        rects.push((
            ClickAction::OpenRecentFile(i),
            Rect::new(cx - item_w / 2.0, iy, cx + item_w / 2.0, iy + item_h),
        ));
    }

    // Search field
    let box_h = 280.0;
    let box_y1 = cy + box_h / 2.0;
    let search_y = box_y1 - 28.0;
    let search_w = box_w - 40.0;
    rects.push((
        ClickAction::ToggleSearch,
        Rect::new(
            cx - search_w / 2.0,
            search_y,
            cx + search_w / 2.0,
            search_y + 22.0,
        ),
    ));

    rects
}

/// Draws the image stage area (centered image with fit/actual modes).
pub fn paint_image_stage(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Background is handled by the main view-immersive paint

    let Some(ref doc) = state.document else {
        paint_empty_state(state, text_cache, &mut painter, size);
        return;
    };
    let dims = doc.dimensions;

    // If we have real image data, render it
    if let Some(ref image_data) = state.current_image_data {
        let img_rect = compute_image_rect(state, size, dims);

        // Checkerboard background for transparent images
        if state.checkerboard_bg {
            paint_checkerboard(&mut painter, img_rect);
        }

        // Draw the actual image using Painter::draw_image
        painter.draw_image(image_data, img_rect);
    } else {
        // No decoded image data yet - show placeholder with filename
        let img_rect = compute_image_rect(state, size, dims);

        // Image placeholder - dark rectangle with filename
        let placeholder_bg = Color::rgb8(0x1A, 0x1A, 0x1A);
        painter.fill_rounded_rect(img_rect, placeholder_bg, 2.0);

        // Draw image filename centered in the placeholder
        if let Some(d) = dims {
            let label = format!("{} x {}", d.width, d.height);
            let tw = text_cache.measure_text_width(&label, 14.0, FontWeight::NORMAL);
            painter.draw_text_cached(
                text_cache,
                &label,
                img_rect.x0 + (img_rect.width() - tw) / 2.0,
                img_rect.y0 + img_rect.height() / 2.0 + 5.0,
                Color::rgb8(0x60, 0xA5, 0xFA),
                14.0,
                FontWeight::NORMAL,
                false,
                false,
            );
        }

        // Draw filename above
        let tw = text_cache.measure_text_width(&doc.file_name, 13.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            &doc.file_name,
            img_rect.x0 + (img_rect.width() - tw) / 2.0,
            img_rect.y0 + img_rect.height() / 2.0 - 15.0,
            Color::rgb8(0xD4, 0xD4, 0xD4),
            13.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
    }
}

/// Computes the display rectangle for the image based on fit mode, zoom, pan, rotation.
pub fn compute_image_rect(
    state: &ViewState,
    viewport: Size,
    dims: Option<super::state::ImageDimensions>,
) -> Rect {
    let d = match dims {
        Some(d) => d,
        None => return Rect::from_origin_size((0.0, 0.0), viewport),
    };

    let nat_w = d.width as f64;
    let nat_h = d.height as f64;

    match state.fit_mode {
        FitMode::Fit => {
            // Fit: scale to fit viewport maintaining aspect ratio
            let scale_x = viewport.width / nat_w;
            let scale_y = viewport.height / nat_h;
            let scale = scale_x.min(scale_y).min(1.0);
            let w = nat_w * scale;
            let h = nat_h * scale;
            let x = (viewport.width - w) / 2.0;
            let y = (viewport.height - h) / 2.0;
            Rect::new(x, y, x + w, y + h)
        }
        FitMode::Actual => {
            // Actual: use zoom and pan
            let w = nat_w * state.zoom;
            let h = nat_h * state.zoom;
            let cx = viewport.width / 2.0 + state.pan_x;
            let cy = viewport.height / 2.0 + state.pan_y;
            Rect::new(cx - w / 2.0, cy - h / 2.0, cx + w / 2.0, cy + h / 2.0)
        }
    }
}

/// Draws a checkerboard pattern behind transparent images.
fn paint_checkerboard(painter: &mut Painter, rect: Rect) {
    let cell: f64 = 8.0;
    let light = Color::rgb8(0xCC, 0xCC, 0xCC);
    let dark = Color::rgb8(0x99, 0x99, 0x99);

    let x0 = rect.x0;
    let y0 = rect.y0;
    let x1 = rect.x1;
    let y1 = rect.y1;

    painter.push_clip(rect);

    let mut cy: f64 = y0;
    let mut row = 0u32;
    while cy < y1 {
        let cell_h = cell.min(y1 - cy);
        let mut cx: f64 = x0;
        let mut col = 0u32;
        while cx < x1 {
            let cell_w = cell.min(x1 - cx);
            let color = if (row + col).is_multiple_of(2) {
                light
            } else {
                dark
            };
            painter.fill_rect(Rect::new(cx, cy, cx + cell_w, cy + cell_h), color);
            cx += cell;
            col += 1;
        }
        cy += cell;
        row += 1;
    }

    painter.pop_clip();
}

/// Draws the empty state when no image is loaded.
/// Matches the React `EmptyState` component and `.empty-state` CSS.
fn paint_empty_state(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    painter: &mut Painter,
    size: Size,
) {
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;

    // Empty state box - matches .empty-state CSS
    let box_w = 400.0_f64.min(size.width - 24.0);
    let box_h = 280.0;
    let box_rect = Rect::new(
        cx - box_w / 2.0,
        cy - box_h / 2.0,
        cx + box_w / 2.0,
        cy + box_h / 2.0,
    );

    // Background
    let bg = Color::rgba8(0x0F, 0x0F, 0x0F, 170);
    painter.fill_rounded_rect(box_rect, bg, 12.0);

    // Border
    painter.stroke_rounded_rect(box_rect, Color::rgb8(0x3A, 0x3A, 0x3A), 1.0, 12.0);

    // Title: "Tench View"
    let title = "Tench View";
    let tw = text_cache.measure_text_width(title, 20.0, FontWeight::BOLD);
    painter.draw_text_cached(
        text_cache,
        title,
        cx - tw / 2.0,
        cy - 80.0,
        Color::rgb8(0x60, 0xA5, 0xFA),
        20.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Subtitle: "No image loaded"
    let subtitle = "No image loaded";
    let tw = text_cache.measure_text_width(subtitle, 14.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        subtitle,
        cx - tw / 2.0,
        cy - 50.0,
        Color::rgb8(0x8A, 0x8A, 0x8A),
        14.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Drop hint
    let hint = "Drop an image here, or use the buttons above";
    let tw = text_cache.measure_text_width(hint, 12.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        hint,
        cx - tw / 2.0,
        cy - 25.0,
        Color::rgb8(0x6A, 0x6A, 0x6A),
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Buttons row
    let buttons = [
        ("Open Image", ClickAction::OpenFileDialog),
        ("Open Folder", ClickAction::OpenFolderDialog),
        ("Open Archive", ClickAction::OpenArchiveDialog),
    ];
    let btn_w = 100.0;
    let btn_gap = 12.0;
    let total_w = buttons.len() as f64 * btn_w + (buttons.len() - 1) as f64 * btn_gap;
    let start_x = cx - total_w / 2.0;

    for (i, (label, action)) in buttons.iter().enumerate() {
        let bx = start_x + i as f64 * (btn_w + btn_gap);
        let by = cy + 10.0;
        let btn_rect = Rect::new(bx, by, bx + btn_w, by + 32.0);

        painter.fill_rounded_rect(btn_rect, Color::rgb8(0x1A, 0x1A, 0x1A), 6.0);
        painter.stroke_rounded_rect(btn_rect, Color::rgb8(0x3A, 0x3A, 0x3A), 1.0, 6.0);

        let tw = text_cache.measure_text_width(label, 12.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            label,
            bx + (btn_w - tw) / 2.0,
            by + 20.0,
            Color::rgb8(0xD4, 0xD4, 0xD4),
            12.0,
            FontWeight::MEDIUM,
            false,
            false,
        );

        // Register click region
        state.register_click(btn_rect, action.clone());
    }

    // Recent files
    if state.recent_files.is_empty() {
        let recent_hint = "Recent files will appear here";
        let tw = text_cache.measure_text_width(recent_hint, 11.0, FontWeight::NORMAL);
        painter.draw_text_cached(
            text_cache,
            recent_hint,
            cx - tw / 2.0,
            cy + 70.0,
            Color::rgb8(0x2A, 0x2A, 0x2A),
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    } else {
        // Show recent files as clickable items
        let recent_label = "Recent";
        let tw = text_cache.measure_text_width(recent_label, 11.0, FontWeight::NORMAL);
        painter.draw_text_cached(
            text_cache,
            recent_label,
            cx - tw / 2.0,
            cy + 58.0,
            Color::rgb8(0x6A, 0x6A, 0x6A),
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        let max_recent = 5.min(state.recent_files.len());
        let item_h = 20.0;
        let item_w = box_w - 40.0;
        let start_y = cy + 72.0;

        for i in 0..max_recent {
            let path = &state.recent_files[i];
            let file_name = path.split(['/', '\\']).next_back().unwrap_or("file");

            let iy = start_y + i as f64 * (item_h + 2.0);
            let item_rect = Rect::new(cx - item_w / 2.0, iy, cx + item_w / 2.0, iy + item_h);

            // Subtle background
            painter.fill_rounded_rect(item_rect, Color::rgba8(0x1A, 0x1A, 0x1A, 120), 3.0);

            // Truncate filename if too long
            let max_chars = ((item_w - 16.0) / 6.5) as usize;
            let display_name = if file_name.len() > max_chars {
                format!("{}...", &file_name[..max_chars.saturating_sub(3)])
            } else {
                file_name.to_string()
            };

            painter.draw_text_cached(
                text_cache,
                &display_name,
                cx - item_w / 2.0 + 8.0,
                iy + 14.0,
                Color::rgb8(0xA0, 0xA0, 0xA0),
                11.0,
                FontWeight::NORMAL,
                false,
                false,
            );

            // Register click region for this recent file
            state.register_click(item_rect, ClickAction::OpenRecentFile(i));
        }
    }

    // Search bar at the bottom of the welcome box
    let search_y = box_rect.y1 - 28.0;
    let search_w = box_w - 40.0;
    let search_rect = Rect::new(
        cx - search_w / 2.0,
        search_y,
        cx + search_w / 2.0,
        search_y + 22.0,
    );
    painter.fill_rounded_rect(search_rect, Color::rgb8(0x1A, 0x1A, 0x1A), 4.0);
    painter.stroke_rounded_rect(search_rect, Color::rgb8(0x3A, 0x3A, 0x3A), 1.0, 4.0);

    let search_label = if state.search_query.is_empty() {
        "Search files..."
    } else {
        &state.search_query
    };
    let search_color = if state.search_query.is_empty() {
        Color::rgb8(0x5A, 0x5A, 0x5A)
    } else {
        Color::rgb8(0xD4, 0xD4, 0xD4)
    };
    painter.draw_text_cached(
        text_cache,
        search_label,
        search_rect.x0 + 8.0,
        search_y + 15.0,
        search_color,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(search_rect, ClickAction::ToggleSearch);
}
