use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BG_DARK, BTN_ACTION, BTN_DIM, GRID_COLOR};

pub(super) fn paint_subtitle_style_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    modal_x: f64,
    modal_w: f64,
) {
    if !state.subtitle_style_open {
        return;
    }

    let style_h = 360.0;
    let style_y = (size.height - style_h) / 2.0;
    let style_rect = Rect::new(modal_x, style_y, modal_x + modal_w, style_y + style_h);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );
    p.fill_rounded_rect(style_rect, BG_DARK, 8.0);
    p.stroke_rounded_rect(style_rect, GRID_COLOR, 1.0, 8.0);
    p.draw_text(
        "Subtitle Style",
        modal_x + 16.0,
        style_y + 24.0,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    let style = &state.subtitle_style;
    let mut sy = style_y + 52.0;
    let labels = [
        format!("Font Size: {}", style.font_size),
        format!("Font Family: {}", style.font_family),
        "Text Color: White".to_string(),
        format!("BG Opacity: {:.0}%", style.bg_opacity * 100.0),
        format!("Position: {:.0}%", style.position * 100.0),
        format!("Stroke Width: {:.1}", style.stroke_width),
        format!("Shadow Offset: {:.1}", style.shadow_offset),
    ];
    let style_deltas: [(f32, f32); 7] = [
        (-2.0, 2.0),
        (0.0, 0.0),
        (0.0, 0.0),
        (-10.0, 10.0),
        (-5.0, 5.0),
        (-1.0, 1.0),
        (-1.0, 1.0),
    ];
    for (i, label) in labels.iter().enumerate() {
        p.draw_text(
            label,
            modal_x + 16.0,
            sy,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
        // - button
        let minus_r = Rect::new(
            modal_x + modal_w - 70.0,
            sy - 10.0,
            modal_x + modal_w - 50.0,
            sy + 6.0,
        );
        p.fill_rounded_rect(minus_r, BTN_DIM, 4.0);
        p.draw_text(
            "-",
            minus_r.x0 + 10.0,
            sy - 2.0,
            theme.on_surface,
            10.0,
            FontWeight::BOLD,
            true,
        );
        let (minus_delta, plus_delta) = style_deltas[i];
        state.register_click(minus_r, ClickAction::AdjustSubtitleStyle(i, minus_delta));
        // + button
        let plus_r = Rect::new(
            modal_x + modal_w - 44.0,
            sy - 10.0,
            modal_x + modal_w - 24.0,
            sy + 6.0,
        );
        p.fill_rounded_rect(plus_r, BTN_DIM, 4.0);
        p.draw_text(
            "+",
            plus_r.x0 + 10.0,
            sy - 2.0,
            theme.on_surface,
            10.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(plus_r, ClickAction::AdjustSubtitleStyle(i, plus_delta));
        sy += 32.0;
    }
    let close_rect = Rect::new(
        modal_x + modal_w / 2.0 - 40.0,
        sy + 8.0,
        modal_x + modal_w / 2.0 + 40.0,
        sy + 36.0,
    );
    p.fill_rounded_rect(close_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Close",
        close_rect.x0 + 40.0,
        sy + 26.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(close_rect, ClickAction::CloseModal);
}
