use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::controls;
use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BG_DARK, BTN_ACTION, BTN_DIM, GRID_COLOR};

pub(super) fn paint_subtitle_search_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    modal_x: f64,
    modal_w: f64,
) {
    if !state.subtitle_search_open {
        return;
    }

    let search_h = 200.0;
    let search_y = (size.height - search_h) / 2.0;
    let search_rect = Rect::new(modal_x, search_y, modal_x + modal_w, search_y + search_h);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );
    p.fill_rounded_rect(search_rect, BG_DARK, 8.0);
    p.stroke_rounded_rect(search_rect, GRID_COLOR, 1.0, 8.0);
    p.draw_text(
        "Search Subtitles",
        modal_x + 16.0,
        search_y + 24.0,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    let input_r = Rect::new(
        modal_x + 16.0,
        search_y + 40.0,
        modal_x + modal_w - 16.0,
        search_y + 70.0,
    );
    p.fill_rounded_rect(input_r, theme.background, theme.border_radius);
    p.stroke_rounded_rect(input_r, theme.border, 1.0, theme.border_radius);
    let search_display = if state.subtitle_search_text.is_empty() {
        "Search text..."
    } else {
        &state.subtitle_search_text
    };
    p.draw_text(
        search_display,
        modal_x + 24.0,
        search_y + 60.0,
        if state.subtitle_search_text.is_empty() {
            theme.disabled
        } else {
            theme.on_surface
        },
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    state.register_click(input_r, ClickAction::FocusSubtitleSearch);
    if let Some(result_time) = state.subtitle_search_result_time {
        p.draw_text(
            &format!("Found at {}", controls::format_single_time(result_time)),
            modal_x + 16.0,
            search_y + 90.0,
            theme.primary,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
    } else if !state.subtitle_search_text.is_empty() {
        p.draw_text(
            "No match found",
            modal_x + 16.0,
            search_y + 90.0,
            theme.secondary,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
    }
    // Find Next / Prev buttons
    let next_btn = Rect::new(
        modal_x + modal_w - 100.0,
        search_y + 120.0,
        modal_x + modal_w - 16.0,
        search_y + 148.0,
    );
    p.fill_rounded_rect(next_btn, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Find Next",
        next_btn.x0 + 42.0,
        search_y + 138.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(next_btn, ClickAction::SearchSubtitleNext);
    let prev_btn = Rect::new(
        modal_x + 16.0,
        search_y + 120.0,
        modal_x + 90.0,
        search_y + 148.0,
    );
    p.fill_rounded_rect(prev_btn, BTN_DIM, theme.border_radius);
    p.draw_text(
        "Prev",
        prev_btn.x0 + 37.0,
        search_y + 138.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(prev_btn, ClickAction::SearchSubtitlePrev);
    let close_btn = Rect::new(
        modal_x + modal_w / 2.0 - 30.0,
        search_y + 160.0,
        modal_x + modal_w / 2.0 + 30.0,
        search_y + 188.0,
    );
    p.fill_rounded_rect(close_btn, BTN_DIM, theme.border_radius);
    p.draw_text(
        "Close",
        close_btn.x0 + 30.0,
        search_y + 178.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(close_btn, ClickAction::CloseModal);
}
