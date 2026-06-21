use super::*;
use tench_composer_core::*;

impl ComposerState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            project: ComposerProject::new("Untitled Project".into()),
            mode: ComposerMode::Edit,
            left_tab: LeftPanelTab::Media,
            active_inspector_tab: 0,
            is_playing: false,
            current_frame: 0,
            zoom: 50.0,
            snap: true,
            ripple: false,
            magnetic: false,
            selected_clip_id: None,
            selected_clip_ids: Vec::new(),
            selected_template_idx: None,
            import_status: "Ready".into(),
            composer_notice: "Project ready".into(),
            notice_expires_at: None,
            show_render_queue: false,
            show_ai_panel: false,
            subtitle_text: String::new(),
            subtitle_focused: false,
            input_focus: ComposerInputFocus::None,
            in_point: None,
            out_point: None,
            loop_playback: false,
            shuttle_speed: 1.0,
            shuttle_direction: 0,
            j_press_count: 0,
            l_press_count: 0,
            drag: None,
            drag_start_pos: None,
            context_menu: None,
            clipboard: ComposerClipboard::default(),
            left_panel_w: 240.0,
            right_panel_w: 280.0,
            timeline_h: 240.0,
            save_path: None,
            last_auto_save: None,
            recent_projects: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            effects_search: String::new(),
            transitions_search: String::new(),
            waveform_cache: std::collections::HashMap::new(),
            auto_save_interval: std::time::Duration::from_secs(120),
        }
    }

    pub fn example() -> Self {
        let mut state = Self::new();
        let media_id = state.project.next_id();
        state.project.add_media(MediaAsset {
            id: media_id,
            path: "/tmp/tench-composer-demo.mp4".into(),
            name: "demo.mp4".into(),
            media_type: MediaType::Video,
            duration_frames: Some(180),
            width: Some(1920),
            height: Some(1080),
            fps: Some(24.0),
            audio_channels: Some(2),
            audio_sample_rate: Some(48_000),
            file_size: 0,
        });
        if let Some(track_id) = state
            .project
            .timeline
            .tracks
            .iter()
            .find(|track| track.kind == TrackType::Video)
            .map(|track| track.id)
        {
            state.add_clip_to_track(
                track_id,
                "Opening Clip".into(),
                "/tmp/tench-composer-demo.mp4".into(),
                0,
                120,
            );
            state.current_frame = 48;
        }
        state.save_path = Some("/tmp/tench-composer-demo.tench-composer".into());
        state.composer_notice = "Demo project ready".into();
        state
    }
}
