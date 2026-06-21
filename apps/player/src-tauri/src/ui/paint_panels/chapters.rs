use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::controls;
use crate::ui::state::*;
use crate::ui::theme::{BTN_ACTION, CHAPTER_AI_MARKER};

pub(super) fn paint_chapters_tab(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    size: Size,
    spacing: f64,
    mut dy: f64,
) {
    // Export/Import buttons
    let export_rect = Rect::new(
        video_rect.x1 + spacing,
        dy,
        video_rect.x1 + spacing + 60.0,
        dy + 24.0,
    );
    p.fill_rounded_rect(export_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Export",
        export_rect.x0 + 30.0,
        dy + 15.0,
        Color::WHITE,
        9.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(export_rect, ClickAction::ExportChapters);

    let import_rect = Rect::new(
        video_rect.x1 + spacing + 66.0,
        dy,
        video_rect.x1 + spacing + 126.0,
        dy + 24.0,
    );
    p.fill_rounded_rect(import_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Import",
        import_rect.x0 + 30.0,
        dy + 15.0,
        Color::WHITE,
        9.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(import_rect, ClickAction::ImportChapters);

    let add_rect = Rect::new(
        video_rect.x1 + spacing + 132.0,
        dy,
        video_rect.x1 + spacing + 192.0,
        dy + 24.0,
    );
    p.fill_rounded_rect(add_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Add",
        add_rect.x0 + 30.0,
        dy + 15.0,
        Color::WHITE,
        9.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(add_rect, ClickAction::ShowAddChapterModal);
    dy += 32.0;

    let chapter_data: Vec<(usize, String, f64, bool)> = state
        .chapters
        .iter()
        .enumerate()
        .map(|(ch_idx, ch)| (ch_idx, ch.title.clone(), ch.time, ch.ai_generated))
        .collect();
    for (ch_idx, title, time, ai_gen) in &chapter_data {
        let ch_row = Rect::new(
            video_rect.x1 + spacing,
            dy - 8.0,
            size.width - spacing - 50.0,
            dy + 16.0,
        );
        p.draw_text(
            &format!("{} {}", controls::format_single_time(*time), title),
            video_rect.x1 + spacing,
            dy,
            if *ai_gen {
                CHAPTER_AI_MARKER
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        state.register_click(ch_row, ClickAction::JumpToChapter(*ch_idx));

        // Delete button (X)
        let del_rect = Rect::new(
            size.width - spacing - 48.0,
            dy - 6.0,
            size.width - spacing - 32.0,
            dy + 10.0,
        );
        p.draw_text(
            "X",
            del_rect.x0 + 8.0,
            dy + 3.0,
            theme.secondary,
            9.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(del_rect, ClickAction::DeleteChapter(*ch_idx));

        // Rename button
        let rename_rect = Rect::new(
            size.width - spacing - 30.0,
            dy - 6.0,
            size.width - spacing,
            dy + 10.0,
        );
        p.draw_text(
            "Edit",
            rename_rect.x0 + 15.0,
            dy + 3.0,
            theme.secondary,
            9.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(rename_rect, ClickAction::RenameChapter(*ch_idx));

        dy += 22.0;
    }
}
