use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BG_DARK, GRID_COLOR};

pub(super) fn paint_help_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    modal_x: f64,
    modal_w: f64,
) {
    if !state.help_open {
        return;
    }

    let help_h = 440.0;
    let help_y = (size.height - help_h) / 2.0;
    let help_rect = Rect::new(modal_x, help_y, modal_x + modal_w, help_y + help_h);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );
    p.fill_rounded_rect(help_rect, BG_DARK, 8.0);
    p.stroke_rounded_rect(help_rect, GRID_COLOR, 1.0, 8.0);
    p.draw_text(
        "Keyboard Shortcuts",
        modal_x + 16.0,
        help_y + 24.0,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    let shortcuts = [
        ("Space", "Play / Pause"),
        ("Left/Right", "Seek -5s / +5s"),
        ("Up/Down", "Volume Up / Down"),
        ("J / L", "Seek -10s / +10s"),
        ("Z / X", "Subtitle offset -100ms / +100ms"),
        ("C", "Cycle subtitle tracks"),
        ("F", "Toggle fullscreen"),
        ("M", "Toggle mute"),
        ("[ / ]", "Playback speed down / up"),
        ("N", "Next in playlist"),
        ("P", "Previous in playlist"),
        ("R", "Cycle repeat mode"),
        ("S", "Toggle shuffle"),
        ("G", "Toggle GIF recording"),
        ("T", "Take screenshot"),
        ("Ctrl+O", "Open file"),
        ("Ctrl+L", "Open URL"),
        ("0-9", "Jump to 0%-90% position"),
        ("?", "Show this help"),
        ("Esc", "Close modal / Exit fullscreen"),
    ];
    let mut sy = help_y + 48.0;
    for (key, desc) in &shortcuts {
        p.draw_text(
            key,
            modal_x + 16.0,
            sy,
            theme.primary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            desc,
            modal_x + 100.0,
            sy,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
        sy += 20.0;
    }
    let close_rect = Rect::new(
        modal_x + modal_w - 40.0,
        help_y + 4.0,
        modal_x + modal_w - 8.0,
        help_y + 32.0,
    );
    p.draw_text(
        "X",
        close_rect.x0 + 16.0,
        help_y + 22.0,
        theme.secondary,
        12.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(close_rect, ClickAction::CloseModal);
}
