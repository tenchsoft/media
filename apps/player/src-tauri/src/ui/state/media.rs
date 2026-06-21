use super::*;
use tench_media_runtime::player as media_service;

impl PlayerState {
    // ── Media loading ──

    /// Open a media file and update state.
    pub fn open_media(&mut self, title: impl Into<String>, duration: f64) {
        let title = title.into();
        self.has_media = true;
        self.title = title.clone();
        self.current_time = 0.0;
        self.duration = duration.max(0.0);
        self.is_playing = false;
        self.subtitle_text = None;
        self.remembered_position = None;
        self.ab_loop = None;
        self.ab_stage = 0;
        self.media_info.file_name = title.clone();
        self.show_toast("Media loaded");
    }

    /// Load a media file from path with metadata guessing.
    pub fn open_media_from_path(&mut self, path: &str) {
        let info = media_service::guess_media_info(path);
        let file_name = path
            .split(['/', '\\'])
            .next_back()
            .unwrap_or("media")
            .to_string();

        self.media_path = Some(path.to_string());
        self.media_info.file_name = info.file_name.clone();
        self.media_info.video_codec = info.video_codec;
        self.media_info.audio_codec = info.audio_codec;
        self.media_info.bitrate = if info.file_size > 0 {
            format!("{:.1} Mbps", info.file_size as f64 / 1_000_000.0)
        } else {
            "Unknown".into()
        };

        // Restore saved position
        self.remembered_position = crate::platform_util::PersistentState::load_position(path);

        // Add to recent files
        crate::platform_util::PersistentState::add_recent(path, &file_name, 0.0);

        // Refresh recent files list
        self.recent_files = crate::platform_util::PersistentState::recent_files()
            .into_iter()
            .map(|e| PlaylistEntry {
                title: e.title,
                duration: e.duration,
                path: e.path,
            })
            .collect();

        // Add to playlist if not already there
        if !self.playlist.iter().any(|e| e.path == path) {
            self.playlist.insert(
                0,
                PlaylistEntry {
                    title: file_name.clone(),
                    duration: 0.0,
                    path: path.to_string(),
                },
            );
        }
        self.current_playlist_index = self.playlist.iter().position(|e| e.path == path);

        self.open_media(file_name, 0.0);
    }

    /// Load a folder of media files into the playlist.
    pub fn open_folder(&mut self, folder: &str) {
        match media_service::scan_video_folder(folder) {
            Ok(entries) => {
                if entries.is_empty() {
                    self.show_toast("No video files found");
                    return;
                }
                self.playlist = entries
                    .into_iter()
                    .map(|e| PlaylistEntry {
                        title: e.title,
                        duration: e.duration,
                        path: e.path,
                    })
                    .collect();
                self.current_playlist_index = Some(0);
                if let Some(first) = self.playlist.first() {
                    let path = first.path.clone();
                    self.open_media_from_path(&path);
                }
            }
            Err(e) => {
                self.show_toast(format!("Error: {}", e));
            }
        }
    }

    /// Load subtitle cues from a file.
    pub fn load_subtitles(&mut self, path: &str) {
        match media_service::parse_subtitle_file(path) {
            Ok(cues) => {
                self.subtitle_cues = cues;
                let lang = path.split('.').nth_back(1).unwrap_or("und").to_string();
                self.subtitle_tracks.push(SubtitleTrack {
                    language: lang,
                    active: true,
                    offset_ms: 0,
                });
                self.show_toast("Subtitles loaded");
            }
            Err(e) => {
                self.show_toast(format!("Subtitle error: {}", e));
            }
        }
    }

    /// Update the subtitle display text based on current playback position.
    /// Respects per-track offset_ms (positive = subtitles appear later).
    pub fn update_subtitle_for_position(&mut self) {
        if self.subtitle_cues.is_empty() {
            return;
        }
        // Get active track offset
        let offset_s = self
            .subtitle_tracks
            .iter()
            .find(|t| t.active)
            .map(|t| t.offset_ms as f64 / 1000.0)
            .unwrap_or(0.0);
        let adjusted_t = self.current_time - offset_s;
        let active = self
            .subtitle_cues
            .iter()
            .find(|c| adjusted_t >= c.start && adjusted_t <= c.end);
        self.subtitle_text = active.map(|c| c.text.clone());
    }
}
