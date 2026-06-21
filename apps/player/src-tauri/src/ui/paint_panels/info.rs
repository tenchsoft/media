use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::*;
use crate::ui::theme::BTN_ACTION;

#[allow(clippy::too_many_arguments)]
pub(super) fn paint_info_tab(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    size: Size,
    spacing: f64,
    mut dy: f64,
    backend: Option<&crate::gst_backend::PlayerBackend>,
) {
    let info = &state.media_info;
    for (label, value) in [
        ("Title", info.title.as_str()),
        ("Artist", info.artist.as_str()),
        ("Album", info.album.as_str()),
        ("File", info.file_name.as_str()),
        ("Resolution", info.resolution.as_str()),
        ("Video", info.video_codec.as_str()),
        ("Audio", info.audio_codec.as_str()),
        ("Bitrate", info.bitrate.as_str()),
    ] {
        p.draw_text(
            label,
            video_rect.x1 + spacing,
            dy,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            value,
            video_rect.x1 + 104.0,
            dy,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
        dy += 22.0;
    }
    p.draw_text(
        &format!("FPS {:.2}", info.frame_rate),
        video_rect.x1 + spacing,
        dy,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    dy += 26.0;

    // Audio track selection
    if let Some(backend) = backend {
        let n_audio = backend.n_audio_streams();
        if n_audio > 1 {
            p.draw_text(
                "Audio Tracks",
                video_rect.x1 + spacing,
                dy,
                theme.secondary,
                theme.font_size_small,
                FontWeight::BOLD,
                false,
            );
            dy += 20.0;
            let current = backend.current_audio_track();
            for i in 0..n_audio {
                let label = format!(
                    "Track {}{}",
                    i + 1,
                    if i as i32 == current { "  active" } else { "" }
                );
                let rect = Rect::new(
                    video_rect.x1 + spacing,
                    dy - 8.0,
                    size.width - spacing,
                    dy + 16.0,
                );
                p.draw_text(
                    &label,
                    video_rect.x1 + spacing + 8.0,
                    dy,
                    if i as i32 == current {
                        theme.primary
                    } else {
                        theme.on_surface
                    },
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    false,
                );
                state.register_click(rect, ClickAction::SelectAudioTrack(i as i32));
                dy += 22.0;
            }
        }
    } else if state.has_media {
        p.draw_text(
            "Audio Tracks",
            video_rect.x1 + spacing,
            dy,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        dy += 20.0;
        let rect = Rect::new(
            video_rect.x1 + spacing,
            dy - 8.0,
            size.width - spacing,
            dy + 16.0,
        );
        p.draw_text(
            "Track 1  active",
            video_rect.x1 + spacing + 8.0,
            dy,
            theme.primary,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        state.register_click(rect, ClickAction::SelectAudioTrack(0));
        dy += 22.0;
    }

    // Audio device selection
    if !state.audio_devices.is_empty() {
        p.draw_text(
            "Audio Devices",
            video_rect.x1 + spacing,
            dy,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        dy += 20.0;
        let audio_devices = state.audio_devices.clone();
        for (name, _class) in audio_devices {
            let is_selected = state.selected_audio_device.as_deref() == Some(name.as_str());
            let display_name = if name.len() > 30 {
                format!("{}...", &name[..27])
            } else {
                name.clone()
            };
            let label = format!(
                "{}{}",
                display_name,
                if is_selected { "  active" } else { "" }
            );
            let rect = Rect::new(
                video_rect.x1 + spacing,
                dy - 8.0,
                size.width - spacing,
                dy + 16.0,
            );
            p.draw_text(
                &label,
                video_rect.x1 + spacing + 8.0,
                dy,
                if is_selected {
                    theme.primary
                } else {
                    theme.on_surface
                },
                theme.font_size_small,
                FontWeight::MEDIUM,
                false,
            );
            state.register_click(rect, ClickAction::SelectAudioDevice(name.clone()));
            dy += 22.0;
        }
    }

    dy += 8.0;
    let eq_rect = Rect::new(
        video_rect.x1 + spacing,
        dy,
        video_rect.x1 + spacing + 96.0,
        dy + 24.0,
    );
    p.fill_rounded_rect(eq_rect, BTN_ACTION, theme.border_radius);
    p.draw_text(
        "Equalizer",
        eq_rect.x0 + 48.0,
        dy + 15.0,
        Color::WHITE,
        9.0,
        FontWeight::BOLD,
        true,
    );
    state.register_click(eq_rect, ClickAction::ToggleEqualizer);
}
