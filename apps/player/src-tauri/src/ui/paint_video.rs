//! Video surface rendering: video frame, empty state, subtitles, GIF indicator.

use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::state::{AspectMode, PlayerState};
use super::theme::{GRID_COLOR, PLAY_ICON_FAINT, SUBTITLE_BG};

// Paint the video frame, empty state, subtitle overlay, and GIF recording indicator.
//
// NOTE: this function intentionally accepts many parameters because the
// video rendering pipeline needs access to multiple independent pieces of
// state (frame data, video dimensions, recording state, layout metrics).
#[allow(clippy::too_many_arguments)]
pub fn paint_video(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    cx: f64,
    cy: f64,
    video_frame: Option<&tench_ui::peniko::ImageData>,
    video_dims: (u32, u32),
    gif_recording: bool,
    gif_recording_start: Option<std::time::Instant>,
) {
    if let Some(frame) = video_frame {
        paint_video_frame(p, state, video_rect, frame, video_dims);
    } else if state.has_media {
        paint_loading_or_audio(p, state, theme, video_rect, cx, cy);
    } else {
        paint_empty_state(p, cx, cy);
    }

    paint_subtitles(p, state, theme, video_rect, cx);
    paint_gif_indicator(p, video_rect, gif_recording, gif_recording_start);
}

fn paint_video_frame(
    p: &mut Painter<'_>,
    state: &PlayerState,
    video_rect: &Rect,
    frame: &tench_ui::peniko::ImageData,
    video_dims: (u32, u32),
) {
    let (fw, fh) = video_dims;
    let area_w = video_rect.x1;
    let area_h = video_rect.y1 - video_rect.y0;

    let (draw_w, draw_h) = match state.aspect_mode {
        AspectMode::Fit => {
            let scale_x = area_w / (fw as f64);
            let scale_y = area_h / (fh as f64);
            let scale = scale_x.min(scale_y);
            ((fw as f64) * scale, (fh as f64) * scale)
        }
        AspectMode::Fill => {
            let scale_x = area_w / (fw as f64);
            let scale_y = area_h / (fh as f64);
            let scale = scale_x.max(scale_y);
            ((fw as f64) * scale, (fh as f64) * scale)
        }
        AspectMode::Original => (fw as f64, fh as f64),
        AspectMode::SixteenNine => {
            let target_ratio = 16.0 / 9.0;
            let draw_w = area_w;
            let draw_h = area_w / target_ratio;
            if draw_h <= area_h {
                (draw_w, draw_h)
            } else {
                (area_h * target_ratio, area_h)
            }
        }
        AspectMode::FourThree => {
            let target_ratio = 4.0 / 3.0;
            let draw_w = area_w;
            let draw_h = area_w / target_ratio;
            if draw_h <= area_h {
                (draw_w, draw_h)
            } else {
                (area_h * target_ratio, area_h)
            }
        }
    };

    let draw_x = (area_w - draw_w) / 2.0;
    let draw_y = video_rect.y0 + (area_h - draw_h) / 2.0;
    let draw_rect = Rect::new(draw_x, draw_y, draw_x + draw_w, draw_y + draw_h);
    p.draw_image(frame, draw_rect);
}

fn paint_loading_or_audio(
    p: &mut Painter<'_>,
    state: &PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    cx: f64,
    cy: f64,
) {
    if state.media_info.resolution == "Audio only" && !state.audio_levels.is_empty() {
        let bar_count = state.audio_levels.len().min(32);
        let total_bar_w = video_rect.width() * 0.6;
        let bar_w = total_bar_w / bar_count as f64;
        let gap = bar_w * 0.2;
        let effective_bar_w = bar_w - gap;
        let start_x = video_rect.x0 + (video_rect.width() - total_bar_w) / 2.0;
        let base_y = cy + 40.0;
        let max_bar_h = 80.0;

        for (i, &db) in state.audio_levels.iter().take(bar_count).enumerate() {
            let normalized = ((db + 60.0) / 60.0).clamp(0.0, 1.0);
            let bar_h = max_bar_h * normalized;
            let bx = start_x + i as f64 * bar_w;
            let bar_rect = Rect::new(bx, base_y - bar_h, bx + effective_bar_w, base_y);
            let color = if normalized > 0.8 {
                Color::rgba8(255, 100, 100, 200)
            } else if normalized > 0.5 {
                Color::rgba8(255, 200, 100, 200)
            } else {
                theme.primary
            };
            p.fill_rounded_rect(bar_rect, color, 2.0);
        }

        p.draw_text(
            "Audio",
            cx,
            cy - 30.0,
            theme.on_surface,
            20.0,
            FontWeight::BOLD,
            true,
        );
    } else {
        p.draw_text(
            "Loading...",
            cx,
            cy,
            PLAY_ICON_FAINT,
            24.0,
            FontWeight::MEDIUM,
            true,
        );
    }
}

