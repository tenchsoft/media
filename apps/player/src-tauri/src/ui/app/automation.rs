use super::super::state::{ClickAction, PlayerState};
use super::super::video_surface;
use tench_ui::prelude::*;
use tench_ui::{UiAutomationNode, UiAutomationRect};

pub(crate) fn player_automation_nodes(
    state: &PlayerState,
    size: Size,
    gif_recording: bool,
) -> Vec<UiAutomationNode> {
    let mut nodes = Vec::new();
    let mut next_id = 1;

    let controls_h = 64.0;
    let overlay_h = 40.0;
    let side_panel_w = 320.0;
    let video_right = video_surface::video_right(
        size.width,
        state.ai_panel_open,
        state.drawer.is_some(),
        side_panel_w,
    );
    let video_rect = video_surface::video_rect(
        size.width,
        size.height,
        overlay_h,
        controls_h,
        state.ai_panel_open,
        state.drawer.is_some(),
        side_panel_w,
    );

    push_player_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Video surface",
        "player.video.surface",
        video_rect,
        true,
    );

    for region in &state.click_regions {
        for debug_id in action_debug_ids(state, size, gif_recording, &region.action, region.rect) {
            push_player_node(
                &mut nodes,
                &mut next_id,
                action_role(&region.action),
                action_label(&region.action),
                debug_id,
                region.rect,
                true,
            );
        }
    }

    if let Some(menu) = &state.context_menu {
        let menu_w = 220.0;
        let item_h = 28.0;
        let info_section_h = 60.0;
        let menu_h = info_section_h + menu.items.len() as f64 * item_h + 8.0;
        let mx = menu.x.min(size.width - menu_w);
        let my = menu.y.min(size.height - menu_h);
        for (idx, item) in menu.items.iter().enumerate() {
            let debug_id = match item.id.as_str() {
                "play_pause" => "player.context.play_pause",
                "stop" => "player.context.stop",
                "screenshot" => "player.context.screenshot",
                "fullscreen" => "player.context.fullscreen",
                "open_file" => "player.context.open_file",
                "show_in_files" => "player.context.show_in_files",
                "cycle_aspect" => "player.context.aspect",
                "cycle_repeat" => "player.context.repeat",
                "toggle_shuffle" => "player.context.shuffle",
                _ => continue,
            };
            let y0 = my + info_section_h + idx as f64 * item_h;
            push_player_node(
                &mut nodes,
                &mut next_id,
                "menuitem",
                item.label.clone(),
                debug_id,
                Rect::new(mx + 4.0, y0, mx + menu_w - 4.0, y0 + item_h),
                true,
            );
        }
        push_player_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Dismiss context menu",
            "player.context.dismiss",
            Rect::new(0.0, 0.0, 36.0, 36.0),
            true,
        );
        push_player_node(
            &mut nodes,
            &mut next_id,
            "status",
            "Context hover highlight",
            "player.automatic.context_hover",
            Rect::new(mx, my, mx + menu_w, my + menu_h),
            false,
        );
    }

    for (debug_id, label) in [
        ("player.automatic.video_frame", "Video frame render"),
        (
            "player.automatic.playback_progress",
            "Playback position and progress",
        ),
        (
            "player.automatic.subtitle_timing",
            "Subtitle timing overlay",
        ),
        (
            "player.automatic.buffering_progress",
            "Buffering progress bar",
        ),
        (
            "player.automatic.seek_hover_thumbnail",
            "Seek hover thumbnail preview",
        ),
        ("player.automatic.toast_lifecycle", "Toast lifecycle"),
        ("player.automatic.ab_loop", "A-B loop enforcement"),
        ("player.automatic.gapless_next", "Gapless next track"),
        ("player.automatic.audio_visualizer", "Audio only visualizer"),
        ("player.automatic.gif_frame_capture", "GIF frame capture"),
        (
            "player.automatic.gif_recording_indicator",
            "GIF recording indicator",
        ),
        ("player.automatic.side_panel_layout", "Side panel layout"),
        (
            "player.automatic.click_region_refresh",
            "Click region refresh",
        ),
        (
            "player.automatic.empty_state_drop_prompt",
            "Empty state drop prompt",
        ),
        ("player.automatic.media_info_refresh", "Media info refresh"),
    ] {
        push_player_node(
            &mut nodes,
            &mut next_id,
            "status",
            label,
            debug_id,
            Rect::new(0.0, 0.0, video_right, size.height),
            false,
        );
    }

    nodes
}

