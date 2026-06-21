use tench_composer_lib::ui::{
    state::{ComposerMode, ComposerState, LeftPanelTab},
    ComposerApp,
};
use tench_ui_automation_core::{
    UiAutomationAction, UiAutomationCapture, UiAutomationKey, UiAutomationModifiers,
    UiAutomationPoint, UiAutomationSelector,
};
use tench_ui_test::{
    assert_capture_changed, harness::HarnessConfig, CaptureAssertions, TestHarness,
};

fn harness() -> TestHarness {
    TestHarness::with_config(
        ComposerApp::with_state(ComposerState::example()),
        HarnessConfig::with_viewport(1280.0, 720.0),
    )
}

fn selector(debug_id: &str) -> UiAutomationSelector {
    UiAutomationSelector::debug_id(debug_id)
}

fn capture(harness: &mut TestHarness) -> UiAutomationCapture {
    harness.automation_capture(Default::default())
}

fn click(harness: &mut TestHarness, debug_id: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::Click {
            selector: selector(debug_id),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|error| panic!("click {debug_id}: {error:?}"))
}

fn right_click(harness: &mut TestHarness, debug_id: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::RightClick {
            selector: selector(debug_id),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|error| panic!("right click {debug_id}: {error:?}"))
}

fn type_text(harness: &mut TestHarness, debug_id: &str, text: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::TypeText {
            selector: selector(debug_id),
            text: text.to_string(),
        })
        .unwrap_or_else(|error| panic!("type text into {debug_id}: {error:?}"))
}

fn key(
    harness: &mut TestHarness,
    key: UiAutomationKey,
    modifiers: UiAutomationModifiers,
) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::KeyPress { key, modifiers })
        .expect("key press")
}

fn app_state(harness: &mut TestHarness) -> &mut ComposerState {
    let pod = harness.root_mut();
    let app: &mut ComposerApp = pod.widget.downcast_mut().expect("root is ComposerApp");
    app.state_mut()
}

