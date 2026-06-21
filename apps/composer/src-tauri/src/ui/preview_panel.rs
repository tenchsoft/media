//! Preview monitor rendering for Composer.

use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::preview;
use crate::ui::state::ComposerState;

pub fn paint_preview(
    p: &mut Painter,
    state: &ComposerState,
    size: Size,
    theme: &Theme,
    text_cache: &mut TextCache,
) {
    let left_w = state.left_panel_w;
    let right_w = state.right_panel_w;
    let timeline_h = state.timeline_h;
    let toolbar_h = 48.0;

    let center_right = size.width - right_w;
    let preview_bottom = size.height - timeline_h;

    let preview_rect = Rect::new(left_w, toolbar_h, center_right, preview_bottom);
    p.fill_rect(preview_rect, Color::rgb8(0x0A, 0x0A, 0x0A));

    let mon_rect = preview::monitor_rect(preview_rect);
    let px = mon_rect.x0;
    let py = mon_rect.y0;
    let ph = mon_rect.y1 - mon_rect.y0;
    p.fill_rounded_rect(mon_rect, Color::rgb8(0x15, 0x15, 0x20), 4.0);
    p.stroke_rounded_rect(mon_rect, Color::rgb8(0x30, 0x30, 0x40), 1.0, 4.0);

    // Playback state / shuttle indicator
    let state_label = if state.is_playing {
        preview::shuttle_label(state.shuttle_direction, state.shuttle_speed)
    } else {
        "Paused".into()
    };
    p.draw_text_cached(
        text_cache,
        &state_label,
        px + 10.0,
        py + 22.0,
        Color::rgba8(255, 255, 255, 180),
        theme.font_size_small,
        FontWeight::BOLD,
        false,
        false,
    );

    // Frame counter / timecode (HH:MM:SS:FF format)
    let total_frames = state.total_frames();
    let fps = state.fps();
    p.draw_text_cached(
        text_cache,
        &format!(
            "{} / {}  |  Frame {}/{}",
            preview::format_timecode(state.current_frame, fps),
            preview::format_timecode(total_frames, fps),
            state.current_frame,
            total_frames,
        ),
        px + 10.0,
        py + ph - 10.0,
        Color::rgba8(255, 255, 255, 180),
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
        false,
    );

    // Playhead indicator
    let playhead_x = preview::monitor_playhead_x(mon_rect, state.current_frame, total_frames);
    p.draw_line(
        Point::new(playhead_x, py + ph - 20.0),
        Point::new(playhead_x, py + ph - 4.0),
        theme.primary,
        2.0,
    );

    // In/Out point visualization on monitor
    if let Some(inp) = state.in_point {
        let in_x = preview::monitor_playhead_x(mon_rect, inp, total_frames);
        p.draw_line(
            Point::new(in_x, py + 4.0),
            Point::new(in_x, py + ph - 4.0),
            Color::rgb8(0x22, 0xC5, 0x5E),
            1.5,
        );
        p.draw_text_cached(
            text_cache,
            "IN",
            in_x + 2.0,
            py + 14.0,
            Color::rgb8(0x22, 0xC5, 0x5E),
            9.0,
            FontWeight::BOLD,
            false,
            false,
        );
    }
    if let Some(outp) = state.out_point {
        let out_x = preview::monitor_playhead_x(mon_rect, outp, total_frames);
        p.draw_line(
            Point::new(out_x, py + 4.0),
            Point::new(out_x, py + ph - 4.0),
            Color::rgb8(0xEF, 0x44, 0x44),
            1.5,
        );
        p.draw_text_cached(
            text_cache,
            "OUT",
            out_x + 2.0,
            py + 14.0,
            Color::rgb8(0xEF, 0x44, 0x44),
            9.0,
            FontWeight::BOLD,
            false,
            false,
        );
    }

    // Center-right separator
    p.draw_line(
        Point::new(center_right, toolbar_h),
        Point::new(center_right, size.height - timeline_h),
        theme.border,
        1.0,
    );
}