fn action_debug_ids(
    state: &PlayerState,
    size: Size,
    gif_recording: bool,
    action: &ClickAction,
    rect: Rect,
) -> Vec<String> {
    match action {
        ClickAction::PlayPause => vec!["player.controls.play_pause".into()],
        ClickAction::SeekTo(_) => vec!["player.seekbar.position".into()],
        ClickAction::VolumeSet(_) => vec!["player.controls.volume".into()],
        ClickAction::ToggleMute => vec!["player.controls.mute".into()],
        ClickAction::SetSpeed(speed) => vec![format!("player.speed.{}", speed_debug_id(*speed))],
        ClickAction::ToggleDrawer(tab) => {
            vec![format!(
                "player.top.{}",
                tab.label().to_lowercase().replace(' ', "_")
            )]
        }
        ClickAction::SelectSubtitleTrack(idx) => vec![format!("player.subtitle.external.{idx}")],
        ClickAction::SelectAudioTrack(idx) => vec![format!("player.info.audio_track.{idx}")],
        ClickAction::Screenshot => vec!["player.controls.screenshot".into()],
        ClickAction::ToggleABLoop => vec!["player.controls.ab_loop".into()],
        ClickAction::SeekRelative(delta) if *delta < 0.0 => {
            vec!["player.controls.seek_back_10".into()]
        }
        ClickAction::SeekRelative(_) => vec!["player.controls.seek_forward_10".into()],
        ClickAction::ToggleSpeedMenu => vec!["player.controls.speed_menu".into()],
        ClickAction::ToggleAiPanel => vec!["player.top.ai".into()],
        ClickAction::ToggleGifCapture if state.gif_capture_open && rect.y1 < size.height - 64.0 => {
            vec!["player.gif_modal.close".into()]
        }
        ClickAction::ToggleGifCapture => vec!["player.controls.gif".into()],
        ClickAction::StartGifRecord if gif_recording => vec!["player.gif_modal.stop".into()],
        ClickAction::StartGifRecord => vec!["player.gif_modal.start".into()],
        ClickAction::StopGifRecord => vec!["player.gif_modal.stop".into()],
        ClickAction::Fullscreen => vec!["player.controls.fullscreen".into()],
        ClickAction::CycleAspect => vec!["player.controls.aspect".into()],
        ClickAction::CycleRepeat => vec!["player.controls.repeat".into()],
        ClickAction::ToggleShuffle => vec!["player.controls.shuffle".into()],
        ClickAction::RemoveFromPlaylist(idx) => vec![format!("player.playlist.remove.{idx}")],
        ClickAction::PlayPlaylistItem(idx) => vec![format!("player.playlist.row.{idx}")],
        ClickAction::OpenRecentFile(idx) => vec![format!("player.recent.row.{idx}")],
        ClickAction::JumpToChapter(idx) => vec![format!("player.chapter.row.{idx}")],
        ClickAction::JumpToRememberedPosition => vec!["player.seekbar.remembered".into()],
        ClickAction::AddToPlaylist => vec!["player.playlist.add_files".into()],
        ClickAction::SendAiPrompt(prompt) => {
            vec![format!("player.ai.feature.{}", prompt_id(prompt))]
        }
        ClickAction::CancelAiRequest => vec!["player.ai.cancel".into()],
        ClickAction::SearchSubtitleNext => vec!["player.subtitle_search.next".into()],
        ClickAction::SearchSubtitlePrev => vec!["player.subtitle_search.prev".into()],
        ClickAction::SetSubtitleEncoding(name) => {
            vec![format!("player.subtitle.encoding.{}", encoding_id(name))]
        }
        ClickAction::SubmitUrl => vec!["player.url.play".into()],
        ClickAction::TogglePip => vec!["player.pip.indicator".into()],
        ClickAction::ToggleEqualizer => vec!["player.info.equalizer".into()],
        ClickAction::SetEqBand(idx, value) => {
            let direction = if *value < state.eq_bands.get(*idx).copied().unwrap_or(0.0) {
                "minus"
            } else {
                "plus"
            };
            vec![format!("player.equalizer.band.{idx}.{direction}")]
        }
        ClickAction::DeleteChapter(idx) => vec![format!("player.chapter.delete.{idx}")],
        ClickAction::RenameChapter(idx) => vec![format!("player.chapter.rename.{idx}")],
        ClickAction::ShowAddChapterModal => vec!["player.chapters.add".into()],
        ClickAction::ExportChapters => vec!["player.chapters.export".into()],
        ClickAction::ImportChapters => vec!["player.chapters.import".into()],
        ClickAction::GifOptions => vec!["player.gif_modal.options".into()],
        ClickAction::StartGifRecording => vec!["player.gif_options.start".into()],
        ClickAction::ShowSubtitleSearch => vec!["player.subtitles.search".into()],
        ClickAction::ShowSubtitleStyle => vec!["player.subtitles.style".into()],
        ClickAction::FocusUrlInput => vec!["player.url.input".into()],
        ClickAction::FocusSubtitleSearch => vec!["player.subtitle_search.input".into()],
        ClickAction::FocusChapterNameInput => vec!["player.add_chapter.input".into()],
        ClickAction::ConfirmAddChapter => vec!["player.add_chapter.add".into()],
        ClickAction::CloseModal => close_modal_debug_ids(state),
        ClickAction::SubtitleOffsetForTrack(idx, delta) if *delta < 0 => {
            vec![format!("player.subtitle.external.{idx}.offset_minus")]
        }
        ClickAction::SubtitleOffsetForTrack(idx, _) => {
            vec![format!("player.subtitle.external.{idx}.offset_plus")]
        }
        ClickAction::SelectBuiltinSubtitleTrack(idx) if *idx < 0 => {
            vec!["player.subtitle.builtin.none".into()]
        }
        ClickAction::SelectBuiltinSubtitleTrack(idx) => {
            vec![format!("player.subtitle.builtin.{idx}")]
        }
        ClickAction::SelectAudioDevice(name) => {
            vec![format!("player.info.audio_device.{}", simple_id(name))]
        }
        ClickAction::SetEqPresetNamed(name) => {
            vec![format!("player.equalizer.preset.{}", simple_id(name))]
        }
        ClickAction::FocusAiInput => vec!["player.ai.input".into()],
        ClickAction::AdjustSubtitleStyle(prop_idx, delta) => {
            let direction = subtitle_style_direction(size, rect, *delta);
            vec![format!(
                "player.subtitle_style.{}.{}",
                subtitle_style_prop(*prop_idx),
                direction
            )]
        }
        _ => Vec::new(),
    }
}

