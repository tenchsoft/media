use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::PlayerState;
use crate::ui::theme::TOAST_BG;

/// Paint the toast notification.
pub fn paint_toast(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    controls_h: f64,
    toast_time: &mut Option<std::time::Instant>,
) {
    let Some(toast) = &state.toast else {
        return;
    };

    let expired = toast_time.is_some_and(|t| t.elapsed().as_millis() > 3000);
    if expired {
        state.toast = None;
        *toast_time = None;
    } else {
        if toast_time.is_none() {
            *toast_time = Some(std::time::Instant::now());
        }
        let rect = Rect::new(
            size.width / 2.0 - 140.0,
            size.height - controls_h - 52.0,
            size.width / 2.0 + 140.0,
            size.height - controls_h - 18.0,
        );
        p.fill_rounded_rect(rect, TOAST_BG, theme.border_radius);
        p.draw_text(
            toast,
            size.width / 2.0,
            rect.y0 + 22.0,
            Color::WHITE,
            theme.font_size_small,
            FontWeight::BOLD,
            true,
        );
    }
}
