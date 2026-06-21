use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::controls;
use crate::ui::state::{ClickAction, PlayerState};
use crate::ui::theme::{BG_DARK, BTN_ACTION, BTN_DIM, GRID_COLOR};

pub(super) fn paint_add_chapter_modal(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    modal_x: f64,
    modal_w: f64,
) {
    if !state.show_add_chapter_modal {
        return;
    }

    let ch_h = 140.0;
    let ch_y = (size.height - ch_h) / 2.0;
    let ch_rect = Rect::new(modal_x, ch_y, modal_x + modal_w, ch_y + ch_h);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 120),
    );
    p.fill_rounded_rect(ch_rect, BG_DARK, 8.0);
    p.stroke_rounded_rect(ch_rect, GRID_COLOR, 1.0, 8.0);
    p.draw_text(
        &format!(
            "Add Chapter at {}",
            controls::format_single_time(state.current_time)
        ),
        modal_x + 16.0,
        ch_y + 24.0,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    let input_r = Rect::new(
        modal_x + 16.0,
        ch_y + 40.0,
        modal_x + modal_w - 16.0,
        ch_y + 70.0,
    );
    p.fill_rounded_rect(input_r, theme.background, theme.border_radius);
    p.stroke_rounded_rect(input_r, theme.border, 1.0, theme.border_radius);
    let name_display = if state.chapter_name_input.is_empty() {
        "Chapter name..."
    } else {
        &state.chapter_name_input
    };
    p.draw_text(
        name_display,
        modal_x + 24.0,
        ch_y + 60.0,
        if state.chapter_name_input.is_empty() {
            theme.disabled
        } else {
            theme.on_surface
        },
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    state.register_click(input_r, ClickAction::FocusChapterNameInput);
    let add_btn = Rect::new(
        modal_x + modal_w - 100.0,
        ch_y + 80.0,
        modal_x + modal_w - 16.0,
        ch_y + 108.0,
    );
    p.fill_rounded_rect(add_btn, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Add",
        add_btn.x0 + 42.0,
        ch_y + 98.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(add_btn, ClickAction::ConfirmAddChapter);
    let cancel_btn = Rect::new(modal_x + 16.0, ch_y + 80.0, modal_x + 90.0, ch_y + 108.0);
    p.fill_rounded_rect(cancel_btn, BTN_DIM, theme.border_radius);
    p.draw_text(
        "Cancel",
        cancel_btn.x0 + 37.0,
        ch_y + 98.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(cancel_btn, ClickAction::CloseModal);
}
