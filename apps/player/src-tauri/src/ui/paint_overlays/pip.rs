use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, PlayerState};

/// Paint the PiP indicator.
pub fn paint_pip_indicator(p: &mut Painter<'_>, state: &mut PlayerState, size: Size) {
    if !state.pip_mode {
        return;
    }
    let pip_rect = Rect::new(size.width - 160.0, 8.0, size.width - 8.0, 28.0);
    p.fill_rounded_rect(pip_rect, Color::rgba8(0, 0, 0, 180), 4.0);
    p.draw_text(
        "Picture-in-Picture",
        pip_rect.x0 + 76.0,
        pip_rect.y0 + 14.0,
        Color::WHITE,
        9.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(pip_rect, ClickAction::TogglePip);
}
