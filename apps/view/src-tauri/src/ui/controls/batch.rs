use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::super::state::{ClickAction, ViewState};
use super::super::theme::{
    ACCENT_VIEW, BORDER_COLOR, BTN_BG, INPUT_BG, PANEL_BG, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

// Batch Panel

/// Draws the batch processing panel.
/// Matches `.batch-panel` CSS: 340px wide, right side, full height.
pub fn paint_batch_panel(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let panel_w = 340.0_f64.min(size.width);
    let x = size.width - panel_w;

    // Full-height panel
    let panel_rect = Rect::new(x, 0.0, size.width, size.height);
    painter.fill_rect(panel_rect, PANEL_BG);

    // Left border
    painter.draw_line(
        Point::new(x, 0.0),
        Point::new(x, size.height),
        BORDER_COLOR,
        1.0,
    );

    // Clip to panel bounds
    painter.push_clip(panel_rect);

    let pad = 16.0;
    let mut y = 16.0;

    // Header
    painter.draw_text_cached(
        text_cache,
        "Batch Processing",
        x + pad,
        y + 8.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Close button
    let close_x = size.width - pad - 40.0;
    let close_rect = Rect::new(close_x, y, close_x + 40.0, y + 28.0);
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(close_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Cancel",
        close_x + 2.0,
        y + 18.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ToggleBatch);

    y += 40.0;

    // Mode toggle
    let modes = [
        ("Resize", state.batch_mode_resize),
        ("Convert", !state.batch_mode_resize),
    ];
    let mode_w = 80.0;
    let mode_gap = 8.0;
    let mut mx = x + pad;

    for (label, active) in modes {
        let bg = if active { ACCENT_VIEW } else { BTN_BG };
        let tc = if active {
            Color::rgb8(0x0F, 0x0F, 0x0F)
        } else {
            TEXT_PRIMARY
        };
        let btn_rect = Rect::new(mx, y, mx + mode_w, y + 28.0);
        painter.fill_rounded_rect(btn_rect, bg, 4.0);
        painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);
        let tw = text_cache.measure_text_width(label, 11.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            label,
            mx + (mode_w - tw) / 2.0,
            y + 18.0,
            tc,
            11.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        mx += mode_w + mode_gap;
    }

    // Register click for each mode button
    let resize_btn_rect = Rect::new(x + pad, y, x + pad + mode_w, y + 28.0);
    state.register_click(resize_btn_rect, ClickAction::BatchModeResize);
    let convert_btn_rect = Rect::new(
        x + pad + mode_w + mode_gap,
        y,
        x + pad + mode_w * 2.0 + mode_gap,
        y + 28.0,
    );
    state.register_click(convert_btn_rect, ClickAction::BatchModeConvert);

    y += 40.0;

    // Fields based on mode
    if state.batch_mode_resize {
        painter.draw_text_cached(
            text_cache,
            "Width",
            x + pad,
            y + 8.0,
            TEXT_SECONDARY,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        let w_input = Rect::new(x + pad, y + 14.0, x + panel_w / 2.0 - 4.0, y + 42.0);
        painter.fill_rounded_rect(w_input, INPUT_BG, 4.0);
        painter.stroke_rounded_rect(w_input, BORDER_COLOR, 1.0, 4.0);
        painter.draw_text_cached(
            text_cache,
            &state.batch_width.to_string(),
            x + pad + 8.0,
            y + 34.0,
            TEXT_PRIMARY,
            13.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        let h_x = x + panel_w / 2.0 + 4.0;
        painter.draw_text_cached(
            text_cache,
            "Height",
            h_x,
            y + 8.0,
            TEXT_SECONDARY,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        let h_input = Rect::new(h_x, y + 14.0, x + panel_w - pad, y + 42.0);
        painter.fill_rounded_rect(h_input, INPUT_BG, 4.0);
        painter.stroke_rounded_rect(h_input, BORDER_COLOR, 1.0, 4.0);
        painter.draw_text_cached(
            text_cache,
            &state.batch_height.to_string(),
            h_x + 8.0,
            y + 34.0,
            TEXT_PRIMARY,
            13.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    } else {
        let formats = ["png", "jpg", "webp"];
        let mut fx = x + pad;
        for fmt in &formats {
            let is_active = *fmt == state.batch_format;
            let bg = if is_active { ACCENT_VIEW } else { BTN_BG };
            let tc = if is_active {
                Color::rgb8(0x0F, 0x0F, 0x0F)
            } else {
                TEXT_PRIMARY
            };
            let btn_rect = Rect::new(fx, y, fx + 48.0, y + 28.0);
            painter.fill_rounded_rect(btn_rect, bg, 4.0);
            painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);
            let label = fmt.to_uppercase();
            let tw = text_cache.measure_text_width(&label, 11.0, FontWeight::MEDIUM);
            painter.draw_text_cached(
                text_cache,
                &label,
                fx + (48.0 - tw) / 2.0,
                y + 18.0,
                tc,
                11.0,
                FontWeight::MEDIUM,
                false,
                false,
            );
            state.register_click(btn_rect, ClickAction::BatchSelectFormat(fmt.to_string()));
            fx += 56.0;
        }
    }

    y += 56.0;

    // File list header - register click for select all
    let header_rect = Rect::new(x + pad, y, x + panel_w - pad, y + 28.0);
    painter.fill_rounded_rect(header_rect, Color::rgba8(0x1A, 0x1A, 0x1A, 40), 4.0);
    painter.stroke_rounded_rect(header_rect, BORDER_COLOR, 1.0, 4.0);
    state.register_click(header_rect, ClickAction::BatchToggleSelectAll);

    let selected_count = state.batch_selected.len();
    let total = state.sorted_entries.len();
    let count_label = format!("{} / {}", selected_count, total);
    painter.draw_text_cached(
        text_cache,
        "Select All",
        x + pad + 8.0,
        y + 18.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    let tw = text_cache.measure_text_width(&count_label, 11.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        &count_label,
        x + panel_w - pad - tw - 8.0,
        y + 18.0,
        TEXT_MUTED,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );

    y += 36.0;

    // File list (show first 20)
    let visible_count = state.sorted_entries.len().min(20);
    for i in 0..visible_count {
        let entry = &state.sorted_entries[i];
        let is_selected = state.batch_selected.contains(&i);

        // Clickable row for the entire file entry
        let row_rect = Rect::new(x + pad, y, x + panel_w - pad, y + 22.0);

        let check = if is_selected { "\u{2611}" } else { "\u{2610}" };
        painter.draw_text_cached(
            text_cache,
            check,
            x + pad + 4.0,
            y + 8.0,
            ACCENT_VIEW,
            12.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Truncate filename if too long
        let max_chars = ((panel_w - pad * 2.0 - 28.0) / 7.0) as usize;
        let display_name = if entry.file_name.len() > max_chars {
            format!("{}...", &entry.file_name[..max_chars.saturating_sub(3)])
        } else {
            entry.file_name.clone()
        };
        painter.draw_text_cached(
            text_cache,
            &display_name,
            x + pad + 24.0,
            y + 8.0,
            TEXT_PRIMARY,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Register click region for toggling this file
        state.register_click(row_rect, ClickAction::BatchToggleFile(i));

        y += 22.0;
    }

    if state.sorted_entries.len() > 20 {
        painter.draw_text_cached(
            text_cache,
            &format!("... and {} more", state.sorted_entries.len() - 20),
            x + pad + 24.0,
            y + 8.0,
            TEXT_MUTED,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    }

    // Apply button position
    let apply_y = size.height - 52.0;

    // Progress bar and cancel button when batch is running
    if state.batch_running {
        let progress_y = apply_y - 48.0;
        let progress_w = panel_w - pad * 2.0 - 70.0;
        let progress_rect = Rect::new(x + pad, progress_y, x + pad + progress_w, progress_y + 20.0);

        // Progress bar background
        painter.fill_rounded_rect(progress_rect, Color::rgb8(0x2A, 0x2A, 0x2A), 4.0);

        // Progress bar fill
        if let Some((done, total)) = state.batch_progress {
            if total > 0 {
                let pct = done as f64 / total as f64;
                let fill_w = progress_w * pct;
                let fill_rect = Rect::new(x + pad, progress_y, x + pad + fill_w, progress_y + 20.0);
                painter.fill_rounded_rect(fill_rect, ACCENT_VIEW, 4.0);

                // Percentage text
                let pct_text = format!("{:.0}%", pct * 100.0);
                let tw = text_cache.measure_text_width(&pct_text, 10.0, FontWeight::BOLD);
                painter.draw_text_cached(
                    text_cache,
                    &pct_text,
                    x + pad + (progress_w - tw) / 2.0,
                    progress_y + 14.0,
                    TEXT_PRIMARY,
                    10.0,
                    FontWeight::BOLD,
                    false,
                    false,
                );
            }
        }

        // Cancel button
        let cancel_x = x + pad + progress_w + 8.0;
        let cancel_rect = Rect::new(cancel_x, progress_y, cancel_x + 58.0, progress_y + 20.0);
        painter.fill_rounded_rect(cancel_rect, Color::rgb8(0x8B, 0x00, 0x00), 4.0);
        painter.draw_text_cached(
            text_cache,
            "Cancel",
            cancel_x + 4.0,
            progress_y + 14.0,
            TEXT_PRIMARY,
            10.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(cancel_rect, ClickAction::BatchCancel);
    }

    // Output folder option
    let output_y = if state.batch_running {
        apply_y - 78.0
    } else {
        apply_y - 38.0
    };
    let output_label = if state.batch_output_folder.is_empty() {
        "Output: Same as source"
    } else {
        "Output: Custom folder"
    };
    painter.draw_text_cached(
        text_cache,
        output_label,
        x + pad,
        output_y + 12.0,
        TEXT_MUTED,
        10.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    let browse_x = x + pad + 160.0;
    let browse_rect = Rect::new(browse_x, output_y, browse_x + 50.0, output_y + 20.0);
    painter.fill_rounded_rect(browse_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(browse_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Browse",
        browse_x + 3.0,
        output_y + 14.0,
        TEXT_SECONDARY,
        10.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(browse_rect, ClickAction::BatchBrowseOutput);

    // Apply button at bottom
    let apply_w = panel_w - pad * 2.0;
    let apply_rect = Rect::new(x + pad, apply_y, x + pad + apply_w, apply_y + 36.0);
    let is_enabled = selected_count > 0 && !state.batch_running;
    let bg = if is_enabled { ACCENT_VIEW } else { BTN_BG };
    painter.fill_rounded_rect(apply_rect, bg, 6.0);
    painter.stroke_rounded_rect(apply_rect, BORDER_COLOR, 1.0, 6.0);

    let apply_label = if state.batch_running {
        if let Some((done, total)) = state.batch_progress {
            format!("Processing {}/{}", done, total)
        } else {
            "Processing...".to_string()
        }
    } else {
        format!("Apply ({})", selected_count)
    };
    let tc = if is_enabled {
        Color::rgb8(0x0F, 0x0F, 0x0F)
    } else {
        TEXT_MUTED
    };
    let tw = text_cache.measure_text_width(&apply_label, 12.0, FontWeight::BOLD);
    painter.draw_text_cached(
        text_cache,
        &apply_label,
        x + pad + (apply_w - tw) / 2.0,
        apply_y + 22.0,
        tc,
        12.0,
        FontWeight::BOLD,
        false,
        false,
    );
    if is_enabled {
        state.register_click(apply_rect, ClickAction::BatchApply);
    }

    painter.pop_clip();
}