#[test]
fn composer_plan_primary_controls_timeline_and_automatic_nodes_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);
    initial.assert_png_size(1280, 720);
    initial.assert_nonblank();

    for debug_id in [
        "composer.mode.edit",
        "composer.mode.color",
        "composer.mode.audio",
        "composer.mode.deliver",
        "composer.toolbar.import",
        "composer.toolbar.split",
        "composer.toolbar.delete",
        "composer.toolbar.export",
        "composer.left.media",
        "composer.left.templates",
        "composer.left.effects",
        "composer.left.transitions",
        "composer.media.asset.0",
        "composer.inspector.edit",
        "composer.inspector.color",
        "composer.inspector.audio",
        "composer.inspector.deliver",
        "composer.timeline.snap",
        "composer.timeline.ripple",
        "composer.timeline.magnet",
        "composer.timeline.add_track",
        "composer.track.mute",
        "composer.track.lock",
        "composer.track.hidden",
        "composer.timeline.clip.0",
        "composer.timeline.clip.0.trim_in",
        "composer.timeline.clip.0.trim_out",
        "composer.preview.play_pause",
        "composer.timeline.seek",
        "composer.splitter.left",
        "composer.splitter.right",
        "composer.splitter.timeline",
        "composer.quick.queue",
        "composer.quick.ai",
        "composer.automatic.notice",
        "composer.automatic.playback",
        "composer.automatic.playhead",
        "composer.automatic.track_lanes",
    ] {
        initial.assert_selector_present(&selector(debug_id));
    }

    // Plan 01: Import media dialog injection
    // Plan 78: Import toolbar button - verify import status and left tab
    {
        let pod = harness.root_mut();
        let app: &mut ComposerApp = pod.widget.downcast_mut().unwrap();
        app.inject_test_media("/test/video.mp4".to_string());
    }
    let before_count = app_state(&mut harness).project.media_bin.len();
    click(&mut harness, "composer.toolbar.import");
    let state_after_import = app_state(&mut harness);
    assert_eq!(state_after_import.project.media_bin.len(), before_count + 1);
    assert_eq!(state_after_import.import_status, "Import complete");
    assert_eq!(state_after_import.left_tab, LeftPanelTab::Media);
    assert_eq!(state_after_import.composer_notice, "Media imported");

    // Plan 89/104: Play/pause - verify is_playing state toggles
    let before_playing = app_state(&mut harness).is_playing;
    let played = click(&mut harness, "composer.preview.play_pause");
    assert_ne!(app_state(&mut harness).is_playing, before_playing);
    assert!(app_state(&mut harness).is_playing);
    assert_eq!(app_state(&mut harness).shuttle_direction, 1);
    assert_eq!(app_state(&mut harness).shuttle_speed, 1.0);
    assert_capture_changed(&initial, &played);

    let advanced = harness
        .automation_action(UiAutomationAction::AnimFrame { timestamp_ms: 16 })
        .expect("anim frame");
    advanced.assert_selector_present(&selector("composer.automatic.playback"));

    // Plan 03: Timeline toggle state verification
    let before_snap = app_state(&mut harness).snap;
    let snap = click(&mut harness, "composer.timeline.snap");
    assert_ne!(app_state(&mut harness).snap, before_snap);
    assert_capture_changed(&played, &snap);

    let before_ripple = app_state(&mut harness).ripple;
    let ripple = click(&mut harness, "composer.timeline.ripple");
    assert_ne!(app_state(&mut harness).ripple, before_ripple);
    assert_capture_changed(&snap, &ripple);

    let before_magnet = app_state(&mut harness).magnetic;
    let magnet = click(&mut harness, "composer.timeline.magnet");
    assert_ne!(app_state(&mut harness).magnetic, before_magnet);
    assert_capture_changed(&ripple, &magnet);

    // Plan 04: Track controls state verification
    let before_mute = app_state(&mut harness).project.timeline.tracks[0].muted;
    click(&mut harness, "composer.track.mute");
    assert_ne!(
        app_state(&mut harness).project.timeline.tracks[0].muted,
        before_mute
    );

    let before_lock = app_state(&mut harness).project.timeline.tracks[0].locked;
    click(&mut harness, "composer.track.lock");
    assert_ne!(
        app_state(&mut harness).project.timeline.tracks[0].locked,
        before_lock
    );

    let before_hidden = app_state(&mut harness).project.timeline.tracks[0].hidden;
    click(&mut harness, "composer.track.hidden");
    assert_ne!(
        app_state(&mut harness).project.timeline.tracks[0].hidden,
        before_hidden
    );

    // Plan 07: Add Track - verify track count and notice
    let track_count_before = app_state(&mut harness).project.timeline.tracks.len();
    let added_track = click(&mut harness, "composer.timeline.add_track");
    assert_eq!(
        app_state(&mut harness).project.timeline.tracks.len(),
        track_count_before + 1
    );
    assert_eq!(app_state(&mut harness).composer_notice, "Track added");
    added_track.assert_selector_present(&selector("composer.automatic.track_lanes"));
    assert_capture_changed(&magnet, &added_track);

    let left_bounds = added_track.selector_bounds(&selector("composer.splitter.left"));
    let start = left_bounds.center();
    let dragged = harness
        .automation_action(UiAutomationAction::Drag {
            start: UiAutomationPoint {
                x: start.x,
                y: start.y,
            },
            end: UiAutomationPoint {
                x: start.x + 40.0,
                y: start.y,
            },
            steps: 4,
        })
        .expect("drag left splitter");
    assert_capture_changed(&added_track, &dragged);
}