fn close_modal_debug_ids(state: &PlayerState) -> Vec<String> {
    let mut ids = Vec::new();
    if state.help_open {
        ids.push("player.help.close".into());
    }
    if state.url_input_open {
        ids.push("player.url.cancel".into());
    }
    if state.subtitle_style_open {
        ids.push("player.subtitle_style.close".into());
    }
    if state.subtitle_search_open {
        ids.push("player.subtitle_search.close".into());
    }
    if state.gif_options_open {
        ids.push("player.gif_options.cancel".into());
    }
    if state.eq_open {
        ids.push("player.equalizer.close".into());
    }
    if state.show_add_chapter_modal {
        ids.push("player.add_chapter.cancel".into());
    }
    ids
}

fn speed_debug_id(speed: f64) -> &'static str {
    if (speed - 0.25).abs() < 0.01 {
        "0_25x"
    } else if (speed - 0.5).abs() < 0.01 {
        "0_5x"
    } else if (speed - 0.75).abs() < 0.01 {
        "0_75x"
    } else if (speed - 1.0).abs() < 0.01 {
        "1x"
    } else if (speed - 1.25).abs() < 0.01 {
        "1_25x"
    } else if (speed - 1.5).abs() < 0.01 {
        "1_5x"
    } else if (speed - 1.75).abs() < 0.01 {
        "1_75x"
    } else if (speed - 2.0).abs() < 0.01 {
        "2x"
    } else if (speed - 3.0).abs() < 0.01 {
        "3x"
    } else {
        "4x"
    }
}

fn prompt_id(prompt: &str) -> String {
    simple_id(prompt)
}

fn encoding_id(name: &str) -> String {
    match name {
        "Auto" => "auto".into(),
        "UTF-8" => "utf_8".into(),
        "Shift-JIS" => "shift_jis".into(),
        "EUC-KR" => "euc_kr".into(),
        "CP1252" => "cp1252".into(),
        _ => simple_id(name),
    }
}

fn subtitle_style_prop(prop_idx: usize) -> &'static str {
    match prop_idx {
        0 => "font_size",
        1 => "font_family",
        2 => "text_color",
        3 => "background_opacity",
        4 => "position",
        5 => "stroke_width",
        6 => "shadow_offset",
        _ => "unknown",
    }
}

fn subtitle_style_direction(size: Size, rect: Rect, delta: f32) -> &'static str {
    if delta < 0.0 || (delta == 0.0 && rect.x1 < size.width / 2.0 + 140.0) {
        "minus"
    } else {
        "plus"
    }
}

fn simple_id(value: &str) -> String {
    value
        .to_lowercase()
        .replace('&', "and")
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn action_label(action: &ClickAction) -> String {
    format!("{action:?}")
}

fn action_role(action: &ClickAction) -> &'static str {
    match action {
        ClickAction::SeekTo(_)
        | ClickAction::VolumeSet(_)
        | ClickAction::SetEqBand(_, _)
        | ClickAction::AdjustSubtitleStyle(_, _) => "slider",
        ClickAction::FocusAiInput
        | ClickAction::FocusUrlInput
        | ClickAction::FocusSubtitleSearch
        | ClickAction::FocusChapterNameInput => "textbox",
        ClickAction::SetSpeed(_)
        | ClickAction::SetSubtitleEncoding(_)
        | ClickAction::SelectBuiltinSubtitleTrack(_)
        | ClickAction::SelectSubtitleTrack(_)
        | ClickAction::SelectAudioTrack(_)
        | ClickAction::SelectAudioDevice(_) => "option",
        _ => "button",
    }
}

fn push_player_node(
    nodes: &mut Vec<UiAutomationNode>,
    next_id: &mut u64,
    role: &str,
    label: impl Into<String>,
    debug_id: impl Into<String>,
    rect: Rect,
    enabled: bool,
) {
    nodes.push(UiAutomationNode {
        id: *next_id,
        debug_id: Some(debug_id.into()),
        role: role.to_string(),
        label: Some(label.into()),
        value: None,
        bounds: UiAutomationRect {
            x: rect.x0,
            y: rect.y0,
            width: rect.width(),
            height: rect.height(),
        },
        enabled,
        focused: false,
        hovered: false,
        children: Vec::new(),
    });
    *next_id = next_id.saturating_add(1);
}
