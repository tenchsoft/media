//! Right panel (inspector) rendering for Composer.

use tench_composer_core::*;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::inspector;
use crate::ui::state::{ClickAction, ComposerState};

pub fn paint_right_panel(
    p: &mut Painter,
    state: &ComposerState,
    size: Size,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
) {
    let right_w = state.right_panel_w;
    let toolbar_h = 48.0;
    let _timeline_h = state.timeline_h;
    let spacing = theme.spacing;

    let ix = size.width - right_w;
    let mut iy = toolbar_h + theme.spacing_large;

    p.draw_text_cached(
        text_cache,
        "INSPECTOR",
        ix + spacing,
        iy,
        theme.secondary,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
        false,
    );
    iy += 20.0;

    // Inspector tabs
    let mut tab_x = ix + spacing;
    for (i, tab) in inspector::TABS.iter().enumerate() {
        let tw = inspector::TAB_W;
        if i == state.active_inspector_tab {
            let tab_rect = inspector::tab_rect(ix, iy, spacing, i);
            p.fill_rounded_rect(tab_rect, theme.primary, theme.border_radius);
            p.draw_text_cached(
                text_cache,
                tab,
                tab_x + 8.0,
                iy,
                theme.on_primary,
                theme.font_size_small,
                FontWeight::MEDIUM,
                false,
                false,
            );
        } else {
            p.draw_text_cached(
                text_cache,
                tab,
                tab_x + 8.0,
                iy,
                theme.secondary,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
                false,
            );
        }
        let tab_rect = inspector::tab_rect(ix, iy, spacing, i);
        register_click(tab_rect, ClickAction::SelectInspectorTab(i));
        tab_x += tw + spacing;
    }
    iy += 24.0;

    // Inspector content
    match state.active_inspector_tab {
        0 => paint_edit_tab(
            p,
            state,
            theme,
            register_click,
            text_cache,
            ix,
            &mut iy,
            spacing,
        ),
        1 => paint_color_tab(p, state, theme, text_cache, ix, &mut iy, spacing),
        2 => paint_audio_tab(
            p,
            state,
            theme,
            register_click,
            text_cache,
            ix,
            &mut iy,
            spacing,
        ),
        3 => paint_deliver_tab(
            p,
            state,
            theme,
            register_click,
            text_cache,
            ix,
            &mut iy,
            spacing,
        ),
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_edit_tab(
    p: &mut Painter,
    state: &ComposerState,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
    ix: f64,
    iy: &mut f64,
    spacing: f64,
) {
    if let Some(clip) = state.selected_clip() {
        p.draw_text_cached(
            text_cache,
            "Clip Properties",
            ix + spacing,
            *iy,
            theme.on_background,
            theme.font_size,
            FontWeight::BOLD,
            false,
            false,
        );
        *iy += 20.0;

        let fields = [
            (
                "Name",
                clip.name.clone(),
                Some(ClickAction::SetClipName(clip.id, format!("{}*", clip.name))),
            ),
            ("Source", clip.source_path.clone(), None),
            ("Timeline In", clip.timeline_in.to_string(), None),
            ("Duration", format!("{} frames", clip.duration), None),
            (
                "Speed",
                format!("{:.1}x", clip.speed),
                Some(ClickAction::SetClipSpeed(clip.id, clip.speed + 0.1)),
            ),
            (
                "Reversed",
                if clip.reversed { "Yes" } else { "No" }.to_string(),
                Some(ClickAction::ToggleClipReversed(clip.id)),
            ),
        ];
        for (label, value, action) in &fields {
            p.draw_text_cached(
                text_cache,
                label,
                ix + spacing,
                *iy,
                theme.secondary,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
                false,
            );
            let val_rect = Rect::new(ix + 100.0, *iy - 8.0, ix + 240.0, *iy + 8.0);
            p.fill_rounded_rect(val_rect, theme.background, theme.border_radius);
            p.draw_text_cached(
                text_cache,
                value,
                ix + 108.0,
                *iy,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
                false,
            );
            if let Some(action) = action {
                register_click(val_rect, action.clone());
            }
            *iy += 24.0;
        }
    } else {
        p.draw_text_cached(
            text_cache,
            inspector::empty_state_message(0),
            ix + spacing,
            *iy,
            theme.disabled,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_audio_tab(
    p: &mut Painter,
    state: &ComposerState,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
    ix: f64,
    iy: &mut f64,
    spacing: f64,
) {
    if let Some(track) = state.selected_track() {
        p.draw_text_cached(
            text_cache,
            "Audio Levels",
            ix + spacing,
            *iy,
            theme.on_background,
            theme.font_size,
            FontWeight::BOLD,
            false,
            false,
        );
        *iy += 20.0;

        // Volume slider visualization
        p.draw_text_cached(
            text_cache,
            &format!("Volume: {:.0}%", track.volume * 100.0),
            ix + spacing,
            *iy,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
        *iy += 18.0;
        let slider_rect = Rect::new(ix + spacing, *iy, ix + 240.0, *iy + 6.0);
        p.fill_rounded_rect(slider_rect, theme.border, 3.0);
        let vol_w = (slider_rect.x1 - slider_rect.x0) * track.volume.clamp(0.0, 2.0) / 2.0;
        p.fill_rounded_rect(
            Rect::new(
                slider_rect.x0,
                slider_rect.y0,
                slider_rect.x0 + vol_w,
                slider_rect.y1,
            ),
            theme.primary,
            3.0,
        );
        register_click(
            Rect::new(
                slider_rect.x0,
                slider_rect.y0 - 8.0,
                slider_rect.x1,
                slider_rect.y1 + 8.0,
            ),
            ClickAction::SetTrackVolume(track.id, (track.volume + 0.1).min(2.0)),
        );
        *iy += 16.0;

        // Pan slider visualization
        p.draw_text_cached(
            text_cache,
            &format!("Pan: {:.1}", track.pan),
            ix + spacing,
            *iy,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
        *iy += 18.0;
        let pan_rect = Rect::new(ix + spacing, *iy, ix + 240.0, *iy + 6.0);
        p.fill_rounded_rect(pan_rect, theme.border, 3.0);
        let pan_center = (pan_rect.x0 + pan_rect.x1) / 2.0;
        let pan_offset = (pan_rect.x1 - pan_rect.x0) / 2.0 * track.pan.clamp(-1.0, 1.0);
        p.fill_rounded_rect(
            Rect::new(
                pan_center,
                pan_rect.y0,
                pan_center + pan_offset,
                pan_rect.y1,
            ),
            Color::rgb8(0x22, 0xC5, 0x5E),
            3.0,
        );
        register_click(
            Rect::new(
                pan_rect.x0,
                pan_rect.y0 - 8.0,
                pan_rect.x1,
                pan_rect.y1 + 8.0,
            ),
            ClickAction::SetTrackPan(track.id, (track.pan + 0.1).clamp(-1.0, 1.0)),
        );
        *iy += 16.0;

        let muted_rect = Rect::new(ix + spacing, *iy - 12.0, ix + 240.0, *iy + 8.0);
        p.draw_text_cached(
            text_cache,
            &format!("Muted: {}", if track.muted { "Yes" } else { "No" }),
            ix + spacing,
            *iy,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
        register_click(muted_rect, ClickAction::ToggleTrackMuted(track.id));
    } else {
        p.draw_text_cached(
            text_cache,
            inspector::empty_state_message(2),
            ix + spacing,
            *iy,
            theme.disabled,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
    }
}

fn paint_color_tab(
    p: &mut Painter,
    state: &ComposerState,
    theme: &Theme,
    text_cache: &mut TextCache,
    ix: f64,
    iy: &mut f64,
    spacing: f64,
) {
    p.draw_text_cached(
        text_cache,
        "Color Grading",
        ix + spacing,
        *iy,
        theme.on_background,
        theme.font_size,
        FontWeight::BOLD,
        false,
        false,
    );
    *iy += 20.0;

    let effects: Vec<_> = state
        .project
        .effects
        .iter()
        .filter(|e| matches!(e.kind, EffectKind::Video(_)))
        .collect();
    if effects.is_empty() {
        p.draw_text_cached(
            text_cache,
            inspector::empty_state_message(1),
            ix + spacing,
            *iy,
            theme.disabled,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
            false,
        );
    } else {
        for effect in effects {
            let label = match &effect.kind {
                EffectKind::Video(t) => t.label(),
                _ => "Unknown",
            };
            p.draw_text_cached(
                text_cache,
                label,
                ix + spacing,
                *iy,
                theme.on_surface,
                theme.font_size,
                FontWeight::NORMAL,
                false,
                false,
            );
            *iy += 20.0;
        }
    }
}

// too_many_arguments: all parameters are needed for the deliver tab painting logic
#[allow(clippy::too_many_arguments)]
fn paint_deliver_tab(
    p: &mut Painter,
    state: &ComposerState,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
    ix: f64,
    iy: &mut f64,
    spacing: f64,
) {
    let settings = &state.project.export_settings;
    p.draw_text_cached(
        text_cache,
        "Export Settings",
        ix + spacing,
        *iy,
        theme.on_background,
        theme.font_size,
        FontWeight::BOLD,
        false,
        false,
    );
    *iy += 20.0;

    let fields = [
        (
            "Format",
            format!(
                "{} ({})",
                settings.format.label(),
                settings.format.extension()
            ),
        ),
        ("Codec", settings.codec.label().to_string()),
        (
            "Resolution",
            format!("{}x{}", settings.width, settings.height),
        ),
        ("FPS", format!("{:.0}", settings.fps)),
        ("Bitrate", format!("{} kbps", settings.bitrate_kbps)),
    ];
    for (idx, (label, value)) in fields.iter().enumerate() {
        p.draw_text_cached(
            text_cache,
            label,
            ix + spacing,
            *iy,
            theme.secondary,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
            false,
        );
        let val_rect = Rect::new(ix + 80.0, *iy - 8.0, ix + 240.0, *iy + 8.0);
        p.fill_rounded_rect(val_rect, theme.background, theme.border_radius);
        p.draw_text_cached(
            text_cache,
            value,
            ix + 88.0,
            *iy,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
            false,
        );
        let action = match idx {
            0 => ClickAction::SetExportFormat(match settings.format {
                ExportFormat::Mp4 => ExportFormat::Mov,
                ExportFormat::Mov => ExportFormat::WebM,
                ExportFormat::WebM => ExportFormat::Avi,
                ExportFormat::Avi => ExportFormat::Mkv,
                ExportFormat::Mkv => ExportFormat::Mp4,
            }),
            1 => ClickAction::SetExportCodec(match settings.codec {
                VideoCodec::H264 => VideoCodec::H265,
                VideoCodec::H265 => VideoCodec::Vp9,
                VideoCodec::Vp9 => VideoCodec::Av1,
                VideoCodec::Av1 => VideoCodec::ProRes,
                VideoCodec::ProRes => VideoCodec::H264,
            }),
            2 => ClickAction::SetExportResolution(1280, 720),
            3 => ClickAction::SetExportFps(if settings.fps >= 60.0 { 24.0 } else { 60.0 }),
            _ => ClickAction::SetExportBitrate(settings.bitrate_kbps.saturating_add(1000)),
        };
        register_click(val_rect, action);
        *iy += 24.0;
    }

    *iy += 8.0;
    let export_rect = Rect::new(
        ix + spacing,
        *iy,
        ix + spacing + 120.0,
        *iy + theme.button_height,
    );
    p.fill_rounded_rect(export_rect, theme.primary, theme.border_radius);
    p.draw_text_cached(
        text_cache,
        "Export",
        ix + spacing + 60.0,
        *iy + 18.0,
        theme.on_primary,
        theme.font_size,
        FontWeight::MEDIUM,
        true,
        false,
    );
    register_click(export_rect, ClickAction::Export);
}
