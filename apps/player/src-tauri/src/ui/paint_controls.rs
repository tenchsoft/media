//! Controls rendering: top overlay bar, bottom controls bar, seekbar, speed menu.

use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::controls;
use super::state::{ClickAction, DrawerTab, PlayerState, RepeatMode};
use super::theme::{
    AB_LOOP_COLOR, BTN_ACTION, BTN_DEFAULT, CHAPTER_AI_MARKER, CHAPTER_MARKER, CONTROLS_BG,
    REMEMBERED_MARKER, SEEKBAR_TRACK, TOP_BAR_BG,
};

/// Paint the top overlay bar (title, drawer tabs, AI button).
pub fn paint_top_bar(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    overlay_h: f64,
    _spacing: f64,
    spacing_large: f64,
) {
    let top_rect = Rect::new(0.0, 0.0, video_rect.x1, overlay_h);
    p.fill_rect(top_rect, TOP_BAR_BG);

    // Title with truncation
    let title_display = if state.title.len() > 60 {
        format!("{}...", &state.title[..57])
    } else {
        state.title.clone()
    };
    p.draw_text(
        &title_display,
        spacing_large,
        22.0,
        Color::WHITE,
        theme.font_size,
        FontWeight::MEDIUM,
        false,
    );

    // Drawer tab buttons
    let mut top_btn_x = (video_rect.x1 - 390.0).max(spacing_large + 320.0);
    for tab in DrawerTab::ALL {
        let rect = Rect::new(top_btn_x, 6.0, top_btn_x + 74.0, 34.0);
        let active = state.drawer == Some(tab);
        p.fill_rounded_rect(
            rect,
            if active { theme.primary } else { BTN_DEFAULT },
            theme.border_radius,
        );
        p.draw_text(
            tab.label(),
            top_btn_x + 37.0,
            22.0,
            Color::WHITE,
            10.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(rect, ClickAction::ToggleDrawer(tab));
        top_btn_x += 78.0;
    }

    // AI panel toggle
    let ai_btn_rect = Rect::new(top_btn_x, 6.0, top_btn_x + 56.0, 34.0);
    p.fill_rounded_rect(
        ai_btn_rect,
        if state.ai_panel_open {
            theme.primary
        } else {
            BTN_DEFAULT
        },
        theme.border_radius,
    );
    p.draw_text(
        "AI",
        top_btn_x + 28.0,
        22.0,
        Color::WHITE,
        theme.font_size,
        FontWeight::BOLD,
        true,
    );
    state.register_click(ai_btn_rect, ClickAction::ToggleAiPanel);
}

// Paint the bottom controls bar: seekbar, buttons, volume, speed menu.
//
// NOTE: this function intentionally accepts many parameters because the
// control bar layout requires access to multiple independent pieces of
// state (seek hover position, thumbnail data, layout metrics). Grouping
// them into a single struct would add indirection without meaningful
// benefit since every field is used.
#[allow(clippy::too_many_arguments)]
pub fn paint_controls(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    video_rect: &Rect,
    controls_h: f64,
    spacing: f64,
    spacing_large: f64,
    seek_hover_pos: Option<f64>,
    seek_thumbnail: Option<&tench_ui::peniko::ImageData>,
) {
    let ctrl_y = size.height - controls_h;
    let ctrl_rect = Rect::new(0.0, ctrl_y, video_rect.x1, size.height);
    p.fill_rect(ctrl_rect, CONTROLS_BG);

    paint_seekbar(
        p,
        state,
        theme,
        size,
        video_rect,
        ctrl_y,
        controls_h,
        spacing_large,
        seek_hover_pos,
        seek_thumbnail,
    );
    paint_buttons_row(p, state, theme, video_rect, ctrl_y, spacing, spacing_large);
    paint_speed_menu(p, state, theme);
}

#[allow(clippy::too_many_arguments)]
fn paint_seekbar(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    size: Size,
    video_rect: &Rect,
    ctrl_y: f64,
    _controls_h: f64,
    seekbar_margin: f64,
    seek_hover_pos: Option<f64>,
    seek_thumbnail: Option<&tench_ui::peniko::ImageData>,
) {
    let seekbar_y = ctrl_y + 4.0;
    let seekbar_w = video_rect.x1 - seekbar_margin * 2.0;
    let seekbar_track = Rect::new(
        seekbar_margin,
        seekbar_y,
        seekbar_margin + seekbar_w,
        seekbar_y + 4.0,
    );
    p.fill_rounded_rect(seekbar_track, SEEKBAR_TRACK, 2.0);

    // Buffering progress bar
    if state.buffering_percent < 100 && state.duration > 0.0 {
        let buf_w = seekbar_w * (state.buffering_percent as f64 / 100.0);
        let buf_fill = Rect::new(
            seekbar_margin,
            seekbar_y,
            seekbar_margin + buf_w,
            seekbar_y + 4.0,
        );
        p.fill_rounded_rect(buf_fill, BTN_DEFAULT, 2.0);
    }

    // Seekbar progress
    let progress = if state.duration > 0.0 {
        state.current_time / state.duration
    } else {
        0.0
    };
    let progress_w = seekbar_w * progress;
    let seekbar_fill = Rect::new(
        seekbar_margin,
        seekbar_y,
        seekbar_margin + progress_w,
        seekbar_y + 4.0,
    );
    p.fill_rounded_rect(seekbar_fill, theme.primary, 2.0);

    // Remembered position marker
    if let Some(remembered) = state.remembered_position {
        if state.duration > 0.0 {
            let rx = seekbar_margin + seekbar_w * (remembered / state.duration).clamp(0.0, 1.0);
            p.draw_line(
                Point::new(rx, seekbar_y - 5.0),
                Point::new(rx, seekbar_y + 9.0),
                REMEMBERED_MARKER,
                1.0,
            );
            let marker_rect = Rect::new(rx - 4.0, seekbar_y - 6.0, rx + 4.0, seekbar_y + 10.0);
            state.register_click(marker_rect, ClickAction::JumpToRememberedPosition);
        }
    }

    // Chapter markers
    for chapter in &state.chapters {
        if state.duration > 0.0 {
            let mx = seekbar_margin + seekbar_w * (chapter.time / state.duration).clamp(0.0, 1.0);
            p.fill_rounded_rect(
                Rect::new(mx - 2.0, seekbar_y - 2.0, mx + 2.0, seekbar_y + 6.0),
                if chapter.ai_generated {
                    CHAPTER_AI_MARKER
                } else {
                    CHAPTER_MARKER
                },
                2.0,
            );
        }
    }

    // A-B loop highlight
    if let Some((a, b)) = state.ab_loop {
        if state.duration > 0.0 {
            let ax = seekbar_margin + seekbar_w * (a / state.duration).clamp(0.0, 1.0);
            let bx = seekbar_margin + seekbar_w * (b / state.duration).clamp(0.0, 1.0);
            p.fill_rounded_rect(
                Rect::new(ax, seekbar_y + 7.0, bx.max(ax + 2.0), seekbar_y + 10.0),
                AB_LOOP_COLOR,
                2.0,
            );
        }
    }

    // Seekbar handle
    let handle_x = seekbar_margin + progress_w;
    let handle_rect = Rect::new(
        handle_x - 5.0,
        seekbar_y - 3.0,
        handle_x + 5.0,
        seekbar_y + 7.0,
    );
    p.fill_rounded_rect(handle_rect, theme.primary, 5.0);

    // Seekbar thumbnail preview on hover
    if let (Some(ratio), Some(thumb)) = (seek_hover_pos, seek_thumbnail) {
        let thumb_w = 160.0;
        let thumb_h = 90.0;
        let preview_x = seekbar_margin + seekbar_w * ratio - thumb_w / 2.0;
        let preview_x = preview_x
            .max(seekbar_margin)
            .min(seekbar_margin + seekbar_w - thumb_w);
        let preview_y = seekbar_y - thumb_h - 12.0;
        let preview_rect = Rect::new(
            preview_x,
            preview_y,
            preview_x + thumb_w,
            preview_y + thumb_h,
        );

        p.fill_rounded_rect(
            Rect::new(
                preview_x + 2.0,
                preview_y + 2.0,
                preview_x + thumb_w + 2.0,
                preview_y + thumb_h + 2.0,
            ),
            Color::rgba8(0, 0, 0, 60),
            4.0,
        );
        p.draw_image(thumb, preview_rect);
        p.stroke_rounded_rect(preview_rect, Color::rgba8(255, 255, 255, 80), 1.0, 4.0);

        let thumb_pos = state.duration * ratio;
        let time_str = controls::format_single_time(thumb_pos);
        p.draw_text(
            &time_str,
            preview_x + thumb_w / 2.0 - 15.0,
            preview_y + thumb_h + 10.0,
            Color::WHITE,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
    }

    // Register seekbar click region
    let seek_click_rect = Rect::new(
        seekbar_margin,
        seekbar_y - 6.0,
        seekbar_margin + seekbar_w,
        seekbar_y + 12.0,
    );
    if state.has_media {
        state.register_click(seek_click_rect, ClickAction::SeekTo(0.0));
    }

    let _ = size;
}

fn paint_buttons_row(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    ctrl_y: f64,
    spacing: f64,
    spacing_large: f64,
) {
    let btn_y = ctrl_y + 24.0;
    let mut bx = spacing_large;

    // Play/Pause
    let play_label = if state.is_playing { "||" } else { ">" };
    let play_rect = Rect::new(bx, btn_y, bx + 36.0, btn_y + 32.0);
    p.fill_rounded_rect(play_rect, theme.primary, theme.border_radius);
    p.draw_text(
        play_label,
        bx + 18.0,
        btn_y + 18.0,
        theme.on_primary,
        16.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(play_rect, ClickAction::PlayPause);
    bx += 48.0;

    // Skip backward (<<)
    let prev_rect = Rect::new(bx, btn_y, bx + 36.0, btn_y + 32.0);
    p.draw_text(
        "<<",
        bx,
        btn_y + 18.0,
        Color::WHITE,
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    state.register_click(prev_rect, ClickAction::SeekRelative(-10.0));
    bx += 36.0;

    // Skip forward (>>)
    let next_rect = Rect::new(bx, btn_y, bx + 36.0, btn_y + 32.0);
    p.draw_text(
        ">>",
        bx,
        btn_y + 18.0,
        Color::WHITE,
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    state.register_click(next_rect, ClickAction::SeekRelative(10.0));
    bx += 48.0;

    // Time display
    let time_text = controls::format_time(state.current_time, state.duration);
    p.draw_text(
        &time_text,
        bx,
        btn_y + 18.0,
        theme.secondary,
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    bx += 140.0;

    // Volume label
    let vol_label = if state.is_muted { "Mute" } else { "Vol" };
    let vol_label_rect = Rect::new(bx, btn_y, bx + 36.0, btn_y + 32.0);
    p.draw_text(
        vol_label,
        bx,
        btn_y + 18.0,
        Color::WHITE,
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    state.register_click(vol_label_rect, ClickAction::ToggleMute);
    bx += 36.0;

    // Volume bar
    let vol_bar_w = 80.0;
    let vol_track = Rect::new(bx, btn_y + 12.0, bx + vol_bar_w, btn_y + 18.0);
    p.fill_rounded_rect(vol_track, SEEKBAR_TRACK, 2.0);
    let vol_fill_w = vol_bar_w * state.volume;
    let vol_fill = Rect::new(bx, btn_y + 12.0, bx + vol_fill_w, btn_y + 18.0);
    p.fill_rounded_rect(vol_fill, theme.primary, 2.0);
    let vol_click_rect = Rect::new(bx, btn_y + 6.0, bx + vol_bar_w, btn_y + 24.0);
    state.register_click(vol_click_rect, ClickAction::VolumeSet(0.0));
    bx += vol_bar_w + spacing;

    // Playback rate
    let rate_text = format!("{}x", state.playback_rate);
    let rate_rect = Rect::new(bx, btn_y + 2.0, bx + 52.0, btn_y + 28.0);
    p.draw_text(
        &rate_text,
        bx,
        btn_y + 18.0,
        theme.secondary,
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    state.register_click(rate_rect, ClickAction::ToggleSpeedMenu);
    state.speed_menu_anchor = (bx, btn_y);
    bx += 52.0;

    // Repeat mode indicator
    let repeat_label = state.repeat_mode.label();
    let repeat_rect = Rect::new(bx, btn_y + 2.0, bx + 40.0, btn_y + 28.0);
    let repeat_active = state.repeat_mode != RepeatMode::None;
    p.fill_rounded_rect(
        repeat_rect,
        if repeat_active {
            theme.primary
        } else {
            BTN_ACTION
        },
        theme.border_radius,
    );
    p.draw_text(
        repeat_label,
        bx + 20.0,
        btn_y + 18.0,
        Color::WHITE,
        10.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(repeat_rect, ClickAction::CycleRepeat);
    bx += 44.0;

    // Shuffle indicator
    let shuffle_label = if state.shuffle_enabled {
        "\u{21C4}on"
    } else {
        "\u{21C4}"
    };
    let shuffle_rect = Rect::new(bx, btn_y + 2.0, bx + 40.0, btn_y + 28.0);
    p.fill_rounded_rect(
        shuffle_rect,
        if state.shuffle_enabled {
            theme.primary
        } else {
            BTN_ACTION
        },
        theme.border_radius,
    );
    p.draw_text(
        shuffle_label,
        bx + 20.0,
        btn_y + 18.0,
        Color::WHITE,
        10.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(shuffle_rect, ClickAction::ToggleShuffle);
    bx += 44.0;

    // Aspect mode indicator
    let aspect_label = state.aspect_mode.label();
    let aspect_rect = Rect::new(bx, btn_y + 2.0, bx + 36.0, btn_y + 28.0);
    p.fill_rounded_rect(aspect_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        aspect_label,
        bx + 18.0,
        btn_y + 18.0,
        Color::WHITE,
        10.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(aspect_rect, ClickAction::CycleAspect);
    bx += 40.0;

    // A-B loop label
    if let Some(label) = controls::ab_loop_label(state.ab_loop) {
        p.draw_text(
            &label,
            bx,
            btn_y + 18.0,
            AB_LOOP_COLOR,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        bx += 120.0;
    }

    // Action buttons: AB, Shot, GIF, Full
    let action_labels = ["AB", "Shot", "GIF", "Full"];
    let action_clicks = [
        ClickAction::ToggleABLoop,
        ClickAction::Screenshot,
        ClickAction::ToggleGifCapture,
        ClickAction::Fullscreen,
    ];
    for (i, label) in action_labels.iter().enumerate() {
        let rect = Rect::new(bx, btn_y + 2.0, bx + 44.0, btn_y + 28.0);
        p.fill_rounded_rect(rect, BTN_ACTION, theme.border_radius);
        p.draw_text(
            label,
            bx + 22.0,
            btn_y + 18.0,
            Color::WHITE,
            10.0,
            FontWeight::BOLD,
            true,
        );
        state.register_click(rect, action_clicks[i].clone());
        bx += 50.0;
    }

    let _ = video_rect;
}

fn paint_speed_menu(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
) {
    if !state.show_speed_menu {
        return;
    }

    let anchor_x = state.speed_menu_anchor.0;
    let anchor_y = state.speed_menu_anchor.1;
    let menu_w = 80.0;
    let menu_h = controls::speed_options().len() as f64 * 20.0 + 12.0;
    let menu = Rect::new(anchor_x, anchor_y - menu_h, anchor_x + menu_w, anchor_y);
    p.fill_rounded_rect(menu, theme.surface, theme.border_radius);
    p.stroke_rounded_rect(menu, theme.border, 1.0, theme.border_radius);
    let mut sy = menu.y0 + 18.0;
    for speed in controls::speed_options() {
        let item_rect = Rect::new(menu.x0 + 4.0, sy - 12.0, menu.x1 - 4.0, sy + 8.0);
        p.draw_text(
            &format!("{speed}x"),
            menu.x0 + 12.0,
            sy,
            if (state.playback_rate - speed).abs() < f64::EPSILON {
                theme.primary
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        state.register_click(item_rect, ClickAction::SetSpeed(speed));
        sy += 20.0;
    }
}
