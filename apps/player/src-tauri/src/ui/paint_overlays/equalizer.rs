use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BG_DARK, BTN_ACTION, BTN_DIM, GRID_COLOR};

pub(super) fn paint_equalizer_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    modal_x: f64,
    modal_w: f64,
) {
    if !state.eq_open {
        return;
    }

    let eq_h = 280.0;
    let eq_y = (size.height - eq_h) / 2.0;
    let eq_rect = Rect::new(modal_x, eq_y, modal_x + modal_w, eq_y + eq_h);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );
    p.fill_rounded_rect(eq_rect, BG_DARK, 8.0);
    p.stroke_rounded_rect(eq_rect, GRID_COLOR, 1.0, 8.0);
    p.draw_text(
        "Audio Equalizer",
        modal_x + 16.0,
        eq_y + 24.0,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    let eq = &state.eq_bands;
    let mut sy = eq_y + 52.0;
    let eq_labels = [
        format!("60Hz: {:.1} dB", eq[0]),
        format!("250Hz: {:.1} dB", eq[1]),
        format!("1kHz: {:.1} dB", eq[2]),
        format!("4kHz: {:.1} dB", eq[3]),
        format!("16kHz: {:.1} dB", eq[4]),
    ];
    for (band_idx, label) in eq_labels.iter().enumerate() {
        p.draw_text(
            label,
            modal_x + 16.0,
            sy,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
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
        state.register_click(
            minus_r,
            ClickAction::SetEqBand(
                band_idx,
                (state.eq_bands[band_idx] - 1.0).clamp(-12.0, 12.0),
            ),
        );
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
        state.register_click(
            plus_r,
            ClickAction::SetEqBand(
                band_idx,
                (state.eq_bands[band_idx] + 1.0).clamp(-12.0, 12.0),
            ),
        );
        sy += 32.0;
    }
    // Presets
    sy += 8.0;
    p.draw_text(
        "Presets:",
        modal_x + 16.0,
        sy,
        theme.secondary,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
    );
    sy += 20.0;
    let presets = ["Flat", "Bass Boost", "Treble Boost", "Voice", "Loudness"];
    for (pi, preset) in presets.iter().enumerate() {
        let px = modal_x + 16.0 + (pi as f64) * 68.0;
        let preset_r = Rect::new(px, sy - 10.0, px + 62.0, sy + 8.0);
        p.fill_rounded_rect(preset_r, BTN_DIM, 4.0);
        p.draw_text(
            preset,
            px + 31.0,
            sy - 1.0,
            theme.on_surface,
            9.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(preset_r, ClickAction::SetEqPresetNamed(preset.to_string()));
    }
    let close_btn = Rect::new(
        modal_x + modal_w / 2.0 - 40.0,
        eq_y + eq_h - 40.0,
        modal_x + modal_w / 2.0 + 40.0,
        eq_y + eq_h - 12.0,
    );
    p.fill_rounded_rect(close_btn, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Close",
        close_btn.x0 + 40.0,
        eq_y + eq_h - 22.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(close_btn, ClickAction::CloseModal);
}
