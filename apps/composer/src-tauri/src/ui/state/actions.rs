use super::*;
use tench_composer_core::*;

impl ComposerState {
    pub fn select_mode(&mut self, mode: ComposerMode) {
        self.mode = mode;
        self.active_inspector_tab = mode.inspector_index();
    }

    pub fn select_left_tab(&mut self, tab: LeftPanelTab) {
        self.left_tab = tab;
    }

    pub fn select_inspector_tab(&mut self, index: usize) {
        self.active_inspector_tab = index;
        self.mode = match index {
            0 => ComposerMode::Edit,
            1 => ComposerMode::Color,
            2 => ComposerMode::Audio,
            3 => ComposerMode::Deliver,
            _ => self.mode,
        };
    }

    pub fn select_template(&mut self, index: usize) -> bool {
        let Some(template) = self.project.templates.get(index) else {
            return false;
        };
        self.selected_template_idx = Some(index);
        // Apply template settings to the project.
        self.project.timeline.framerate = template.fps;
        self.project.timeline.width = template.width;
        self.project.timeline.height = template.height;
        self.project.export_settings.fps = template.fps;
        self.project.export_settings.width = template.width;
        self.project.export_settings.height = template.height;
        self.set_notice(format!("{} template applied", template.name));
        true
    }

    pub fn import_media(&mut self, path: String) {
        let id = self.project.next_id();
        let name = std::path::Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".into());
        let ext = std::path::Path::new(&path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let media_type = match ext.as_str() {
            "mp4" | "avi" | "mov" | "mkv" | "webm" | "flv" | "wmv" | "m4v" => MediaType::Video,
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => MediaType::Audio,
            _ => MediaType::Image,
        };
        let file_size = tench_media_runtime::composer::media_file_size(&path);

        // Extract metadata from media file when available.
        let (duration_frames, width, height, fps, audio_channels, audio_sample_rate) =
            Self::extract_media_metadata(&path, self.project.timeline.framerate);

        self.project.add_media(MediaAsset {
            id,
            path,
            name,
            media_type,
            duration_frames,
            width,
            height,
            fps,
            audio_channels,
            audio_sample_rate,
            file_size,
        });
        self.left_tab = LeftPanelTab::Media;
        self.import_status = "Import complete".into();
        self.set_notice("Media imported");
    }

    /// Attempt to extract metadata from a media file.
    // type_complexity: return type groups media metadata fields that are always used together
    #[allow(clippy::type_complexity)]
    fn extract_media_metadata(
        path: &str,
        project_fps: f64,
    ) -> (
        Option<u32>,
        Option<u32>,
        Option<u32>,
        Option<f64>,
        Option<u32>,
        Option<u32>,
    ) {
        let info = tench_media_runtime::composer::import_media_files(&[path.to_string()]);
        if let Ok(infos) = info {
            if let Some(mi) = infos.first() {
                let duration_frames = mi.duration.map(|d| (d * project_fps).round() as u32);
                return (
                    duration_frames,
                    mi.width,
                    mi.height,
                    Some(project_fps),
                    None,
                    None,
                );
            }
        }
        (None, None, None, None, None, None)
    }

    pub fn split_at_playhead(&mut self) -> bool {
        let Some(clip_id) = self.selected_clip_id else {
            return false;
        };
        let track_id = self
            .tracks()
            .iter()
            .find_map(|t| t.clips.iter().find(|c| c.id == clip_id).map(|_| t.id));
        let Some(track_id) = track_id else {
            return false;
        };

        self.push_undo();
        let new_id = ClipId(self.project.next_id());
        match self
            .project
            .timeline
            .split_clip(track_id, clip_id, self.current_frame, new_id)
        {
            Ok((_, right_id)) => {
                self.selected_clip_id = Some(right_id);
                self.set_notice("Clip split at playhead");
                true
            }
            Err(e) => {
                self.undo_stack.pop();
                self.set_notice(e);
                false
            }
        }
    }

    pub fn delete_selected_clip(&mut self) -> bool {
        let Some(clip_id) = self.selected_clip_id else {
            return false;
        };
        let track_id = self
            .tracks()
            .iter()
            .find_map(|t| t.clips.iter().find(|c| c.id == clip_id).map(|_| t.id));
        let Some(track_id) = track_id else {
            return false;
        };

        self.push_undo();

        if self.ripple {
            match self.project.timeline.ripple_delete(track_id, clip_id) {
                Ok(()) => {
                    self.selected_clip_id = None;
                    self.selected_clip_ids.retain(|id| *id != clip_id);
                    self.set_notice("Clip deleted (ripple)");
                    true
                }
                Err(e) => {
                    self.undo_stack.pop();
                    self.set_notice(e);
                    false
                }
            }
        } else {
            match self.project.timeline.remove_clip(track_id, clip_id) {
                Ok(()) => {
                    self.selected_clip_id = None;
                    self.selected_clip_ids.retain(|id| *id != clip_id);
                    self.set_notice("Clip deleted");
                    true
                }
                Err(e) => {
                    self.undo_stack.pop();
                    self.set_notice(e);
                    false
                }
            }
        }
    }

    pub fn enqueue_render(&mut self) {
        let name = format!(
            "{}.{}",
            self.project.name,
            self.project.export_settings.format.extension()
        );
        self.project
            .enqueue_render(name, self.project.export_settings.clone());
        self.show_render_queue = true;
        self.set_notice("Render queued");
    }

    pub fn add_clip_to_track(
        &mut self,
        track_id: TrackId,
        name: String,
        source: String,
        timeline_in: u32,
        duration: u32,
    ) {
        self.push_undo();
        let clip_id = ClipId(self.project.next_id());
        let clip = Clip {
            id: clip_id,
            name,
            source_path: source,
            media_in: 0,
            media_out: duration,
            timeline_in,
            duration,
            speed: 1.0,
            reversed: false,
            enabled: true,
            effect_ids: Vec::new(),
            transition_in: None,
            transition_out: None,
        };
        let mode = if self.ripple {
            EditMode::Insert
        } else {
            EditMode::Overwrite
        };
        match self.project.timeline.insert_clip(track_id, clip, mode) {
            Ok(()) => {
                self.selected_clip_id = Some(clip_id);
                self.set_notice("Clip added");
            }
            Err(e) => {
                self.undo_stack.pop();
                self.set_notice(e);
            }
        }
    }

    /// Add media from bin to a specific track at a given timeline position.
    pub fn drop_media_on_track(&mut self, media_idx: usize, track_id: TrackId, timeline_in: u32) {
        let Some(asset) = self.media_bin().get(media_idx).cloned() else {
            return;
        };
        let duration = asset
            .duration_frames
            .unwrap_or((self.fps() * 5.0).round() as u32);
        self.add_clip_to_track(track_id, asset.name, asset.path, timeline_in, duration);
    }

    /// Add an effect to the selected clip.
    pub fn add_effect_to_clip(&mut self, clip_id: ClipId, effect_type: VideoEffectType) {
        self.push_undo();
        let effect_id = self.project.next_id();
        let effect = Effect::new_video(effect_id, effect_type);
        self.project.effects.push(effect);
        for track in &mut self.project.timeline.tracks {
            for clip in &mut track.clips {
                if clip.id == clip_id {
                    clip.effect_ids.push(effect_id);
                    self.set_notice(format!("{} added", effect_type.label()));
                    return;
                }
            }
        }
        self.undo_stack.pop();
    }

    /// Move a clip to a new position.
    pub fn move_clip(
        &mut self,
        clip_id: ClipId,
        source_track: TrackId,
        dest_track: TrackId,
        new_in: u32,
    ) {
        self.push_undo();
        if let Err(e) = self
            .project
            .timeline
            .move_clip(clip_id, source_track, dest_track, new_in)
        {
            self.undo_stack.pop();
            self.set_notice(e);
        } else {
            self.set_notice("Clip moved");
        }
    }

    /// Trim clip in-point.
    pub fn trim_clip_in(&mut self, clip_id: ClipId, track_id: TrackId, new_in: u32) {
        self.push_undo();
        if let Err(e) = self
            .project
            .timeline
            .trim_clip_in(track_id, clip_id, new_in)
        {
            self.undo_stack.pop();
            self.set_notice(e);
        }
    }

    /// Trim clip out-point.
    pub fn trim_clip_out(&mut self, clip_id: ClipId, track_id: TrackId, new_out: u32) {
        self.push_undo();
        if let Err(e) = self
            .project
            .timeline
            .trim_clip_out(track_id, clip_id, new_out)
        {
            self.undo_stack.pop();
            self.set_notice(e);
        }
    }

    /// Add a new track.
    pub fn add_track(&mut self, kind: TrackType) {
        let id = TrackId(self.project.next_id());
        let count = self.tracks().iter().filter(|t| t.kind == kind).count();
        let name = match kind {
            TrackType::Video => format!("V{} - Video", count + 1),
            TrackType::Audio => format!("A{} - Audio", count + 1),
            TrackType::Subtitle => format!("S{} - Subtitles", count + 1),
        };
        self.project.timeline.add_track(Track::new(id, name, kind));
        self.set_notice("Track added");
    }

    /// Delete a track.
    pub fn delete_track(&mut self, track_id: TrackId) {
        self.push_undo();
        if self.project.timeline.remove_track(track_id) {
            self.set_notice("Track deleted");
        }
    }

    /// Cut selected clip to clipboard.
    pub fn cut_clip(&mut self, clip_id: ClipId) {
        if let Some(clip) = self.find_clip(clip_id).cloned() {
            self.clipboard.clip = Some(clip);
            self.delete_selected_clip();
            self.set_notice("Clip cut");
        }
    }

    /// Copy selected clip to clipboard.
    pub fn copy_clip(&mut self, clip_id: ClipId) {
        if let Some(clip) = self.find_clip(clip_id).cloned() {
            self.clipboard.clip = Some(clip);
            self.set_notice("Clip copied");
        }
    }

    /// Paste clipboard clip at playhead.
    pub fn paste_clip(&mut self) {
        let Some(clip) = self.clipboard.clip.clone() else {
            self.set_notice("Nothing to paste");
            return;
        };
        let track_id = self.tracks().iter().find(|t| !t.locked).map(|t| t.id);
        let Some(track_id) = track_id else {
            self.set_notice("No unlocked track");
            return;
        };
        self.add_clip_to_track(
            track_id,
            clip.name.clone(),
            clip.source_path.clone(),
            self.current_frame,
            clip.duration,
        );
    }

    /// Duplicate selected clip.
    pub fn duplicate_clip(&mut self, clip_id: ClipId) {
        let Some(clip) = self.find_clip(clip_id).cloned() else {
            return;
        };
        let track_id = self
            .tracks()
            .iter()
            .find(|t| t.clips.iter().any(|c| c.id == clip_id))
            .map(|t| t.id);
        let Some(track_id) = track_id else {
            return;
        };
        self.add_clip_to_track(
            track_id,
            format!("{} (copy)", clip.name),
            clip.source_path.clone(),
            clip.timeline_out(),
            clip.duration,
        );
    }
}
