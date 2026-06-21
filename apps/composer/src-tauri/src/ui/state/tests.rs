use super::*;
use tench_composer_core::TrackType;

#[test]
fn new_state_has_project() {
    let state = ComposerState::new();
    assert_eq!(state.project.name, "Untitled Project");
    assert_eq!(state.tracks().len(), 5);
    assert_eq!(state.mode, ComposerMode::Edit);
}

#[test]
fn import_media_adds_to_bin() {
    let mut state = ComposerState::new();
    state.import_media("/tmp/test.mp4".into());
    assert_eq!(state.media_bin().len(), 1);
    assert_eq!(state.left_tab, LeftPanelTab::Media);
}

#[test]
fn split_and_delete_clip() {
    let mut state = ComposerState::new();
    let track_id = state.tracks()[0].id;
    state.add_clip_to_track(track_id, "Test".into(), "test.mp4".into(), 0, 200);
    assert!(state.selected_clip_id.is_some());

    state.seek_to_frame(100);
    assert!(state.split_at_playhead());
    assert_eq!(state.tracks()[0].clips.len(), 2);

    assert!(state.delete_selected_clip());
    assert_eq!(state.tracks()[0].clips.len(), 1);
}

#[test]
fn undo_redo_works() {
    let mut state = ComposerState::new();
    let track_id = state.tracks()[0].id;
    state.add_clip_to_track(track_id, "Test".into(), "test.mp4".into(), 0, 100);
    assert_eq!(state.tracks()[0].clips.len(), 1);

    state.undo();
    assert_eq!(state.tracks()[0].clips.len(), 0);

    state.redo();
    assert_eq!(state.tracks()[0].clips.len(), 1);
}

#[test]
fn notice_auto_dismiss() {
    let mut state = ComposerState::new();
    state.set_notice("Test");
    assert_eq!(state.composer_notice, "Test");
    assert!(state.notice_expires_at.is_some());
}

#[test]
fn inspector_tab_syncs_with_mode() {
    let mut state = ComposerState::new();
    state.select_mode(ComposerMode::Color);
    assert_eq!(state.active_inspector_tab, 1);
    state.select_inspector_tab(3);
    assert_eq!(state.mode, ComposerMode::Deliver);
}

#[test]
fn add_and_delete_track() {
    let mut state = ComposerState::new();
    let initial = state.tracks().len();
    state.add_track(TrackType::Video);
    assert_eq!(state.tracks().len(), initial + 1);
    let new_id = state.tracks().last().unwrap().id;
    state.delete_track(new_id);
    assert_eq!(state.tracks().len(), initial);
}

#[test]
fn snap_position_finds_boundary() {
    let mut state = ComposerState::new();
    let track_id = state.tracks()[0].id;
    state.add_clip_to_track(track_id, "A".into(), "a.mp4".into(), 0, 100);
    state.add_clip_to_track(track_id, "B".into(), "b.mp4".into(), 200, 100);
    assert_eq!(state.snap_position(98, 5), 100);
    assert_eq!(state.snap_position(50, 5), 50);
}

#[test]
fn shuttle_jkl() {
    let mut state = ComposerState::new();
    state.shuttle_forward();
    assert_eq!(state.shuttle_direction, 1);
    assert_eq!(state.shuttle_speed, 1.0);
    state.shuttle_forward();
    assert_eq!(state.shuttle_speed, 2.0);
    state.shuttle_stop();
    assert!(!state.is_playing);
    state.shuttle_reverse();
    assert_eq!(state.shuttle_direction, -1);
    assert_eq!(state.shuttle_speed, 1.0);
}