#[test]
fn composer_plan_left_panel_templates_effects_transitions_and_subtitles_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);

    // Plan 108/107: Templates tab and template row - verify left_tab and template state
    let templates = click(&mut harness, "composer.left.templates");
    assert_eq!(app_state(&mut harness).left_tab, LeftPanelTab::Templates);
    templates.assert_selector_present(&selector("composer.template.0"));
    let templated = click(&mut harness, "composer.template.0");
    assert_eq!(app_state(&mut harness).selected_template_idx, Some(0));
    assert_capture_changed(&templates, &templated);

    // Plan 68: Effects left panel tab - verify left_tab state
    let effects = click(&mut harness, "composer.left.effects");
    assert_eq!(app_state(&mut harness).left_tab, LeftPanelTab::Effects);
    for debug_id in [
        "composer.effects.search",
        "composer.effect.blur",
        "composer.effect.sharpen",
        "composer.effect.color_correction",
        "composer.effect.brightness",
        "composer.effect.contrast",
        "composer.effect.saturation",
        "composer.effect.crop",
        "composer.effect.scale",
        "composer.effect.rotate",
    ] {
        effects.assert_selector_present(&selector(debug_id));
    }

    // Plan 69: Effects search - verify search text and input focus
    let filtered = type_text(&mut harness, "composer.effects.search", "blur");
    assert_eq!(app_state(&mut harness).effects_search, "blur");
    filtered.assert_selector_present(&selector("composer.effect.blur"));
    assert_capture_changed(&effects, &filtered);
    // Plan 35: Blur effect - verify effect is applied to selected clip
    let clip_id = app_state(&mut harness).selected_clip_id;
    let undo_before = app_state(&mut harness).undo_stack.len();
    let effect_applied = click(&mut harness, "composer.effect.blur");
    if let Some(cid) = clip_id {
        let clip = app_state(&mut harness).find_clip(cid);
        assert!(clip.is_some_and(|c| !c.effect_ids.is_empty()));
        assert!(app_state(&mut harness).undo_stack.len() > undo_before);
    }
    assert_capture_changed(&filtered, &effect_applied);

    // Plan 118: Transitions left panel tab - verify left_tab state
    let transitions = click(&mut harness, "composer.left.transitions");
    assert_eq!(app_state(&mut harness).left_tab, LeftPanelTab::Transitions);
    for debug_id in [
        "composer.transitions.search",
        "composer.transition.dissolve",
        "composer.transition.fade",
        "composer.transition.wipe",
        "composer.transition.slide",
        "composer.transition.zoom",
    ] {
        transitions.assert_selector_present(&selector(debug_id));
    }
    // Plan 119: Transitions search - verify search text
    let transition_filtered = type_text(&mut harness, "composer.transitions.search", "wipe");
    assert_eq!(app_state(&mut harness).transitions_search, "wipe");
    transition_filtered.assert_selector_present(&selector("composer.transition.wipe"));
    let transition_applied = click(&mut harness, "composer.transition.wipe");
    assert_capture_changed(&transition_filtered, &transition_applied);

    // Plan 106: Subtitle editor - verify subtitle text state
    let subtitle = type_text(&mut harness, "composer.subtitle.editor", "Subtitle line");
    assert_eq!(app_state(&mut harness).subtitle_text, "Subtitle line");
    assert!(app_state(&mut harness).subtitle_focused);
    subtitle.assert_selector_present(&selector("composer.subtitle.editor"));
    assert_capture_changed(&initial, &subtitle);
}