fn paint_empty_state(p: &mut Painter<'_>, cx: f64, cy: f64) {
    p.draw_text(
        "Drop a media file here",
        cx,
        cy - 12.0,
        PLAY_ICON_FAINT,
        18.0,
        FontWeight::MEDIUM,
        true,
    );
    p.draw_text(
        "or press O to open",
        cx,
        cy + 12.0,
        GRID_COLOR,
        14.0,
        FontWeight::NORMAL,
        true,
    );
}

fn paint_subtitles(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    cx: f64,
) {
    let Some(subtitle) = &state.subtitle_text else {
        return;
    };

    let font_size = state.subtitle_font_size;
    let sub_y = video_rect.y1 - 48.0;

    let lines: Vec<&str> = subtitle.split('\n').collect();
    let line_height = font_size + 4.0;
    let line_height_px = f64::from(line_height);
    let total_h = line_height_px * lines.len() as f64;
    let start_y = sub_y - (total_h - line_height_px) / 2.0;

    let bg_pad = 12.0;
    let max_line_w = lines
        .iter()
        .map(|l| l.len() as f64 * font_size as f64 * 0.55)
        .fold(0.0f64, f64::max);
    let pill_w = (max_line_w + bg_pad * 2.0).clamp(120.0, video_rect.width() - 40.0);
    let bg_rect = Rect::new(
        cx - pill_w / 2.0,
        start_y - line_height_px / 2.0 - bg_pad,
        cx + pill_w / 2.0,
        start_y + total_h - line_height_px / 2.0 + bg_pad,
    );
    p.fill_rounded_rect(bg_rect, SUBTITLE_BG, theme.border_radius);

    for (i, line) in lines.iter().enumerate() {
        let ly = start_y + i as f64 * line_height_px;
        let outline_offsets = [
            (-1.0, -1.0),
            (1.0, -1.0),
            (-1.0, 1.0),
            (1.0, 1.0),
            (0.0, -1.0),
            (0.0, 1.0),
            (-1.0, 0.0),
            (1.0, 0.0),
        ];
        for (ox, oy) in &outline_offsets {
            p.draw_text(
                line,
                cx + ox,
                ly + oy,
                Color::BLACK,
                font_size,
                FontWeight::MEDIUM,
                true,
            );
        }
        p.draw_text(
            line,
            cx,
            ly,
            Color::WHITE,
            font_size,
            FontWeight::MEDIUM,
            true,
        );
    }
}

fn paint_gif_indicator(
    p: &mut Painter<'_>,
    video_rect: &Rect,
    gif_recording: bool,
    gif_recording_start: Option<std::time::Instant>,
) {
    if !gif_recording {
        return;
    }
    let dot_r = 6.0;
    let dot_x = video_rect.x1 - 20.0;
    let dot_y = video_rect.y0 + 20.0;

    // Blinking effect: toggle visibility every 500ms
    let blink_on = gif_recording_start.is_none_or(|start| {
        let elapsed_ms = start.elapsed().as_millis() as u64;
        (elapsed_ms / 500).is_multiple_of(2)
    });

    if blink_on {
        p.fill_circle(
            Point::new(dot_x, dot_y),
            dot_r,
            Color::rgba8(255, 0, 0, 220),
        );
    }
    p.draw_text(
        "REC",
        dot_x - 30.0,
        dot_y + 4.0,
        Color::WHITE,
        10.0,
        FontWeight::BOLD,
        false,
    );

    // Show elapsed time
    if let Some(start) = gif_recording_start {
        let elapsed = start.elapsed().as_secs();
        let mins = elapsed / 60;
        let secs = elapsed % 60;
        let elapsed_text = format!("{:02}:{:02}", mins, secs);
        p.draw_text(
            &elapsed_text,
            dot_x + 10.0,
            dot_y + 4.0,
            Color::WHITE,
            10.0,
            FontWeight::NORMAL,
            false,
        );
    }
}
