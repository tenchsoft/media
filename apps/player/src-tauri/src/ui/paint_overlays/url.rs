use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BG_DARK, BTN_ACTION, BTN_DIM, GRID_COLOR};

#[allow(clippy::too_many_arguments)]
pub(super) fn paint_url_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    modal_x: f64,
    modal_y: f64,
    modal_w: f64,
) {
    if !state.url_input_open {
        return;
    }

    let url_rect = Rect::new(modal_x, modal_y, modal_x + modal_w, modal_y + 120.0);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );
    p.fill_rounded_rect(url_rect, BG_DARK, 8.0);
    p.stroke_rounded_rect(url_rect, GRID_COLOR, 1.0, 8.0);
    p.draw_text(
        "Open URL",
        modal_x + 16.0,
        modal_y + 24.0,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    let input_r = Rect::new(
        modal_x + 16.0,
        modal_y + 40.0,
        modal_x + modal_w - 16.0,
        modal_y + 70.0,
    );
    p.fill_rounded_rect(input_r, theme.background, theme.border_radius);
    p.stroke_rounded_rect(input_r, theme.border, 1.0, theme.border_radius);
    let url_display = if state.url_input_text.is_empty() {
        "https://..."
    } else {
        &state.url_input_text
    };
    p.draw_text(
        url_display,
        modal_x + 24.0,
        modal_y + 60.0,
        if state.url_input_text.is_empty() {
            theme.disabled
        } else {
            theme.on_surface
        },
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    state.register_click(input_r, ClickAction::FocusUrlInput);
    // Play button
    let play_btn = Rect::new(
        modal_x + modal_w - 100.0,
        modal_y + 80.0,
        modal_x + modal_w - 16.0,
        modal_y + 108.0,
    );
    p.fill_rounded_rect(play_btn, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Play",
        play_btn.x0 + 42.0,
        modal_y + 98.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(play_btn, ClickAction::SubmitUrl);
    // Cancel button
    let cancel_btn = Rect::new(
        modal_x + 16.0,
        modal_y + 80.0,
        modal_x + 90.0,
        modal_y + 108.0,
    );
    p.fill_rounded_rect(cancel_btn, BTN_DIM, theme.border_radius);
    p.draw_text(
        "Cancel",
        cancel_btn.x0 + 37.0,
        modal_y + 98.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(cancel_btn, ClickAction::CloseModal);
}