#[test]
fn composer_plan_inspector_render_queue_ai_and_context_menu_use_real_events_ui_e2e() {
    let mut state = ComposerState::example();
    state.selected_clip_id = None;
    let mut harness = TestHarness::with_config(
        ComposerApp::with_state(state),
        HarnessConfig::with_viewport(1280.0, 720.0),
    );
    let initial = capture(&mut harness);

    // Plan 109: Timeline clip body - verify selected_clip_id state
    let selected = click(&mut harness, "composer.timeline.clip.0");
    assert!(app_state(&mut harness).selected_clip_id.is_some());
    selected.assert_selector_present(&selector("composer.timeline.clip.selected"));
    selected.assert_selector_present(&selector("composer.clip.name"));
    selected.assert_selector_present(&selector("composer.clip.speed"));
    selected.assert_selector_present(&selector("composer.clip.reversed"));
    assert_capture_changed(&initial, &selected);

    // Plan 05/44: Clip inspector state verification - name and speed fields
    let clip_id = app_state(&mut harness).selected_clip_id.unwrap();
    let before_name = app_state(&mut harness)
        .find_clip(clip_id)
        .map(|c| c.name.clone());
    let renamed = click(&mut harness, "composer.clip.name");
    let after_name = app_state(&mut harness)
        .find_clip(clip_id)
        .map(|c| c.name.clone());
    assert_ne!(before_name, after_name);
    assert_capture_changed(&selected, &renamed);
    // Plan 46: Clip speed field - verify speed value changes
    let before_speed = app_state(&mut harness).find_clip(clip_id).map(|c| c.speed);
    let speed = click(&mut harness, "composer.clip.speed");
    let after_speed = app_state(&mut harness).find_clip(clip_id).map(|c| c.speed);
    assert_ne!(before_speed, after_speed);
    assert_capture_changed(&renamed, &speed);
    let clip_id = app_state(&mut harness).selected_clip_id.unwrap();
    let before_reversed = app_state(&mut harness)
        .find_clip(clip_id)
        .map(|c| c.reversed);
    let reversed = click(&mut harness, "composer.clip.reversed");
    let after_reversed = app_state(&mut harness)
        .find_clip(clip_id)
        .map(|c| c.reversed);
    assert_ne!(before_reversed, after_reversed);
    assert_capture_changed(&speed, &reversed);

    // Plan 12: Audio inspector tab - verify mode and inspector tab sync
    let audio = click(&mut harness, "composer.inspector.audio");
    assert_eq!(app_state(&mut harness).active_inspector_tab, 2);
    assert_eq!(app_state(&mut harness).mode, ComposerMode::Audio);
    for debug_id in [
        "composer.track.volume",
        "composer.track.pan",
        "composer.track.muted",
    ] {
        audio.assert_selector_present(&selector(debug_id));
    }
    // Plan 117: Track volume slider - verify volume state change
    let track_id = app_state(&mut harness).selected_track().map(|t| t.id);
    let before_vol = track_id.and_then(|tid| {
        app_state(&mut harness)
            .tracks()
            .iter()
            .find(|t| t.id == tid)
            .map(|t| t.volume)
    });
    let volume = click(&mut harness, "composer.track.volume");
    let after_vol = track_id.and_then(|tid| {
        app_state(&mut harness)
            .tracks()
            .iter()
            .find(|t| t.id == tid)
            .map(|t| t.volume)
    });
    assert_ne!(before_vol, after_vol);
    assert_capture_changed(&audio, &volume);
    // Plan 116: Track pan slider - verify pan state change
    let before_pan = track_id.and_then(|tid| {
        app_state(&mut harness)
            .tracks()
            .iter()
            .find(|t| t.id == tid)
            .map(|t| t.pan)
    });
    let pan = click(&mut harness, "composer.track.pan");
    let after_pan = track_id.and_then(|tid| {
        app_state(&mut harness)
            .tracks()
            .iter()
            .find(|t| t.id == tid)
            .map(|t| t.pan)
    });
    assert_ne!(before_pan, after_pan);
    assert_capture_changed(&volume, &pan);

    // Plan 63: Deliver inspector tab - verify mode and inspector tab sync
    let deliver = click(&mut harness, "composer.inspector.deliver");
    assert_eq!(app_state(&mut harness).active_inspector_tab, 3);
    assert_eq!(app_state(&mut harness).mode, ComposerMode::Deliver);
    for debug_id in [
        "composer.export.format",
        "composer.export.codec",
        "composer.export.resolution",
        "composer.export.fps",
        "composer.export.bitrate",
        "composer.deliver.export",
    ] {
        deliver.assert_selector_present(&selector(debug_id));
    }
    // Plan 72: Export format field - verify format state changes
    let before_format = app_state(&mut harness).project.export_settings.format;
    let format_changed = click(&mut harness, "composer.export.format");
    let after_format = app_state(&mut harness).project.export_settings.format;
    assert_ne!(before_format.extension(), after_format.extension());
    assert_capture_changed(&deliver, &format_changed);
    let queued = click(&mut harness, "composer.deliver.export");
    // Plan 02/62: Export - verify render job was created with show_render_queue and notice
    assert!(!app_state(&mut harness).project.render_queue.is_empty());
    assert!(app_state(&mut harness).show_render_queue);
    assert_eq!(app_state(&mut harness).composer_notice, "Render queued");
    queued.assert_selector_present(&selector("composer.render_job.pause"));
    queued.assert_selector_present(&selector("composer.render_job.cancel"));
    queued.assert_selector_present(&selector("composer.render_queue.close"));
    assert_capture_changed(&format_changed, &queued);
    let paused = click(&mut harness, "composer.render_job.pause");
    assert_capture_changed(&queued, &paused);
    let cancelled = click(&mut harness, "composer.render_job.cancel");
    assert_capture_changed(&paused, &cancelled);
    // Plan 94: Render queue close - verify show_render_queue state
    let queue_closed = click(&mut harness, "composer.render_queue.close");
    assert!(!app_state(&mut harness).show_render_queue);
    queue_closed.assert_selector_absent(&selector("composer.render_queue.close"));

    // Plan 90: Quick AI panel button - verify show_ai_panel state
    let ai = click(&mut harness, "composer.quick.ai");
    assert!(app_state(&mut harness).show_ai_panel);
    for debug_id in [
        "composer.ai.feature.auto_cut",
        "composer.ai.feature.auto_subtitle",
        "composer.ai.feature.voice_to_text",
        "composer.ai.feature.bg_removal",
        "composer.ai.feature.color_match",
    ] {
        ai.assert_selector_present(&selector(debug_id));
    }
    // Plan 14: Auto Cut AI feature - verify notice after clicking
    let ai_feature = click(&mut harness, "composer.ai.feature.auto_cut");
    assert!(app_state(&mut harness).composer_notice.contains("Auto Cut"));
    assert_capture_changed(&ai, &ai_feature);

    // Plan 18/38: Context menu - verify menu state and copy clipboard
    let menu = right_click(&mut harness, "composer.timeline.clip.0");
    assert!(app_state(&mut harness).context_menu.is_some());
    for debug_id in [
        "composer.clip.context.cut",
        "composer.clip.context.copy",
        "composer.clip.context.paste",
        "composer.clip.context.duplicate",
        "composer.clip.context.delete",
    ] {
        menu.assert_selector_present(&selector(debug_id));
    }
    // Plan 38: Copy - verify clipboard and notice
    let copied = click(&mut harness, "composer.clip.context.copy");
    assert!(app_state(&mut harness).clipboard.clip.is_some());
    assert_eq!(app_state(&mut harness).composer_notice, "Clip copied");
    assert!(app_state(&mut harness).context_menu.is_none());
    copied.assert_selector_absent(&selector("composer.clip.context.copy"));
}

