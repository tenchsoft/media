use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BTN_DEFAULT, BTN_DIM};

/// Paint the GIF capture modal.
pub fn paint_gif_capture_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    gif_recording: bool,
) {
    if !state.gif_capture_open {
        return;
    }

    let modal = Rect::new(
        size.width / 2.0 - 150.0,
        size.height / 2.0 - 72.0,
        size.width / 2.0 + 150.0,
        size.height / 2.0 + 72.0,
    );
    p.fill_rounded_rect(modal, theme.surface, theme.border_radius);
    p.stroke_rounded_rect(modal, theme.border, 1.0, theme.border_radius);
    p.draw_text(
        "GIF Capture",
        modal.x0 + 16.0,
        modal.y0 + 26.0,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &format!("State: {}", state.gif_state),
        modal.x0 + 16.0,
        modal.y0 + 50.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    // Start/Stop button
    let btn_label = if gif_recording { "Stop" } else { "Start" };
    let btn_rect = Rect::new(
        modal.x0 + 16.0,
        modal.y1 - 44.0,
        modal.x0 + 80.0,
        modal.y1 - 16.0,
    );
    p.fill_rounded_rect(btn_rect, theme.primary, theme.border_radius);
    p.draw_text(
        btn_label,
        btn_rect.x0 + 32.0,
        btn_rect.y0 + 16.0,
        theme.on_primary,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    let action = if gif_recording {
        ClickAction::StopGifRecord
    } else {
        ClickAction::StartGifRecord
    };
    state.register_click(btn_rect, action);
    // Options button
    let options_rect = Rect::new(
        modal.x0 + 104.0,
        modal.y1 - 44.0,
        modal.x0 + 190.0,
        modal.y1 - 16.0,
    );
    p.fill_rounded_rect(options_rect, BTN_DIM, theme.border_radius);
    p.draw_text(
        "Options",
        options_rect.x0 + 43.0,
        options_rect.y0 + 16.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(options_rect, ClickAction::GifOptions);
    // Close button
    let close_rect = Rect::new(
        modal.x1 - 80.0,
        modal.y1 - 44.0,
        modal.x1 - 16.0,
        modal.y1 - 16.0,
    );
    p.fill_rounded_rect(close_rect, BTN_DEFAULT, theme.border_radius);
    p.draw_text(
        "Close",
        close_rect.x0 + 32.0,
        close_rect.y0 + 16.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(close_rect, ClickAction::ToggleGifCapture);
}
