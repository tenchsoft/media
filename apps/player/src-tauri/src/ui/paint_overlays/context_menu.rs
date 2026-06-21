use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::controls;
use crate::ui::state::PlayerState;
use crate::ui::theme::{BG_DARK, BTN_ACTION, GRID_COLOR};

/// Paint the context menu overlay.
pub fn paint_context_menu(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
) {
    let Some(ref menu) = state.context_menu else {
        return;
    };

    let menu_w = 220.0;
    let item_h = 28.0;
    let info_section_h = 60.0;
    let menu_h = info_section_h + menu.items.len() as f64 * item_h + 8.0;
    let mx = menu.x.min(size.width - menu_w);
    let my = menu.y.min(size.height - menu_h);
    let menu_rect = Rect::new(mx, my, mx + menu_w, my + menu_h);
    // Shadow
    p.fill_rounded_rect(
        Rect::new(mx + 2.0, my + 2.0, mx + menu_w + 2.0, my + menu_h + 2.0),
        Color::rgba8(0, 0, 0, 80),
        theme.border_radius,
    );
    // Background
    p.fill_rounded_rect(menu_rect, BG_DARK, theme.border_radius);
    // Border
    p.stroke_rounded_rect(menu_rect, GRID_COLOR, 1.0, theme.border_radius);

    // Contextual info section
    let info_y = my + 4.0;
    p.draw_text(
        &format!(
            "Time: {}",
            controls::format_time(state.current_time, state.duration)
        ),
        mx + 12.0,
        info_y + 14.0,
        theme.secondary,
        9.0,
        FontWeight::NORMAL,
        false,
    );
    let fps_str = format!("{:.0} FPS", state.media_info.frame_rate);
    p.draw_text(
        &fps_str,
        mx + 12.0,
        info_y + 28.0,
        theme.secondary,
        9.0,
        FontWeight::NORMAL,
        false,
    );
    if !state.media_info.video_codec.is_empty() {
        p.draw_text(
            &format!("Codec: {}", state.media_info.video_codec),
            mx + 12.0,
            info_y + 42.0,
            theme.secondary,
            9.0,
            FontWeight::NORMAL,
            false,
        );
    }
    // Divider line
    p.draw_line(
        Point::new(mx + 8.0, info_y + info_section_h - 8.0),
        Point::new(mx + menu_w - 8.0, info_y + info_section_h - 8.0),
        GRID_COLOR,
        1.0,
    );

    // Items with hover highlight
    for (idx, item) in menu.items.iter().enumerate() {
        let iy = my + info_section_h + idx as f64 * item_h;
        let item_rect = Rect::new(mx + 4.0, iy, mx + menu_w - 4.0, iy + item_h);
        if state.context_menu_hover == Some(idx) {
            p.fill_rounded_rect(item_rect, BTN_ACTION, 4.0);
        }
        p.draw_text(
            &item.label,
            mx + 16.0,
            iy + 18.0,
            if state.context_menu_hover == Some(idx) {
                Color::WHITE
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
    }
}
