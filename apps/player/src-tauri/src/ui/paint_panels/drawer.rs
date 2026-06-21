use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::ai_panel;
use crate::ui::state::*;

use super::{chapters, info, playlist, subtitles};

/// Paint the drawer tab panels (Playlist, Chapters, Subtitles, Info).
#[allow(clippy::too_many_arguments)]
pub fn paint_drawer(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    size: Size,
    spacing: f64,
    spacing_large: f64,
    backend: Option<&crate::gst_backend::PlayerBackend>,
) {
    let Some(drawer) = state.drawer else {
        return;
    };

    let drawer_rect = ai_panel::panel_rect(video_rect.x1, size.width, size.height);
    p.fill_rect(drawer_rect, theme.surface);
    p.draw_line(
        Point::new(video_rect.x1, 0.0),
        Point::new(video_rect.x1, size.height),
        theme.border,
        1.0,
    );
    let mut dy = spacing_large;
    p.draw_text(
        drawer.label(),
        video_rect.x1 + spacing,
        dy,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    dy += 30.0;
    dy -= state.drawer_scroll_y;

    match drawer {
        DrawerTab::Playlist => {
            playlist::paint_playlist_tab(p, state, theme, video_rect, size, spacing, dy)
        }
        DrawerTab::Chapters => {
            chapters::paint_chapters_tab(p, state, theme, video_rect, size, spacing, dy)
        }
        DrawerTab::Subtitles => {
            subtitles::paint_subtitles_tab(p, state, theme, video_rect, size, spacing, dy)
        }
        DrawerTab::Info => {
            info::paint_info_tab(p, state, theme, video_rect, size, spacing, dy, backend)
        }
    }
}
