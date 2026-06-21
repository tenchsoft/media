use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::*;
use crate::ui::theme::{BTN_ACTION, BTN_DIM};

#[allow(clippy::too_many_arguments)]
pub(super) fn paint_subtitles_tab(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    size: Size,
    spacing: f64,
    mut dy: f64,
) {
    // Subtitle search button
    let search_rect = Rect::new(
        video_rect.x1 + spacing,
        dy,
        video_rect.x1 + spacing + 70.0,
        dy + 22.0,
    );
    p.fill_rounded_rect(search_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Search",
        search_rect.x0 + 35.0,
        dy + 13.0,
        Color::WHITE,
        9.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(search_rect, ClickAction::ShowSubtitleSearch);
    // Style button
    let style_rect = Rect::new(
        video_rect.x1 + spacing + 76.0,
        dy,
        video_rect.x1 + spacing + 146.0,
        dy + 22.0,
    );
    p.fill_rounded_rect(style_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Style",
        style_rect.x0 + 35.0,
        dy + 13.0,
        Color::WHITE,
        9.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(style_rect, ClickAction::ShowSubtitleStyle);
    dy += 30.0;

    // External subtitle tracks
    if !state.subtitle_tracks.is_empty() {
        p.draw_text(
            "External",
            video_rect.x1 + spacing,
            dy,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        dy += 22.0;
    }
    for (idx, sub) in state.subtitle_tracks.clone().iter().enumerate() {
        let rect = Rect::new(
            video_rect.x1 + spacing,
            dy - 8.0,
            size.width - spacing - 80.0,
            dy + 16.0,
        );
        let label = if sub.language.is_empty() {
            format!(
                "Track {}  {}ms{}",
                idx + 1,
                sub.offset_ms,
                if sub.active { "  active" } else { "" }
            )
        } else {
            format!(
                "{}  {}ms{}",
                sub.language,
                sub.offset_ms,
                if sub.active { "  active" } else { "" }
            )
        };
        p.draw_text(
            &label,
            video_rect.x1 + spacing,
            dy,
            if sub.active {
                theme.primary
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        state.register_click(rect, ClickAction::SelectSubtitleTrack(idx));

        // Offset +/- buttons
        let minus_rect = Rect::new(
            size.width - spacing - 76.0,
            dy - 8.0,
            size.width - spacing - 52.0,
            dy + 12.0,
        );
        p.fill_rounded_rect(minus_rect, BTN_DIM, theme.border_radius);
        p.draw_text(
            "-",
            minus_rect.x0 + 12.0,
            dy + 3.0,
            theme.on_surface,
            10.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(minus_rect, ClickAction::SubtitleOffsetForTrack(idx, -100));

        let plus_rect = Rect::new(
            size.width - spacing - 48.0,
            dy - 8.0,
            size.width - spacing - 24.0,
            dy + 12.0,
        );
        p.fill_rounded_rect(plus_rect, BTN_DIM, theme.border_radius);
        p.draw_text(
            "+",
            plus_rect.x0 + 12.0,
            dy + 3.0,
            theme.on_surface,
            10.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(plus_rect, ClickAction::SubtitleOffsetForTrack(idx, 100));

        dy += 24.0;
    }

    // Built-in subtitle tracks (from container)
    if state.n_builtin_subtitle_tracks > 0 {
        dy += 8.0;
        p.draw_text(
            "Built-in",
            video_rect.x1 + spacing,
            dy,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        dy += 22.0;

        // Disable option
        let disable_rect = Rect::new(
            video_rect.x1 + spacing,
            dy - 8.0,
            size.width - spacing,
            dy + 16.0,
        );
        let disable_active = state.active_builtin_subtitle_track < 0;
        p.draw_text(
            if disable_active {
                "None  active"
            } else {
                "None"
            },
            video_rect.x1 + spacing,
            dy,
            if disable_active {
                theme.primary
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        state.register_click(disable_rect, ClickAction::SelectBuiltinSubtitleTrack(-1));
        dy += 24.0;

        for idx in 0..state.n_builtin_subtitle_tracks {
            let i = idx as i32;
            let rect = Rect::new(
                video_rect.x1 + spacing,
                dy - 8.0,
                size.width - spacing,
                dy + 16.0,
            );
            let is_active = state.active_builtin_subtitle_track == i;
            let labels = &state.builtin_subtitle_labels;
            let lang_label = if (idx as usize) < labels.len() && !labels.is_empty() {
                format!(
                    "Track {} [{}]{}",
                    idx + 1,
                    labels[idx as usize],
                    if is_active { "  active" } else { "" }
                )
            } else {
                format!(
                    "Track {}{}",
                    idx + 1,
                    if is_active { "  active" } else { "" }
                )
            };
            p.draw_text(
                &lang_label,
                video_rect.x1 + spacing,
                dy,
                if is_active {
                    theme.primary
                } else {
                    theme.on_surface
                },
                theme.font_size_small,
                FontWeight::MEDIUM,
                false,
            );
            state.register_click(rect, ClickAction::SelectBuiltinSubtitleTrack(i));
            dy += 24.0;
        }
    }

    // Encoding selector
    dy += 8.0;
    p.draw_text(
        "Encoding",
        video_rect.x1 + spacing,
        dy,
        theme.secondary,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
    );
    dy += 18.0;
    let encodings = [
        ("Auto", None),
        ("UTF-8", Some(SubtitleEncoding::Utf8)),
        ("Shift-JIS", Some(SubtitleEncoding::ShiftJIS)),
        ("EUC-KR", Some(SubtitleEncoding::EucKR)),
        ("CP1252", Some(SubtitleEncoding::Cp1252)),
    ];
    for (enc_name, enc_val) in encodings.iter() {
        let enc_rect = Rect::new(
            video_rect.x1 + spacing,
            dy - 8.0,
            video_rect.x1 + spacing + 80.0,
            dy + 16.0,
        );
        let is_current = match enc_val {
            None => false,
            Some(ev) => state.subtitle_encoding == *ev,
        };
        p.draw_text(
            enc_name,
            video_rect.x1 + spacing,
            dy,
            if is_current {
                theme.primary
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        state.register_click(
            enc_rect,
            ClickAction::SetSubtitleEncoding(enc_name.to_string()),
        );
        dy += 20.0;
    }
}
