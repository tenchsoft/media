use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::super::state::{ClickAction, ViewState};
use super::super::theme::{
    ACCENT_VIEW, BORDER_COLOR, BTN_BG, INPUT_BG, PANEL_BG, TEXT_PRIMARY, TEXT_SECONDARY,
};

// Slideshow Controls

/// Computes slideshow control button rects without rendering.
/// Used by `automation_children` fallback when paint has not yet occurred.
pub fn slideshow_controls_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.slideshow_playing {
        return Vec::new();
    }
    let mut rects = Vec::new();

    let ctrl_w = 540.0;
    let ctrl_h = 40.0;
    let ctrl_x = (size.width - ctrl_w) / 2.0;
    let ctrl_y = size.height - 100.0;
    let btn_h = ctrl_h - 12.0;
    let btn_y = ctrl_y + 6.0;

    // Approximate button widths (tw + 16.0 from draw_btn! macro)
    let play_w = 46.0;
    let interval_w = 32.0;
    let shuffle_w = 54.0;
    let trans_w = 50.0;
    let loop_w = 46.0;
    let gap = 6.0;

    let mut cur_x = ctrl_x + 12.0;

    // Play/Pause
    rects.push((
        ClickAction::ToggleSlideshow,
        Rect::new(cur_x, btn_y, cur_x + play_w, btn_y + btn_h),
    ));
    cur_x += play_w + gap;

    // Interval
    rects.push((
        ClickAction::SlideshowCycleInterval,
        Rect::new(cur_x, btn_y, cur_x + interval_w, btn_y + btn_h),
    ));
    cur_x += interval_w + gap;

    // Shuffle
    rects.push((
        ClickAction::SlideshowToggleShuffle,
        Rect::new(cur_x, btn_y, cur_x + shuffle_w, btn_y + btn_h),
    ));
    cur_x += shuffle_w + gap;

    // Transition
    rects.push((
        ClickAction::SlideshowCycleTransition,
        Rect::new(cur_x, btn_y, cur_x + trans_w, btn_y + btn_h),
    ));
    cur_x += trans_w + gap;

    // Loop
    rects.push((
        ClickAction::SlideshowToggleLoop,
        Rect::new(cur_x, btn_y, cur_x + loop_w, btn_y + btn_h),
    ));

    // Close (right-aligned)
    let close_w = 60.0;
    let close_x = ctrl_x + ctrl_w - close_w - 12.0;
    rects.push((
        ClickAction::DismissAll,
        Rect::new(close_x, btn_y, close_x + close_w, btn_y + btn_h),
    ));

    rects
}

/// Draws the slideshow controls bar.
/// Matches `.slideshow-controls` CSS: centered, bottom area, with play/pause and interval.
pub fn paint_slideshow_controls(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let ctrl_w = 540.0;
    let ctrl_h = 40.0;
    let ctrl_x = (size.width - ctrl_w) / 2.0;
    let ctrl_y = size.height - 100.0;

    let ctrl_rect = Rect::new(ctrl_x, ctrl_y, ctrl_x + ctrl_w, ctrl_y + ctrl_h);

    // Background
    painter.fill_rounded_rect(ctrl_rect, PANEL_BG, 8.0);
    painter.stroke_rounded_rect(ctrl_rect, BORDER_COLOR, 1.0, 8.0);

    let btn_h = ctrl_h - 12.0;
    let btn_y = ctrl_y + 6.0;
    let text_y = ctrl_y + ctrl_h / 2.0 + 4.0;

    // Pre-read state values to avoid borrow conflicts
    let play_label = if state.slideshow_playing {
        "Pause"
    } else {
        "Play"
    };
    let interval_label = format!("{:.0}s", state.slideshow_interval_ms as f64 / 1000.0);
    let shuffle_label = if state.slideshow_shuffle {
        "Shuf On"
    } else {
        "Shuffle"
    };
    let shuf_bg = if state.slideshow_shuffle {
        ACCENT_VIEW
    } else {
        BTN_BG
    };
    let trans_label = state.slideshow_transition.label();
    let loop_label = if state.slideshow_loop {
        "Loop On"
    } else {
        "Loop"
    };
    let loop_bg = if state.slideshow_loop {
        ACCENT_VIEW
    } else {
        BTN_BG
    };

    let mut cur_x = ctrl_x + 12.0;

    macro_rules! draw_btn {
        ($label:expr, $action:expr, $bg:expr, $weight:expr) => {{
            let tw = text_cache.measure_text_width($label, 11.0, $weight);
            let w = tw + 16.0;
            let rect = Rect::new(cur_x, btn_y, cur_x + w, btn_y + btn_h);
            painter.fill_rounded_rect(rect, $bg, 4.0);
            painter.stroke_rounded_rect(rect, BORDER_COLOR, 1.0, 4.0);
            painter.draw_text_cached(
                text_cache,
                $label,
                cur_x + (w - tw) / 2.0,
                text_y,
                TEXT_PRIMARY,
                11.0,
                $weight,
                false,
                false,
            );
            state.register_click(rect, $action);
            cur_x += w + 6.0;
        }};
    }

    // Play/Pause button
    draw_btn!(
        play_label,
        ClickAction::ToggleSlideshow,
        BTN_BG,
        FontWeight::MEDIUM
    );

    // Interval selector
    draw_btn!(
        &interval_label,
        ClickAction::SlideshowCycleInterval,
        INPUT_BG,
        FontWeight::NORMAL
    );

    // Shuffle button
    draw_btn!(
        shuffle_label,
        ClickAction::SlideshowToggleShuffle,
        shuf_bg,
        FontWeight::MEDIUM
    );

    // Transition effect selector
    draw_btn!(
        trans_label,
        ClickAction::SlideshowCycleTransition,
        INPUT_BG,
        FontWeight::NORMAL
    );

    // Loop toggle
    draw_btn!(
        loop_label,
        ClickAction::SlideshowToggleLoop,
        loop_bg,
        FontWeight::MEDIUM
    );

    // cur_x is advanced by draw_btn! for layout; suppress unused after last button
    let _ = cur_x;

    // Close button (right-aligned)
    let close_w = 60.0;
    let close_x = ctrl_x + ctrl_w - close_w - 12.0;
    let close_rect = Rect::new(close_x, btn_y, close_x + close_w, btn_y + btn_h);
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(close_rect, BORDER_COLOR, 1.0, 4.0);
    let tw = text_cache.measure_text_width("Close", 11.0, FontWeight::NORMAL);
    painter.draw_text_cached(
        text_cache,
        "Close",
        close_x + (close_w - tw) / 2.0,
        text_y,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::DismissAll);
}
