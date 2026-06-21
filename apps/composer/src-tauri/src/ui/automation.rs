use super::state::{ClickAction, ComposerState};
use super::{timeline, ClickRegion};
use tench_composer_core::{TransitionType, VideoEffectType};
use tench_ui::prelude::*;
use tench_ui::{UiAutomationNode, UiAutomationRect};

pub(crate) fn composer_automation_nodes(
    state: &ComposerState,
    regions: &[ClickRegion],
    size: Size,
    base_id: u64,
) -> Vec<UiAutomationNode> {
    let mut nodes = Vec::new();
    let mut next_id = base_id.saturating_mul(1000).saturating_add(1);

    for region in regions {
        for debug_id in action_debug_ids(&region.action) {
            push_composer_node(
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

    let left_w = state.left_panel_w;
    let right_w = state.right_panel_w;
    let timeline_h = state.timeline_h;
    let tl_y = size.height - timeline_h;
    let center_right = size.width - right_w;

    push_composer_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Preview play pause",
        "composer.preview.play_pause",
        Rect::new(left_w, 48.0, center_right, tl_y),
        true,
    );
    push_composer_node(
        &mut nodes,
        &mut next_id,
        "button",
        "Timeline seek",
        "composer.timeline.seek",
        Rect::new(left_w, tl_y, size.width, size.height),
        true,
    );
    push_composer_node(
        &mut nodes,
        &mut next_id,
        "separator",
        "Left splitter",
        "composer.splitter.left",
        Rect::new(left_w - 2.0, 48.0, left_w + 2.0, tl_y),
        true,
    );
    push_composer_node(
        &mut nodes,
        &mut next_id,
        "separator",
        "Right splitter",
        "composer.splitter.right",
        Rect::new(center_right - 2.0, 48.0, center_right + 2.0, tl_y),
        true,
    );
    push_composer_node(
        &mut nodes,
        &mut next_id,
        "separator",
        "Timeline splitter",
        "composer.splitter.timeline",
        Rect::new(0.0, tl_y - 2.0, size.width, tl_y + 2.0),
        true,
    );

    for (clip_index, (track_index, clip, rect)) in
        timeline_clip_nodes(state, size).into_iter().enumerate()
    {
        push_composer_node(
            &mut nodes,
            &mut next_id,
            "button",
            clip.name.clone(),
            format!("composer.timeline.clip.{clip_index}"),
            rect,
            true,
        );
        push_composer_node(
            &mut nodes,
            &mut next_id,
            "button",
            clip.name,
            format!("composer.timeline.clip_id.{}", clip.id.0),
            rect,
            true,
        );
        if state.selected_clip_id == Some(clip.id) {
            push_composer_node(
                &mut nodes,
                &mut next_id,
                "status",
                "Selected clip",
                "composer.timeline.clip.selected",
                rect,
                false,
            );
        }
        let handle_w = timeline::TRIM_HANDLE_W;
        push_composer_node(
            &mut nodes,
            &mut next_id,
            "slider",
            "Trim in",
            format!("composer.timeline.clip.{clip_index}.trim_in"),
            Rect::new(rect.x0, rect.y0, rect.x0 + handle_w, rect.y1),
            true,
        );
        push_composer_node(
            &mut nodes,
            &mut next_id,
            "slider",
            "Trim out",
            format!("composer.timeline.clip.{clip_index}.trim_out"),
            Rect::new(rect.x1 - handle_w, rect.y0, rect.x1, rect.y1),
            true,
        );
        push_composer_node(
            &mut nodes,
            &mut next_id,
            "row",
            "Track lane",
            format!("composer.timeline.track_lane.{track_index}"),
            Rect::new(left_w, rect.y0, size.width, rect.y1),
            false,
        );
    }

    for (idx, _asset) in state.media_bin().iter().enumerate() {
        push_composer_node(
            &mut nodes,
            &mut next_id,
            "button",
            "Media asset",
            format!("composer.media.asset.{idx}"),
            Rect::new(
                8.0,
                100.0 + idx as f64 * 32.0,
                left_w - 8.0,
                132.0 + idx as f64 * 32.0,
            ),
            true,
        );
    }

    if let Some(menu) = &state.context_menu {
        for (idx, item) in menu.items.iter().enumerate() {
            push_composer_node(
                &mut nodes,
                &mut next_id,
                "menuitem",
                item.label.clone(),
                format!(
                    "composer.clip.context.{}",
                    item.label.to_lowercase().replace(' ', "_")
                ),
                Rect::new(
                    menu.x,
                    menu.y + idx as f64 * 24.0,
                    menu.x + 160.0,
                    menu.y + (idx + 1) as f64 * 24.0,
                ),
                item.enabled,
            );
        }
    }

    for (debug_id, label) in [
        ("composer.automatic.notice", "Notice lifecycle"),
        ("composer.automatic.auto_save", "Project auto save"),
        ("composer.automatic.playback", "Playback advance"),
        ("composer.automatic.timecode", "Preview timecode"),
        ("composer.automatic.ruler", "Timeline ruler"),
        ("composer.automatic.playhead", "Timeline playhead"),
        ("composer.automatic.track_lanes", "Track lane layout"),
        ("composer.automatic.empty_state", "Empty state rendering"),
        ("composer.automatic.metadata", "Media metadata"),
        ("composer.automatic.panel_layout", "Panel overlay layout"),
    ] {
        push_composer_node(
            &mut nodes,
            &mut next_id,
            "status",
            label,
            debug_id,
            Rect::new(0.0, 0.0, size.width, size.height),
            false,
        );
    }

    nodes
}

fn timeline_clip_nodes(
    state: &ComposerState,
    size: Size,
) -> Vec<(usize, tench_composer_core::Clip, Rect)> {
    let left_w = state.left_panel_w;
    let timeline_h = state.timeline_h;
    let tl_y = size.height - timeline_h;
    let tl_content_w = timeline::content_width(size.width, left_w);
    let total_frames = state.total_frames();
    let track_h = timeline::track_height(timeline_h, state.tracks().len());
    let mut nodes = Vec::new();
    for (track_index, track) in state.tracks().iter().enumerate() {
        let track_y =
            tl_y + timeline::HEADER_H + timeline::TOOLBAR_H + track_index as f64 * track_h;
        for clip in &track.clips {
            let rect = timeline::clip_rect(
                left_w,
                tl_content_w,
                total_frames,
                track_y,
                track_h,
                clip.timeline_in,
                clip.timeline_out(),
            );
            nodes.push((track_index, clip.clone(), rect));
        }
    }
    nodes
}

fn action_debug_ids(action: &ClickAction) -> Vec<String> {
    match action {
        ClickAction::SelectMode(mode) => {
            vec![format!("composer.mode.{}", mode.label().to_lowercase())]
        }
        ClickAction::ImportMedia => vec!["composer.toolbar.import".into()],
        ClickAction::SplitAtPlayhead => vec!["composer.toolbar.split".into()],
        ClickAction::DeleteClip(_) => vec!["composer.toolbar.delete".into()],
        ClickAction::Export => vec![
            "composer.toolbar.export".into(),
            "composer.deliver.export".into(),
            "composer.export.queue".into(),
        ],
        ClickAction::SelectLeftTab(tab) => vec![format!(
            "composer.left.{}",
            tab.label().to_lowercase().replace("trans.", "transitions")
        )],
        ClickAction::SelectInspectorTab(index) => {
            let name = ["edit", "color", "audio", "deliver"]
                .get(*index)
                .copied()
                .unwrap_or("unknown");
            vec![format!("composer.inspector.{name}")]
        }
        ClickAction::SelectTemplate(index) => vec![format!("composer.template.{index}")],
        ClickAction::SelectClip(Some(id)) => vec![format!("composer.timeline.clip_id.{}", id.0)],
        ClickAction::SelectClip(None) => vec!["composer.media.asset".into()],
        ClickAction::FocusEffectsSearch => vec!["composer.effects.search".into()],
        ClickAction::FocusTransitionsSearch => vec!["composer.transitions.search".into()],
        ClickAction::ApplyEffect(effect) => {
            let mut ids = vec![format!(
                "composer.effect.{}",
                effect.label().to_lowercase().replace(' ', "_")
            )];
            if *effect == VideoEffectType::ColorBalance {
                ids.push("composer.effect.color_correction".into());
            }
            ids
        }
        ClickAction::ApplyTransition(transition) => {
            let label = match transition {
                TransitionType::CrossDissolve => "dissolve",
                TransitionType::FadeIn | TransitionType::FadeOut => "fade",
                TransitionType::WipeLeft | TransitionType::WipeRight => "wipe",
                TransitionType::SlideLeft | TransitionType::SlideRight => "slide",
                _ => return Vec::new(),
            };
            vec![format!("composer.transition.{label}")]
        }
        ClickAction::RunAiFeature(name) => {
            let normalized = name.to_lowercase().replace(['-', ' '], "_");
            match normalized.as_str() {
                "effect_rotate" => vec!["composer.effect.rotate".into()],
                "transition_zoom" => vec!["composer.transition.zoom".into()],
                _ => vec![format!("composer.ai.feature.{normalized}")],
            }
        }
        ClickAction::ToggleRenderQueue => vec!["composer.quick.queue".into()],
        ClickAction::ToggleAiPanel => vec!["composer.quick.ai".into()],
        ClickAction::CloseRenderQueue => vec!["composer.render_queue.close".into()],
        ClickAction::CancelRenderJob(_) => vec!["composer.render_job.cancel".into()],
        ClickAction::PauseRenderJob(_) => vec!["composer.render_job.pause".into()],
        ClickAction::ToggleSnap => vec!["composer.timeline.snap".into()],
        ClickAction::ToggleRipple => vec!["composer.timeline.ripple".into()],
        ClickAction::ToggleMagnet => vec!["composer.timeline.magnet".into()],
        ClickAction::ZoomIn => vec!["composer.timeline.zoom_in".into()],
        ClickAction::ZoomOut => vec!["composer.timeline.zoom_out".into()],
        ClickAction::AddTrack(_) => vec!["composer.timeline.add_track".into()],
        ClickAction::ToggleTrackMute(_) => vec!["composer.track.mute".into()],
        ClickAction::ToggleTrackLock(_) => vec!["composer.track.lock".into()],
        ClickAction::ToggleTrackHidden(_) => vec!["composer.track.hidden".into()],
        ClickAction::SetClipName(_, _) => vec!["composer.clip.name".into()],
        ClickAction::SetClipSpeed(_, _) => vec!["composer.clip.speed".into()],
        ClickAction::ToggleClipReversed(_) => vec!["composer.clip.reversed".into()],
        ClickAction::SetTrackVolume(_, _) => vec!["composer.track.volume".into()],
        ClickAction::SetTrackPan(_, _) => vec!["composer.track.pan".into()],
        ClickAction::ToggleTrackMuted(_) => vec!["composer.track.muted".into()],
        ClickAction::SetExportFormat(_) => vec!["composer.export.format".into()],
        ClickAction::SetExportCodec(_) => vec!["composer.export.codec".into()],
        ClickAction::SetExportResolution(_, _) => vec!["composer.export.resolution".into()],
        ClickAction::SetExportFps(_) => vec!["composer.export.fps".into()],
        ClickAction::SetExportBitrate(_) => vec!["composer.export.bitrate".into()],
        ClickAction::FocusSubtitleEditor => vec!["composer.subtitle.editor".into()],
        _ => Vec::new(),
    }
}

fn action_label(action: &ClickAction) -> String {
    format!("{action:?}")
}

fn action_role(action: &ClickAction) -> &'static str {
    match action {
        ClickAction::FocusSubtitleEditor
        | ClickAction::FocusEffectsSearch
        | ClickAction::FocusTransitionsSearch => "textbox",
        ClickAction::SetTrackVolume(_, _)
        | ClickAction::SetTrackPan(_, _)
        | ClickAction::SetClipSpeed(_, _)
        | ClickAction::SetExportBitrate(_) => "slider",
        _ => "button",
    }
}

fn push_composer_node(
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
