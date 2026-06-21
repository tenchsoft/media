//! Timeline panel rendering for Composer (tracks, clips, ruler, playhead, render queue, AI panel).

use tench_composer_core::*;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{clip_color_for_index, ClickAction, ComposerState};
use crate::ui::timeline;

pub fn paint_timeline(
    p: &mut Painter,
    state: &ComposerState,
    size: Size,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
) {
    let left_w = state.left_panel_w;
    let timeline_h = state.timeline_h;
    let spacing = theme.spacing;
    let tl_y = size.height - timeline_h;

    // Timeline background
    let tl_rect = Rect::new(0.0, tl_y, size.width, size.height);
    p.fill_rect(tl_rect, Color::rgb8(0x14, 0x14, 0x1E));

    p.draw_line(
        Point::new(0.0, tl_y),
        Point::new(size.width, tl_y),
        theme.border,
        1.0,
    );

    let header_h = timeline::HEADER_H;
    let timeline_toolbar_h = timeline::TOOLBAR_H;
    let tl_header = Rect::new(left_w, tl_y, size.width, tl_y + header_h);
    p.fill_rect(tl_header, Color::rgb8(0x1A, 0x1A, 0x28));

    let tl_content_w = timeline::content_width(size.width, left_w);
    let total_frames = state.total_frames();
    let fps = state.fps();

    // Ruler markers
    let frames_per_marker = 60;
    let mut marker_frame = 0u32;
    while marker_frame <= total_frames {
        let mx = timeline::frame_to_x(left_w, tl_content_w, marker_frame, total_frames);
        let secs = marker_frame as f64 / fps;
        p.draw_text_cached(
            text_cache,
            &format!("{:.0}s", secs),
            mx + 2.0,
            tl_y + 14.0,
            theme.disabled,
            10.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        p.draw_line(
            Point::new(mx, tl_y + header_h + timeline_toolbar_h),
            Point::new(mx, size.height),
            Color::rgb8(0x25, 0x25, 0x35),
            0.5,
        );
        marker_frame += frames_per_marker;
    }

    // In/Out point markers on ruler
    if let Some(inp) = state.in_point {
        let in_x = timeline::frame_to_x(left_w, tl_content_w, inp, total_frames);
        p.draw_line(
            Point::new(in_x, tl_y),
            Point::new(in_x, tl_y + header_h),
            Color::rgb8(0x22, 0xC5, 0x5E),
            2.0,
        );
    }
    if let Some(outp) = state.out_point {
        let out_x = timeline::frame_to_x(left_w, tl_content_w, outp, total_frames);
        p.draw_line(
            Point::new(out_x, tl_y),
            Point::new(out_x, tl_y + header_h),
            Color::rgb8(0xEF, 0x44, 0x44),
            2.0,
        );
    }

    // Timeline tool strip
    let tool_y = tl_y + header_h;
    p.fill_rect(
        Rect::new(left_w, tool_y, size.width, tool_y + timeline_toolbar_h),
        Color::rgb8(0x16, 0x16, 0x24),
    );

    p.draw_text_cached(
        text_cache,
        &format!("Zoom {:.0}%", state.zoom),
        left_w + spacing,
        tool_y + 18.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::MEDIUM,
        false,
        false,
    );

    let toggles = [
        ("Snap", state.snap, ClickAction::ToggleSnap),
        ("Ripple", state.ripple, ClickAction::ToggleRipple),
        ("Magnet", state.magnetic, ClickAction::ToggleMagnet),
    ];
    let mut tx = left_w + 92.0;
    for (label, active, action) in toggles {
        let rect = Rect::new(tx, tool_y + 5.0, tx + 58.0, tool_y + 23.0);
        p.fill_rounded_rect(
            rect,
            if active {
                theme.primary
            } else {
                theme.background
            },
            theme.border_radius,
        );
        p.draw_text_cached(
            text_cache,
            label,
            tx + 29.0,
            tool_y + 18.0,
            if active {
                theme.on_primary
            } else {
                theme.on_surface
            },
            10.0,
            FontWeight::MEDIUM,
            true,
            false,
        );
        register_click(rect, action);
        tx += 64.0;
    }

    // "+ Add Track" button
    let add_rect = Rect::new(tx + 8.0, tool_y + 5.0, tx + 96.0, tool_y + 23.0);
    p.fill_rounded_rect(add_rect, theme.background, theme.border_radius);
    p.stroke_rounded_rect(add_rect, theme.border, 1.0, theme.border_radius);
    p.draw_text_cached(
        text_cache,
        "+ Add Track",
        tx + 52.0,
        tool_y + 18.0,
        theme.on_surface,
        10.0,
        FontWeight::MEDIUM,
        true,
        false,
    );
    register_click(add_rect, ClickAction::AddTrack(TrackType::Video));

    // Track labels and clips
    let selected_clip_id = state.selected_clip_id;
    let tracks_owned: Vec<_> = state.tracks().to_vec();
    let track_h = timeline::track_height(timeline_h, tracks_owned.len());
    for (i, track) in tracks_owned.iter().enumerate() {
        let ty = tl_y + header_h + timeline_toolbar_h + (i as f64) * track_h;

        // Track label header
        let label_rect = Rect::new(0.0, ty, left_w, ty + track_h);
        p.fill_rect(label_rect, theme.surface);
        let badge = track.kind.badge();
        p.draw_text_cached(
            text_cache,
            &format!("{} {}", badge, track.name),
            spacing,
            ty + track_h / 2.0 - 6.0,
            if track.muted {
                theme.disabled
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
            false,
        );

        // Track header buttons: Mute / Lock / Hidden
        let btn_w = 18.0;
        let btn_h = 14.0;
        let btn_y = ty + track_h / 2.0 + 4.0;
        let mut btn_x = spacing;

        // Mute button
        let mute_rect = Rect::new(btn_x, btn_y, btn_x + btn_w, btn_y + btn_h);
        p.fill_rounded_rect(
            mute_rect,
            if track.muted {
                Color::rgb8(0xEF, 0x44, 0x44)
            } else {
                theme.background
            },
            2.0,
        );
        p.draw_text_cached(
            text_cache,
            "M",
            btn_x + btn_w / 2.0,
            btn_y + 10.0,
            if track.muted {
                Color::WHITE
            } else {
                theme.disabled
            },
            8.0,
            FontWeight::BOLD,
            true,
            false,
        );
        register_click(mute_rect, ClickAction::ToggleTrackMute(track.id));
        btn_x += btn_w + 2.0;

        // Lock button
        let lock_rect = Rect::new(btn_x, btn_y, btn_x + btn_w, btn_y + btn_h);
        p.fill_rounded_rect(
            lock_rect,
            if track.locked {
                Color::rgb8(0xF5, 0x9E, 0x0B)
            } else {
                theme.background
            },
            2.0,
        );
        p.draw_text_cached(
            text_cache,
            "L",
            btn_x + btn_w / 2.0,
            btn_y + 10.0,
            if track.locked {
                Color::WHITE
            } else {
                theme.disabled
            },
            8.0,
            FontWeight::BOLD,
            true,
            false,
        );
        register_click(lock_rect, ClickAction::ToggleTrackLock(track.id));
        btn_x += btn_w + 2.0;

        // Hidden button
        let hide_rect = Rect::new(btn_x, btn_y, btn_x + btn_w, btn_y + btn_h);
        p.fill_rounded_rect(
            hide_rect,
            if track.hidden {
                Color::rgb8(0x6B, 0x72, 0x80)
            } else {
                theme.background
            },
            2.0,
        );
        p.draw_text_cached(
            text_cache,
            "H",
            btn_x + btn_w / 2.0,
            btn_y + 10.0,
            if track.hidden {
                Color::WHITE
            } else {
                theme.disabled
            },
            8.0,
            FontWeight::BOLD,
            true,
            false,
        );
        register_click(hide_rect, ClickAction::ToggleTrackHidden(track.id));

        // Track separator
        p.draw_line(
            Point::new(0.0, ty),
            Point::new(size.width, ty),
            Color::rgb8(0x25, 0x25, 0x35),
            0.5,
        );

        // Clips
        let clip_clicks: Vec<_> = track
            .clips
            .iter()
            .map(|clip| {
                (
                    timeline::clip_rect(
                        left_w,
                        tl_content_w,
                        total_frames,
                        ty,
                        track_h,
                        clip.timeline_in,
                        clip.timeline_out(),
                    ),
                    clip.id,
                    clip.name.clone(),
                    clip.transition_in.is_some(),
                    clip.transition_out.is_some(),
                )
            })
            .collect();
        for (clip_idx, (crect, clip_id, clip_name, has_trans_in, has_trans_out)) in
            clip_clicks.iter().enumerate()
        {
            let color = clip_color_for_index(clip_idx);
            p.fill_rounded_rect(*crect, color, 3.0);

            // Trim handles (visual indicator on clip edges)
            let handle_w = timeline::TRIM_HANDLE_W;
            // Left handle (trim in)
            let left_handle = Rect::new(crect.x0, crect.y0, crect.x0 + handle_w, crect.y1);
            p.fill_rounded_rect(left_handle, Color::rgba8(255, 255, 255, 30), 3.0);
            // Right handle (trim out)
            let right_handle = Rect::new(crect.x1 - handle_w, crect.y0, crect.x1, crect.y1);
            p.fill_rounded_rect(right_handle, Color::rgba8(255, 255, 255, 30), 3.0);

            // Transition visualization
            if *has_trans_in {
                let trans_w = 12.0;
                let trans_rect = Rect::new(crect.x0, crect.y0, crect.x0 + trans_w, crect.y1);
                p.fill_rounded_rect(trans_rect, Color::rgba8(255, 255, 255, 60), 3.0);
            }
            if *has_trans_out {
                let trans_w = 12.0;
                let trans_rect = Rect::new(crect.x1 - trans_w, crect.y0, crect.x1, crect.y1);
                p.fill_rounded_rect(trans_rect, Color::rgba8(255, 255, 255, 60), 3.0);
            }

            // Selection highlight
            if selected_clip_id == Some(*clip_id) {
                p.stroke_rounded_rect(*crect, theme.on_primary, 1.5, 3.0);
            }
            p.draw_text_cached(
                text_cache,
                clip_name,
                crect.x0 + 5.0,
                ty + track_h / 2.0 + 4.0,
                theme.on_primary,
                10.0,
                FontWeight::MEDIUM,
                false,
                false,
            );
            register_click(*crect, ClickAction::SelectClip(Some(*clip_id)));
        }
    }

    // Playhead line
    let playhead_x = timeline::frame_to_x(left_w, tl_content_w, state.current_frame, total_frames);
    p.draw_line(
        Point::new(playhead_x, tl_y),
        Point::new(playhead_x, size.height),
        Color::rgb8(0xEF, 0x44, 0x44),
        2.0,
    );

    // Playhead triangle
    let tri_size = 6.0;
    let tri_rect = Rect::new(
        playhead_x - tri_size,
        tl_y,
        playhead_x + tri_size,
        tl_y + tri_size * 2.0,
    );
    p.fill_rect(tri_rect, Color::rgb8(0xEF, 0x44, 0x44));
}

/// Paint the render queue drawer overlay.
pub fn paint_render_queue(
    p: &mut Painter,
    state: &ComposerState,
    size: Size,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
) {
    if !state.show_render_queue {
        return;
    }
    let toolbar_h = 48.0;
    let timeline_h = state.timeline_h;
    let tl_y = size.height - timeline_h;

    let drawer = Rect::new(
        size.width - 340.0,
        toolbar_h + 12.0,
        size.width - 12.0,
        tl_y - 12.0,
    );
    p.fill_rounded_rect(drawer, theme.surface, theme.border_radius);
    p.stroke_rounded_rect(drawer, theme.border, 1.0, theme.border_radius);

    // Header with close button
    p.draw_text_cached(
        text_cache,
        "Render Queue",
        drawer.x0 + 12.0,
        drawer.y0 + 22.0,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
        false,
    );

    // X close button
    let close_rect = Rect::new(
        drawer.x1 - 32.0,
        drawer.y0 + 8.0,
        drawer.x1 - 8.0,
        drawer.y0 + 28.0,
    );
    p.fill_rounded_rect(close_rect, theme.background, theme.border_radius);
    p.draw_text_cached(
        text_cache,
        "X",
        drawer.x1 - 20.0,
        drawer.y0 + 22.0,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        true,
        false,
    );
    register_click(close_rect, ClickAction::CloseRenderQueue);

    let mut y = drawer.y0 + 48.0;
    for job in state.render_jobs() {
        let row = Rect::new(drawer.x0 + 12.0, y - 8.0, drawer.x1 - 12.0, y + 34.0);
        p.fill_rounded_rect(row, theme.background, theme.border_radius);
        p.draw_text_cached(
            text_cache,
            &job.name,
            row.x0 + 8.0,
            y + 6.0,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
            false,
        );
        let status_label = match job.status {
            RenderStatus::Queued => "Queued",
            RenderStatus::Rendering => "Rendering",
            RenderStatus::Completed => "Done",
            RenderStatus::Failed => "Failed",
        };
        p.draw_text_cached(
            text_cache,
            &format!("{}% {}", job.progress, status_label),
            row.x0 + 8.0,
            y + 22.0,
            theme.secondary,
            10.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        let track = Rect::new(row.x0 + 150.0, y + 12.0, row.x1 - 8.0, y + 18.0);
        p.fill_rounded_rect(track, theme.border, 3.0);
        p.fill_rounded_rect(
            Rect::new(
                track.x0,
                track.y0,
                track.x0 + (track.x1 - track.x0) * (job.progress as f64 / 100.0),
                track.y1,
            ),
            theme.primary,
            3.0,
        );

        // Cancel / Pause buttons
        let cancel_rect = Rect::new(row.x1 - 50.0, y - 4.0, row.x1 - 28.0, y + 8.0);
        p.fill_rounded_rect(cancel_rect, Color::rgb8(0xEF, 0x44, 0x44), 2.0);
        p.draw_text_cached(
            text_cache,
            "X",
            cancel_rect.x0 + 11.0,
            cancel_rect.y0 + 9.0,
            Color::WHITE,
            8.0,
            FontWeight::BOLD,
            true,
            false,
        );
        register_click(cancel_rect, ClickAction::CancelRenderJob(job.id));

        let pause_rect = Rect::new(row.x1 - 26.0, y - 4.0, row.x1 - 4.0, y + 8.0);
        p.fill_rounded_rect(pause_rect, Color::rgb8(0xF5, 0x9E, 0x0B), 2.0);
        p.draw_text_cached(
            text_cache,
            "||",
            pause_rect.x0 + 11.0,
            pause_rect.y0 + 9.0,
            Color::WHITE,
            8.0,
            FontWeight::BOLD,
            true,
            false,
        );
        register_click(pause_rect, ClickAction::PauseRenderJob(job.id));

        y += 50.0;
    }
}

/// Paint the AI panel overlay.
pub fn paint_ai_panel(
    p: &mut Painter,
    state: &ComposerState,
    size: Size,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
) {
    if !state.show_ai_panel {
        return;
    }
    let toolbar_h = 48.0;
    let timeline_h = state.timeline_h;
    let right_w = state.right_panel_w;
    let center_right = size.width - right_w;
    let tl_y = size.height - timeline_h;

    let panel = Rect::new(
        center_right - 300.0,
        toolbar_h + 12.0,
        center_right - 12.0,
        tl_y - 12.0,
    );
    p.fill_rounded_rect(panel, Color::rgb8(0x1B, 0x20, 0x30), theme.border_radius);
    p.stroke_rounded_rect(panel, theme.border, 1.0, theme.border_radius);
    p.draw_text_cached(
        text_cache,
        "Composer AI",
        panel.x0 + 12.0,
        panel.y0 + 24.0,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
        false,
    );

    // AI feature list
    let features = [
        ("Auto Cut", "Detect scene changes and auto-split clips"),
        ("Auto Subtitle", "Generate subtitles from audio"),
        ("Voice-to-Text", "Transcribe speech to text"),
        ("BG Removal", "Remove background from video"),
        ("Color Match", "Match colors between clips"),
    ];
    let mut fy = panel.y0 + 56.0;
    for (name, desc) in &features {
        let row_rect = Rect::new(panel.x0 + 12.0, fy - 4.0, panel.x1 - 12.0, fy + 36.0);
        p.fill_rounded_rect(row_rect, Color::rgb8(0x25, 0x2A, 0x3A), theme.border_radius);
        p.draw_text_cached(
            text_cache,
            name,
            panel.x0 + 20.0,
            fy + 8.0,
            theme.on_surface,
            theme.font_size,
            FontWeight::MEDIUM,
            false,
            false,
        );
        p.draw_text_cached(
            text_cache,
            desc,
            panel.x0 + 20.0,
            fy + 24.0,
            theme.secondary,
            10.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        register_click(row_rect, ClickAction::RunAiFeature((*name).to_string()));
        fy += 48.0;
    }
}

/// Paint quick action buttons (Render Queue / AI panel toggles).
pub fn paint_quick_actions(
    p: &mut Painter,
    state: &ComposerState,
    size: Size,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
) {
    let right_w = state.right_panel_w;
    let center_right = size.width - right_w;
    let timeline_h = state.timeline_h;
    let tl_y = size.height - timeline_h;

    // Render Queue button
    let quick_x = center_right - 72.0;
    let rq_y = tl_y - 54.0;
    let rq_rect = Rect::new(quick_x, rq_y, quick_x + 60.0, rq_y + 30.0);
    p.fill_rounded_rect(rq_rect, theme.primary, theme.border_radius);
    p.draw_text_cached(
        text_cache,
        "Queue",
        quick_x + 30.0,
        rq_y + 19.0,
        theme.on_primary,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
        false,
    );
    register_click(rq_rect, ClickAction::ToggleRenderQueue);

    // AI button
    let ai_y = tl_y - 92.0;
    let ai_rect = Rect::new(quick_x, ai_y, quick_x + 60.0, ai_y + 30.0);
    p.fill_rounded_rect(ai_rect, theme.primary, theme.border_radius);
    p.draw_text_cached(
        text_cache,
        "AI",
        quick_x + 30.0,
        ai_y + 19.0,
        theme.on_primary,
        theme.font_size_small,
        FontWeight::BOLD,
        true,
        false,
    );
    register_click(ai_rect, ClickAction::ToggleAiPanel);
}
