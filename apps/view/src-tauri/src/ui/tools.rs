//! Edit tools - crop, resize, convert.
//!
//! Matches the React `CropTool`, `ResizeTool`, `ConvertTool` components.

use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::state::{ClickAction, ViewState};
use super::theme::{
    ACCENT_VIEW, BORDER_COLOR, BTN_BG, INPUT_BG, PANEL_BG, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY, TOOLBAR_BG,
};

// Crop Tool

/// Draws the crop tool overlay.
/// Matches `.crop-tool` CSS: full-screen, toolbar at top, canvas below.
pub fn paint_crop_tool(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    // Full-screen dark background
    painter.fill_background(size, Color::rgb8(0x0F, 0x0F, 0x0F));

    // Toolbar at top - matches `.crop-toolbar`
    let toolbar_h = 44.0;
    let toolbar_rect = Rect::new(0.0, 0.0, size.width, toolbar_h);
    painter.fill_rect(toolbar_rect, TOOLBAR_BG);

    // Clip toolbar content
    painter.push_clip(toolbar_rect);

    // Selection info or hint
    let info_text = if let Some((_x, _y, w, h)) = state.crop_selection {
        if w > 2.0 && h > 2.0 {
            format!("Crop: {:.0} x {:.0}", w, h)
        } else {
            "Click and drag to select crop area".to_string()
        }
    } else {
        "Click and drag to select crop area".to_string()
    };

    painter.draw_text_cached(
        text_cache,
        &info_text,
        16.0,
        26.0,
        TEXT_SECONDARY,
        13.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Cancel button
    let cancel_x = size.width - 160.0;
    let cancel_rect = Rect::new(cancel_x, 8.0, cancel_x + 60.0, 36.0);
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Cancel", 12.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + (60.0 - tw) / 2.0,
        28.0,
        TEXT_PRIMARY,
        12.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::CropCancel);

    // Apply button
    let apply_x = size.width - 80.0;
    let has_selection = state
        .crop_selection
        .is_some_and(|(_, _, w, h)| w > 2.0 && h > 2.0);
    let apply_bg = if has_selection { ACCENT_VIEW } else { BTN_BG };
    let apply_rect = Rect::new(apply_x, 8.0, apply_x + 60.0, 36.0);
    painter.fill_rounded_rect(apply_rect, apply_bg, 4.0);
    painter.stroke_rounded_rect(apply_rect, BORDER_COLOR, 1.0, 4.0);
    let apply_color = if has_selection {
        Color::rgb8(0x0F, 0x0F, 0x0F)
    } else {
        TEXT_MUTED
    };
    let tw = text_cache.measure_text_width("Apply", 12.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Apply",
        apply_x + (60.0 - tw) / 2.0,
        28.0,
        apply_color,
        12.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(apply_rect, ClickAction::CropApply);

    painter.pop_clip();

    // Canvas area - draw the actual image
    if let Some(ref image_data) = state.current_image_data {
        let img_w_native = image_data.width as f64;
        let img_h_native = image_data.height as f64;

        let canvas_y = toolbar_h;
        let canvas_w = size.width;
        let canvas_h = size.height - toolbar_h;

        // Compute display scale to fit
        let scale_x = canvas_w / img_w_native;
        let scale_y = canvas_h / img_h_native;
        let scale = scale_x.min(scale_y).min(1.0);
        let img_w = img_w_native * scale;
        let img_h = img_h_native * scale;
        let img_x = (canvas_w - img_w) / 2.0;
        let img_y = canvas_y + (canvas_h - img_h) / 2.0;

        // Draw the actual image
        let img_rect = Rect::new(img_x, img_y, img_x + img_w, img_y + img_h);
        painter.draw_image(image_data, img_rect);

        // Crop selection overlay
        if let Some((sx, sy, sw, sh)) = state.crop_selection {
            if sw > 2.0 && sh > 2.0 {
                let sel_x = img_x + sx * scale;
                let sel_y = img_y + sy * scale;
                let sel_w = sw * scale;
                let sel_h = sh * scale;

                // Dim outside selection
                let dim_color = Color::rgba8(0, 0, 0, 128);

                // Top
                painter.fill_rect(Rect::new(img_x, img_y, img_x + img_w, sel_y), dim_color);
                // Bottom
                painter.fill_rect(
                    Rect::new(img_x, sel_y + sel_h, img_x + img_w, img_y + img_h),
                    dim_color,
                );
                // Left
                painter.fill_rect(Rect::new(img_x, sel_y, sel_x, sel_y + sel_h), dim_color);
                // Right
                painter.fill_rect(
                    Rect::new(sel_x + sel_w, sel_y, img_x + img_w, sel_y + sel_h),
                    dim_color,
                );

                // Selection border
                let sel_rect = Rect::new(sel_x, sel_y, sel_x + sel_w, sel_y + sel_h);
                painter.stroke_rounded_rect(sel_rect, ACCENT_VIEW, 2.0, 0.0);

                // Fill inside selection
                painter.fill_rounded_rect(sel_rect, Color::rgba8(0x60, 0xA5, 0xFA, 30), 0.0);
            }
        }
    }

    // Aspect ratio selector buttons at the bottom of the crop toolbar
    let aspect_specs: &[(&str, ClickAction)] = &[
        ("Free", ClickAction::CropAspectRatioFree),
        ("16:9", ClickAction::CropAspectRatio(16, 9)),
        ("4:3", ClickAction::CropAspectRatio(4, 3)),
        ("1:1", ClickAction::CropAspectRatio(1, 1)),
        ("3:2", ClickAction::CropAspectRatio(3, 2)),
    ];
    let aspect_btn_w = 44.0;
    let aspect_btn_h = 22.0;
    let aspect_btn_gap = 4.0;
    let total_aspect_w =
        aspect_specs.len() as f64 * aspect_btn_w + (aspect_specs.len() - 1) as f64 * aspect_btn_gap;
    let mut ax = (size.width - total_aspect_w) / 2.0;
    let ay = size.height - aspect_btn_h - 12.0;

    for (label, action) in aspect_specs {
        let is_active = match action {
            ClickAction::CropAspectRatioFree => state.crop_aspect_ratio.is_none(),
            ClickAction::CropAspectRatio(rw, rh) => state.crop_aspect_ratio == Some((*rw, *rh)),
            _ => false,
        };
        let btn_rect = Rect::new(ax, ay, ax + aspect_btn_w, ay + aspect_btn_h);
        let bg = if is_active { ACCENT_VIEW } else { BTN_BG };
        let text_c = if is_active {
            Color::rgb8(0x0F, 0x0F, 0x0F)
        } else {
            TEXT_PRIMARY
        };
        painter.fill_rounded_rect(btn_rect, bg, 4.0);
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);
        let tw = text_cache.measure_text_width(label, 10.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            label,
            ax + (aspect_btn_w - tw) / 2.0,
            ay + aspect_btn_h / 2.0 + 3.5,
            text_c,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(btn_rect, action.clone());
        ax += aspect_btn_w + aspect_btn_gap;
    }
}

// Resize Tool

/// Draws the resize tool dialog.
/// Matches `.resize-tool` CSS: 280px wide, top-right, fields and actions.
pub fn paint_resize_tool(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let panel_w = 280.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;
    let panel_h = 280.0;

    let panel_rect = Rect::new(panel_x, panel_y, panel_x + panel_w, panel_y + panel_h);

    // Background
    painter.fill_rounded_rect(panel_rect, PANEL_BG, 8.0);
    painter.stroke_rounded_rect(panel_rect, BORDER_COLOR, 1.0, 8.0);

    // Clip to panel bounds
    painter.push_clip(panel_rect);

    let pad = 16.0;
    let mut y = panel_y + pad;

    // Title
    painter.draw_text_cached(
        text_cache,
        "Resize Image",
        panel_x + pad,
        y + 8.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 30.0;

    // Width field with +/- buttons
    painter.draw_text_cached(
        text_cache,
        "Width",
        panel_x + pad,
        y + 8.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Minus button for width
    let w_minus_rect = Rect::new(panel_x + pad, y + 14.0, panel_x + pad + 24.0, y + 42.0);
    painter.fill_rounded_rect(w_minus_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(w_minus_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "-",
        panel_x + pad + 8.0,
        y + 34.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );
    state.register_click(w_minus_rect, ClickAction::ResizeWidthMinus);

    // Width value display
    let w_val_x = panel_x + pad + 28.0;
    let w_val_w = panel_w / 2.0 - 36.0;
    let w_input = Rect::new(w_val_x, y + 14.0, w_val_x + w_val_w, y + 42.0);
    painter.fill_rounded_rect(w_input, INPUT_BG, 4.0);
    painter.stroke_rounded_rect(w_input, BORDER_COLOR, 1.0, 4.0);
    let w_str = state.resize_width.to_string();
    let tw = text_cache.measure_text_width(&w_str, 13.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        &w_str,
        w_val_x + (w_val_w - tw) / 2.0,
        y + 34.0,
        TEXT_PRIMARY,
        13.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Plus button for width
    let w_plus_x = w_val_x + w_val_w + 4.0;
    let w_plus_rect = Rect::new(w_plus_x, y + 14.0, w_plus_x + 24.0, y + 42.0);
    painter.fill_rounded_rect(w_plus_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(w_plus_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "+",
        w_plus_x + 7.0,
        y + 34.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );
    state.register_click(w_plus_rect, ClickAction::ResizeWidthPlus);

    // Height field with +/- buttons
    let h_section_x = panel_x + panel_w / 2.0 + 4.0;
    painter.draw_text_cached(
        text_cache,
        "Height",
        h_section_x,
        y + 8.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Minus button for height
    let h_minus_rect = Rect::new(h_section_x, y + 14.0, h_section_x + 24.0, y + 42.0);
    painter.fill_rounded_rect(h_minus_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(h_minus_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "-",
        h_section_x + 8.0,
        y + 34.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );
    state.register_click(h_minus_rect, ClickAction::ResizeHeightMinus);

    // Height value display
    let h_val_x = h_section_x + 28.0;
    let h_val_w = panel_w / 2.0 - 36.0 - 8.0;
    let h_input = Rect::new(h_val_x, y + 14.0, h_val_x + h_val_w, y + 42.0);
    painter.fill_rounded_rect(h_input, INPUT_BG, 4.0);
    painter.stroke_rounded_rect(h_input, BORDER_COLOR, 1.0, 4.0);
    let h_str = state.resize_height.to_string();
    let tw = text_cache.measure_text_width(&h_str, 13.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        &h_str,
        h_val_x + (h_val_w - tw) / 2.0,
        y + 34.0,
        TEXT_PRIMARY,
        13.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Plus button for height
    let h_plus_x = h_val_x + h_val_w + 4.0;
    let h_plus_rect = Rect::new(h_plus_x, y + 14.0, h_plus_x + 24.0, y + 42.0);
    painter.fill_rounded_rect(h_plus_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(h_plus_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "+",
        h_plus_x + 7.0,
        y + 34.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );
    state.register_click(h_plus_rect, ClickAction::ResizeHeightPlus);

    y += 56.0;

    // Maintain aspect checkbox (clickable)
    let check_label = if state.resize_maintain_aspect {
        "\u{2611}"
    } else {
        "\u{2610}"
    };
    let aspect_rect = Rect::new(panel_x + pad, y, panel_x + pad + 180.0, y + 20.0);
    painter.draw_text_cached(
        text_cache,
        check_label,
        panel_x + pad,
        y + 8.0,
        ACCENT_VIEW,
        14.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    painter.draw_text_cached(
        text_cache,
        "Maintain aspect ratio",
        panel_x + pad + 22.0,
        y + 8.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(aspect_rect, ClickAction::ResizeToggleAspect);
    y += 24.0;

    // Preview
    let preview = format!("{} x {}", state.resize_width, state.resize_height);
    let tw = text_cache.measure_text_width(&preview, 12.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        &preview,
        panel_x + (panel_w - tw) / 2.0,
        y + 8.0,
        TEXT_MUTED,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    y += 28.0;

    // Action buttons
    let cancel_x = panel_x + panel_w - pad - 140.0;
    let apply_x = panel_x + panel_w - pad - 60.0;
    let btn_y = y;
    let btn_h = 28.0;

    // Cancel button
    let cancel_rect = Rect::new(cancel_x, btn_y, cancel_x + 60.0, btn_y + btn_h);
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Cancel", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + (60.0 - tw) / 2.0,
        btn_y + 18.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::ResizeCancel);

    // Apply button
    let apply_rect = Rect::new(apply_x, btn_y, apply_x + 60.0, btn_y + btn_h);
    painter.fill_rounded_rect(apply_rect, ACCENT_VIEW, 4.0);
    let tw = text_cache.measure_text_width("Apply", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Apply",
        apply_x + (60.0 - tw) / 2.0,
        btn_y + 18.0,
        Color::rgb8(0x0F, 0x0F, 0x0F),
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(apply_rect, ClickAction::ResizeApply);

    painter.pop_clip();
}

// Convert Tool

/// Draws the convert tool dialog.
/// Matches `.convert-tool` CSS: 280px wide, top-right, format buttons.
pub fn paint_convert_tool(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let panel_w = 280.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;
    let panel_h = 260.0;

    let panel_rect = Rect::new(panel_x, panel_y, panel_x + panel_w, panel_y + panel_h);

    // Background
    painter.fill_rounded_rect(panel_rect, PANEL_BG, 8.0);
    painter.stroke_rounded_rect(panel_rect, BORDER_COLOR, 1.0, 8.0);

    // Clip to panel bounds
    painter.push_clip(panel_rect);

    let pad = 16.0;
    let mut y = panel_y + pad;

    // Title
    painter.draw_text_cached(
        text_cache,
        "Convert Format",
        panel_x + pad,
        y + 8.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );
    y += 28.0;

    // Hint
    painter.draw_text_cached(
        text_cache,
        "Select target format:",
        panel_x + pad,
        y + 8.0,
        TEXT_SECONDARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    y += 24.0;

    // Format buttons
    let formats = ["png", "jpg", "webp", "bmp", "tiff"];
    let btn_w = 48.0;
    let btn_h = 28.0;
    let btn_gap = 6.0;
    let total_btn_w = formats.len() as f64 * (btn_w + btn_gap) - btn_gap;
    let mut bx = panel_x + (panel_w - total_btn_w) / 2.0;

    for fmt in &formats {
        let is_active_fmt = *fmt == state.convert_format;
        let bg = if is_active_fmt { ACCENT_VIEW } else { BTN_BG };
        let text_c = if is_active_fmt {
            Color::rgb8(0x0F, 0x0F, 0x0F)
        } else {
            TEXT_PRIMARY
        };

        let btn_rect = Rect::new(bx, y, bx + btn_w, y + btn_h);
        painter.fill_rounded_rect(btn_rect, bg, 4.0);
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);

        let label = fmt.to_uppercase();
        let tw = text_cache.measure_text_width(&label, 11.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            &label,
            bx + (btn_w - tw) / 2.0,
            y + 18.0,
            text_c,
            11.0,
            FontWeight::MEDIUM,
            false,
            false,
        );

        state.register_click(btn_rect, ClickAction::ConvertSelectFormat(fmt.to_string()));

        bx += btn_w + btn_gap;
    }

    y += btn_h + 16.0;

    // Output path section
    painter.draw_text_cached(
        text_cache,
        "Save to:",
        panel_x + pad,
        y + 8.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    y += 20.0;

    let path_display = state
        .convert_output_path
        .as_deref()
        .unwrap_or("Same as source");
    let display_truncated = if path_display.chars().count() > 24 {
        let t: String = path_display.chars().rev().take(24).collect();
        format!("...{}", t.chars().rev().collect::<String>())
    } else {
        path_display.to_string()
    };

    let path_rect = Rect::new(panel_x + pad, y, panel_x + panel_w - pad - 70.0, y + 24.0);
    painter.fill_rounded_rect(path_rect, Color::rgba8(0x0A, 0x0A, 0x0A, 60), 4.0);
    painter.stroke_rounded_rect(path_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        &display_truncated,
        panel_x + pad + 6.0,
        y + 16.0,
        TEXT_MUTED,
        10.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Browse button
    let browse_x = panel_x + panel_w - pad - 60.0;
    let browse_rect = Rect::new(browse_x, y, browse_x + 60.0, y + 24.0);
    painter.fill_rounded_rect(browse_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(browse_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Browse", 10.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Browse",
        browse_x + (60.0 - tw) / 2.0,
        y + 16.0,
        TEXT_PRIMARY,
        10.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(browse_rect, ClickAction::ConvertBrowseOutput);

    y += 36.0;

    // Action buttons
    let cancel_x = panel_x + panel_w - pad - 140.0;
    let apply_x = panel_x + panel_w - pad - 60.0;

    // Cancel button
    let cancel_rect = Rect::new(cancel_x, y, cancel_x + 60.0, y + 28.0);
    painter.fill_rounded_rect(cancel_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(cancel_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Cancel", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        cancel_x + (60.0 - tw) / 2.0,
        y + 18.0,
        TEXT_PRIMARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(cancel_rect, ClickAction::ConvertCancel);

    // Apply button
    let apply_rect = Rect::new(apply_x, y, apply_x + 60.0, y + 28.0);
    painter.fill_rounded_rect(apply_rect, ACCENT_VIEW, 4.0);
    let tw = text_cache.measure_text_width("Apply", 11.0, FontWeight::MEDIUM);
    painter.draw_text_cached(
        text_cache,
        "Apply",
        apply_x + (60.0 - tw) / 2.0,
        y + 18.0,
        Color::rgb8(0x0F, 0x0F, 0x0F),
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(apply_rect, ClickAction::ConvertApply);

    painter.pop_clip();
}

// ---------------------------------------------------------------------------
// Rect computation helpers (automation fallback)
// ---------------------------------------------------------------------------

/// Computes crop tool button rects without rendering.
pub fn crop_tool_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if state.active_edit_tool != Some(super::state::EditTool::Crop) {
        return Vec::new();
    }
    let cancel_x = size.width - 160.0;
    let apply_x = size.width - 80.0;
    vec![
        (
            ClickAction::CropCancel,
            Rect::new(cancel_x, 8.0, cancel_x + 60.0, 36.0),
        ),
        (
            ClickAction::CropApply,
            Rect::new(apply_x, 8.0, apply_x + 60.0, 36.0),
        ),
    ]
}

/// Computes resize tool button rects without rendering.
pub fn resize_tool_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if state.active_edit_tool != Some(super::state::EditTool::Resize) {
        return Vec::new();
    }
    let panel_w = 280.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;
    let pad = 16.0;
    let mut y = panel_y + pad + 30.0;

    let mut rects = Vec::new();

    // Width minus/plus
    let w_minus_rect = Rect::new(panel_x + pad, y + 14.0, panel_x + pad + 24.0, y + 42.0);
    rects.push((ClickAction::ResizeWidthMinus, w_minus_rect));

    let w_val_x = panel_x + pad + 28.0;
    let w_val_w = panel_w / 2.0 - 36.0;
    let w_plus_x = w_val_x + w_val_w + 4.0;
    rects.push((
        ClickAction::ResizeWidthPlus,
        Rect::new(w_plus_x, y + 14.0, w_plus_x + 24.0, y + 42.0),
    ));

    // Height minus/plus
    let h_section_x = panel_x + panel_w / 2.0 + 4.0;
    let h_minus_rect = Rect::new(h_section_x, y + 14.0, h_section_x + 24.0, y + 42.0);
    rects.push((ClickAction::ResizeHeightMinus, h_minus_rect));

    let h_val_x = h_section_x + 28.0;
    let h_val_w = panel_w / 2.0 - 36.0 - 8.0;
    let h_plus_x = h_val_x + h_val_w + 4.0;
    rects.push((
        ClickAction::ResizeHeightPlus,
        Rect::new(h_plus_x, y + 14.0, h_plus_x + 24.0, y + 42.0),
    ));

    y += 56.0;

    // Aspect ratio toggle
    let aspect_rect = Rect::new(panel_x + pad, y, panel_x + pad + 180.0, y + 20.0);
    rects.push((ClickAction::ResizeToggleAspect, aspect_rect));

    y += 24.0 + 28.0;

    // Cancel/Apply
    let cancel_x = panel_x + panel_w - pad - 140.0;
    let apply_x = panel_x + panel_w - pad - 60.0;
    rects.push((
        ClickAction::ResizeCancel,
        Rect::new(cancel_x, y, cancel_x + 60.0, y + 28.0),
    ));
    rects.push((
        ClickAction::ResizeApply,
        Rect::new(apply_x, y, apply_x + 60.0, y + 28.0),
    ));

    rects
}

/// Computes convert tool button rects without rendering.
pub fn convert_tool_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if state.active_edit_tool != Some(super::state::EditTool::Convert) {
        return Vec::new();
    }
    let panel_w = 280.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;
    let pad = 16.0;
    let mut y = panel_y + pad + 28.0 + 24.0;

    let mut rects = Vec::new();

    // Format buttons
    let formats = ["png", "jpg", "webp", "bmp", "tiff"];
    let btn_w = 48.0;
    let btn_gap = 6.0;
    let total_btn_w = formats.len() as f64 * (btn_w + btn_gap) - btn_gap;
    let mut bx = panel_x + (panel_w - total_btn_w) / 2.0;
    for fmt in &formats {
        rects.push((
            ClickAction::ConvertSelectFormat(fmt.to_string()),
            Rect::new(bx, y, bx + btn_w, y + 28.0),
        ));
        bx += btn_w + btn_gap;
    }

    y += 28.0 + 16.0 + 20.0;

    // Browse
    let browse_x = panel_x + panel_w - pad - 60.0;
    rects.push((
        ClickAction::ConvertBrowseOutput,
        Rect::new(browse_x, y, browse_x + 60.0, y + 24.0),
    ));

    y += 36.0;

    // Cancel/Apply
    let cancel_x = panel_x + panel_w - pad - 140.0;
    let apply_x = panel_x + panel_w - pad - 60.0;
    rects.push((
        ClickAction::ConvertCancel,
        Rect::new(cancel_x, y, cancel_x + 60.0, y + 28.0),
    ));
    rects.push((
        ClickAction::ConvertApply,
        Rect::new(apply_x, y, apply_x + 60.0, y + 28.0),
    ));

    rects
}
