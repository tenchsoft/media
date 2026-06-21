use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BG_DARK, BTN_ACTION, BTN_DIM, GRID_COLOR};

pub(super) fn paint_gif_options_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    modal_x: f64,
    modal_w: f64,
) {
    if !state.gif_options_open {
        return;
    }

    let gif_h = 260.0;
    let gif_y = (size.height - gif_h) / 2.0;
    let gif_rect = Rect::new(modal_x, gif_y, modal_x + modal_w, gif_y + gif_h);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );
    p.fill_rounded_rect(gif_rect, BG_DARK, 8.0);
    p.stroke_rounded_rect(gif_rect, GRID_COLOR, 1.0, 8.0);
    p.draw_text(
        "GIF Capture Options",
        modal_x + 16.0,
        gif_y + 24.0,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    let opts = &state.gif_options;
    let mut sy = gif_y + 52.0;
    let gif_labels = [
        format!("FPS: {}", opts.fps),
        format!("Quality: {}%", opts.quality),
        format!("Max Duration: {}s", opts.max_duration_secs),
    ];
    for label in &gif_labels {
        p.draw_text(
            label,
            modal_x + 16.0,
            sy,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
        sy += 28.0;
    }
    let start_btn = Rect::new(
        modal_x + modal_w / 2.0 - 60.0,
        sy + 16.0,
        modal_x + modal_w / 2.0 + 60.0,
        sy + 44.0,
    );
    p.fill_rounded_rect(start_btn, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Start Recording",
        start_btn.x0 + 60.0,
        sy + 34.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(start_btn, ClickAction::StartGifRecording);
    let cancel_btn = Rect::new(modal_x + 16.0, sy + 16.0, modal_x + 90.0, sy + 44.0);
    p.fill_rounded_rect(cancel_btn, BTN_DIM, theme.border_radius);
    p.draw_text(
        "Cancel",
        cancel_btn.x0 + 37.0,
        sy + 34.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(cancel_btn, ClickAction::CloseModal);
}