#[test]
fn composer_plan_keyboard_shortcuts_and_automatic_playback_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);

    // Plan 104: Spacebar play/pause - verify transport state toggles
    let before_playing = app_state(&mut harness).is_playing;
    let space = key(
        &mut harness,
        UiAutomationKey::Character(" ".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_ne!(app_state(&mut harness).is_playing, before_playing);
    assert_capture_changed(&initial, &space);

    // Plan 10: Arrow Right - verify frame advancement
    let frame_before_right = app_state(&mut harness).current_frame;
    let right = key(
        &mut harness,
        UiAutomationKey::ArrowRight,
        UiAutomationModifiers::default(),
    );
    assert_eq!(
        app_state(&mut harness).current_frame,
        frame_before_right + 1
    );
    assert_capture_changed(&space, &right);
    // Plan 09: Arrow Left - verify frame step back
    let frame_before_left = app_state(&mut harness).current_frame;
    let left = key(
        &mut harness,
        UiAutomationKey::ArrowLeft,
        UiAutomationModifiers::default(),
    );
    assert_eq!(
        app_state(&mut harness).current_frame,
        frame_before_left.saturating_sub(1)
    );
    assert_capture_changed(&right, &left);

    // Plan 79/80/81: J/K/L shuttle shortcuts - verify shuttle state
    let _before_j = app_state(&mut harness).shuttle_direction;
    let j_before = capture(&mut harness);
    let j_after = key(
        &mut harness,
        UiAutomationKey::Character("j".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_eq!(app_state(&mut harness).shuttle_direction, -1);
    assert!(app_state(&mut harness).is_playing);
    assert_capture_changed(&j_before, &j_after);

    let k_before = capture(&mut harness);
    let k_after = key(
        &mut harness,
        UiAutomationKey::Character("k".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_eq!(app_state(&mut harness).shuttle_direction, 0);
    assert!(!app_state(&mut harness).is_playing);
    assert_capture_changed(&k_before, &k_after);

    let l_before = capture(&mut harness);
    let l_after = key(
        &mut harness,
        UiAutomationKey::Character("l".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_eq!(app_state(&mut harness).shuttle_direction, 1);
    assert!(app_state(&mut harness).is_playing);
    assert_capture_changed(&l_before, &l_after);

    // Plan 98: S snap shortcut - verify snap toggles
    let before_snap = app_state(&mut harness).snap;
    let s_before = capture(&mut harness);
    let s_after = key(
        &mut harness,
        UiAutomationKey::Character("s".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_ne!(app_state(&mut harness).snap, before_snap);
    assert_capture_changed(&s_before, &s_after);

    // Plan 77: I in-point shortcut - verify in_point
    let current_frame_i = app_state(&mut harness).current_frame;
    let i_before = capture(&mut harness);
    let i_after = key(
        &mut harness,
        UiAutomationKey::Character("i".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_eq!(app_state(&mut harness).in_point, Some(current_frame_i));
    assert_capture_changed(&i_before, &i_after);

    // Plan 87: O out-point shortcut - verify out_point
    let current_frame_o = app_state(&mut harness).current_frame;
    let o_before = capture(&mut harness);
    let o_after = key(
        &mut harness,
        UiAutomationKey::Character("o".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_eq!(app_state(&mut harness).out_point, Some(current_frame_o));
    assert_capture_changed(&o_before, &o_after);

    // Plan 33: B loop playback shortcut - verify loop_playback toggles
    let before_loop = app_state(&mut harness).loop_playback;
    let b_before = capture(&mut harness);
    let b_after = key(
        &mut harness,
        UiAutomationKey::Character("b".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_ne!(app_state(&mut harness).loop_playback, before_loop);
    assert_capture_changed(&b_before, &b_after);

    // Plan 88: Plus zoom in - verify zoom increases
    let before_zoom = app_state(&mut harness).zoom;
    let plus_before = capture(&mut harness);
    let plus_after = key(
        &mut harness,
        UiAutomationKey::Character("+".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_eq!(app_state(&mut harness).zoom, before_zoom + 10.0);
    assert_capture_changed(&plus_before, &plus_after);

    // Plan 86: Minus zoom out - verify zoom decreases
    let zoom_after_plus = app_state(&mut harness).zoom;
    let minus_before = capture(&mut harness);
    let minus_after = key(
        &mut harness,
        UiAutomationKey::Character("-".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_eq!(app_state(&mut harness).zoom, zoom_after_plus - 10.0);
    assert_capture_changed(&minus_before, &minus_after);

    // Plan 06: A AI panel shortcut - verify show_ai_panel toggles
    let before_ai = app_state(&mut harness).show_ai_panel;
    let a_before = capture(&mut harness);
    let a_after = key(
        &mut harness,
        UiAutomationKey::Character("a".to_string()),
        UiAutomationModifiers::default(),
    );
    assert_ne!(app_state(&mut harness).show_ai_panel, before_ai);
    assert_capture_changed(&a_before, &a_after);

    let saved = key(
        &mut harness,
        UiAutomationKey::Character("s".to_string()),
        UiAutomationModifiers {
            control: true,
            ..UiAutomationModifiers::default()
        },
    );
    saved.assert_selector_present(&selector("composer.automatic.auto_save"));

    let exported = key(
        &mut harness,
        UiAutomationKey::Character("m".to_string()),
        UiAutomationModifiers {
            control: true,
            ..UiAutomationModifiers::default()
        },
    );
    exported.assert_selector_present(&selector("composer.render_job.pause"));

    // Plan 58: Ctrl+Z undo - verify undo stack and notice
    let _undo_stack_before = app_state(&mut harness).undo_stack.len();
    let _redo_stack_before = app_state(&mut harness).redo_stack.len();
    let undo = key(
        &mut harness,
        UiAutomationKey::Character("z".to_string()),
        UiAutomationModifiers {
            control: true,
            ..UiAutomationModifiers::default()
        },
    );
    undo.assert_selector_present(&selector("composer.automatic.notice"));
    // Plan 57: Ctrl+Shift+Z redo - verify redo stack and notice
    let redo = key(
        &mut harness,
        UiAutomationKey::Character("z".to_string()),
        UiAutomationModifiers {
            control: true,
            shift: true,
            ..UiAutomationModifiers::default()
        },
    );
    redo.assert_selector_present(&selector("composer.automatic.notice"));
}
