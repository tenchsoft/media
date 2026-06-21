use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::controls;
use crate::ui::state::*;
use crate::ui::theme::BTN_ACTION;

pub(super) fn paint_playlist_tab(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    size: Size,
    spacing: f64,
    mut dy: f64,
) {
    let playlist_count = state.playlist.len();
    let row_height = 48.0;
    let drawer_top = video_rect.y0 + 40.0;
    let drawer_bottom = size.height - controls::CONTROLS_HEIGHT - 8.0;
    let visible_h = (drawer_bottom - drawer_top).max(0.0);

    let scroll_y = state.drawer_scroll_y;
    let first_visible = ((scroll_y) / row_height) as usize;
    let last_visible = (((scroll_y + visible_h) / row_height) as usize + 1).min(playlist_count);

    for idx in first_visible..last_visible {
        let row_y = dy + idx as f64 * row_height;
        let row = Rect::new(
            video_rect.x1 + spacing,
            row_y - 8.0,
            size.width - spacing - 30.0,
            row_y + 34.0,
        );
        let is_current = state.current_playlist_index == Some(idx);
        p.fill_rounded_rect(
            row,
            if is_current {
                theme.primary
            } else {
                theme.background
            },
            theme.border_radius,
        );
        let title = state
            .playlist
            .get(idx)
            .map(|e| e.title.clone())
            .unwrap_or_default();
        let duration = state.playlist.get(idx).map(|e| e.duration).unwrap_or(0.0);
        p.draw_text(
            &title,
            row.x0 + 8.0,
            row_y + 8.0,
            if is_current {
                theme.on_primary
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        p.draw_text(
            &controls::format_single_time(duration),
            row.x0 + 8.0,
            row_y + 24.0,
            if is_current {
                theme.on_primary
            } else {
                theme.secondary
            },
            10.0,
            FontWeight::NORMAL,
            false,
        );
        // Remove button (X)
        let remove_rect = Rect::new(
            size.width - spacing - 24.0,
            row_y + 2.0,
            size.width - spacing,
            row_y + 26.0,
        );
        p.draw_text(
            "X",
            remove_rect.x0 + 12.0,
            row_y + 16.0,
            theme.secondary,
            10.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(remove_rect, ClickAction::RemoveFromPlaylist(idx));
        state.register_click(row, ClickAction::PlayPlaylistItem(idx));
    }
    dy += playlist_count as f64 * row_height;
    dy += 8.0;

    // "+ Add Files" button
    let add_btn_rect = Rect::new(
        video_rect.x1 + spacing,
        dy,
        video_rect.x1 + spacing + 120.0,
        dy + 28.0,
    );
    p.fill_rounded_rect(add_btn_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "+ Add Files",
        add_btn_rect.x0 + 60.0,
        dy + 18.0,
        Color::WHITE,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
    );
    state.register_click(add_btn_rect, ClickAction::AddToPlaylist);
    dy += 36.0;

    p.draw_text(
        "Recent",
        video_rect.x1 + spacing,
        dy,
        theme.secondary,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
    );
    dy += 20.0;
    let recent_data: Vec<(usize, String, Rect)> = state
        .recent_files
        .iter()
        .enumerate()
        .map(|(recent_idx, entry)| {
            let rect = Rect::new(
                video_rect.x1 + spacing,
                dy - 8.0 + (recent_idx as f64) * 18.0,
                size.width - spacing,
                dy + 10.0 + (recent_idx as f64) * 18.0,
            );
            (recent_idx, entry.title.clone(), rect)
        })
        .collect();
    for (recent_idx, title, recent_rect) in &recent_data {
        p.draw_text(
            title,
            video_rect.x1 + spacing,
            dy + (*recent_idx as f64) * 18.0,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
        state.register_click(*recent_rect, ClickAction::OpenRecentFile(*recent_idx));
    }
}
