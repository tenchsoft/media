use super::*;
use tench_media_runtime::player::SubtitleCue;
use tench_ui::prelude::Rect;

#[test]
fn new_state_has_no_media() {
    let state = PlayerState::new();
    assert!(!state.has_media);
    assert!(!state.is_playing);
    assert_eq!(state.current_time, 0.0);
    assert_eq!(state.duration, 0.0);
    assert!(state.playlist.is_empty());
    assert!(state.subtitle_cues.is_empty());
    assert!(state.chapters.is_empty());
    assert!(state.media_path.is_none());
}

#[test]
fn opening_media_resets_playback_and_adds_playlist_entry() {
    let mut state = PlayerState::new();
    state.open_media("lecture.mp4", 300.0);

    assert!(state.has_media);
    assert_eq!(state.title, "lecture.mp4");
    assert_eq!(state.current_time, 0.0);
    assert_eq!(state.duration, 300.0);
    assert!(!state.is_playing);
}

#[test]
fn seek_and_volume_are_clamped() {
    let mut state = PlayerState::new();
    state.open_media("test.mp4", 100.0);

    state.seek_to(9999.0);
    assert_eq!(state.current_time, 100.0);
    state.seek_by(-9999.0);
    assert_eq!(state.current_time, 0.0);

    state.set_volume(2.0);
    assert_eq!(state.volume, 1.0);
    state.set_volume(-1.0);
    assert_eq!(state.volume, 0.0);
    assert!(state.is_muted);
}

#[test]
fn bookmark_uses_current_time() {
    let mut state = PlayerState::new();
    state.open_media("test.mp4", 100.0);
    state.seek_to(42.0);
    let old_len = state.chapters.len();

    state.add_bookmark();

    assert_eq!(state.chapters.len(), old_len + 1);
    assert_eq!(state.chapters.last().unwrap().time, 42.0);
    assert_eq!(
        state.toast.as_deref(),
        Some("Bookmark added — type name and press Enter")
    );
}

#[test]
fn click_regions_register_and_find() {
    let mut state = PlayerState::new();
    let rect = Rect::new(10.0, 10.0, 50.0, 50.0);
    state.register_click(rect, ClickAction::PlayPause);

    assert!(state.click_action_at(30.0, 30.0).is_some());
    assert!(state.click_action_at(5.0, 5.0).is_none());
}

#[test]
fn click_regions_clear_on_new_frame() {
    let mut state = PlayerState::new();
    let rect = Rect::new(10.0, 10.0, 50.0, 50.0);
    state.register_click(rect, ClickAction::PlayPause);
    state.clear_click_regions();
    assert!(state.click_action_at(30.0, 30.0).is_none());
}

#[test]
fn toggle_playback_requires_media() {
    let mut state = PlayerState::new();
    state.toggle_playback();
    assert!(!state.is_playing);

    state.open_media("test.mp4", 100.0);
    state.toggle_playback();
    assert!(state.is_playing);
    state.toggle_playback();
    assert!(!state.is_playing);
}

#[test]
fn speed_up_and_down() {
    let mut state = PlayerState::new();
    assert_eq!(state.playback_rate, 1.0);

    state.speed_up();
    assert_eq!(state.playback_rate, 1.25);

    state.speed_down();
    assert_eq!(state.playback_rate, 1.0);
}

#[test]
fn ab_loop_cycle() {
    let mut state = PlayerState::new();
    state.open_media("test.mp4", 100.0);
    state.seek_to(10.0);

    state.toggle_ab_loop();
    assert_eq!(state.ab_stage, 1);
    assert_eq!(state.ab_loop, Some((10.0, 10.0)));

    state.seek_to(30.0);
    state.toggle_ab_loop();
    assert_eq!(state.ab_stage, 2);
    assert_eq!(state.ab_loop, Some((10.0, 30.0)));

    state.toggle_ab_loop();
    assert_eq!(state.ab_stage, 0);
    assert!(state.ab_loop.is_none());
}

#[test]
fn subtitle_cue_matching() {
    let mut state = PlayerState::new();
    state.open_media("test.mp4", 100.0);
    state.subtitle_cues = vec![
        SubtitleCue {
            id: "1".into(),
            start: 5.0,
            end: 10.0,
            text: "Hello".into(),
        },
        SubtitleCue {
            id: "2".into(),
            start: 15.0,
            end: 20.0,
            text: "World".into(),
        },
    ];

    state.seek_to(7.0);
    assert_eq!(state.subtitle_text, Some("Hello".into()));

    state.seek_to(12.0);
    assert!(state.subtitle_text.is_none());

    state.seek_to(18.0);
    assert_eq!(state.subtitle_text, Some("World".into()));
}

#[test]
fn close_all_panels() {
    let mut state = PlayerState::new();
    state.drawer = Some(DrawerTab::Playlist);
    state.ai_panel_open = true;
    state.gif_capture_open = true;
    state.show_speed_menu = true;

    state.close_all_panels();
    assert!(state.drawer.is_none());
    assert!(!state.ai_panel_open);
    assert!(!state.gif_capture_open);
    assert!(!state.show_speed_menu);
}
