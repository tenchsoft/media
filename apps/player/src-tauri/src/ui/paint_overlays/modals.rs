use tench_ui::prelude::*;

use crate::ui::state::PlayerState;

use super::{chapter, equalizer, gif_options, help, subtitle_search, subtitle_style, url};

/// Paint all modal overlays (help, URL, subtitle style/search, GIF options, EQ, add chapter).
pub fn paint_modals(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
) {
    let modal_w = 360.0;
    let modal_h = 280.0;
    let modal_x = (size.width - modal_w) / 2.0;
    let modal_y = (size.height - modal_h) / 2.0;

    help::paint_help_modal(p, state, theme, size, modal_x, modal_w);
    url::paint_url_modal(p, state, theme, size, modal_x, modal_y, modal_w);
    subtitle_style::paint_subtitle_style_modal(p, state, theme, size, modal_x, modal_w);
    subtitle_search::paint_subtitle_search_modal(p, state, theme, size, modal_x, modal_w);
    gif_options::paint_gif_options_modal(p, state, theme, size, modal_x, modal_w);
    equalizer::paint_equalizer_modal(p, state, theme, size, modal_x, modal_w);
    chapter::paint_add_chapter_modal(p, state, theme, size, modal_x, modal_w);
}
