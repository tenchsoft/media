use super::super::state;
use super::PlayerApp;
use tench_media_runtime::player::{downsample_rgba, save_screenshot_png};

impl PlayerApp {
    pub(crate) fn load_and_play(&mut self, path: &str) {
        // Save position of current file before switching
        if let Some(ref current_path) = self.state.media_path {
            crate::platform_util::PersistentState::save_position(
                current_path,
                self.state.current_time,
            );
        }
        self.state.open_media_from_path(path);
        self.video_frame = None;
        if let Some(ref mut backend) = self.backend {
            backend.load(path);
        }
    }

    pub(crate) fn load_media_for_action(
        &mut self,
        path: &str,
        title: Option<String>,
        duration: Option<f64>,
    ) {
        if self.backend.is_some() {
            self.load_and_play(path);
            return;
        }

        let fallback_title = path
            .split(['/', '\\'])
            .next_back()
            .filter(|name| !name.is_empty())
            .unwrap_or("media")
            .to_string();
        self.state.media_path = Some(path.to_string());
        self.state
            .open_media(title.unwrap_or(fallback_title), duration.unwrap_or(0.0));
    }

    /// Load the current track from the playlist.
    pub(crate) fn load_current_track(&mut self) {
        if let Some(idx) = self.state.current_playlist_index {
            if let Some(path) = self.state.playlist.get(idx).map(|entry| entry.path.clone()) {
                self.load_and_play(&path);
            }
        }
    }

    /// Process events from the GStreamer backend.
    pub(crate) fn process_backend_events(&mut self) {
        let events: Vec<crate::gst_backend::MediaEvent> = {
            let Some(ref rx) = self.backend_rx else {
                return;
            };
            rx.try_iter().collect()
        };
        for event in events {
            match event {
                crate::gst_backend::MediaEvent::VideoFrame {
                    pixels,
                    width,
                    height,
                } => {
                    // Convert RGBA pixels to peniko::ImageData
                    if !pixels.is_empty() && width > 0 && height > 0 {
                        self.video_frame = Some(tench_ui::peniko::ImageData {
                            data: pixels.clone().into(),
                            format: tench_ui::peniko::ImageFormat::Rgba8,
                            alpha_type: tench_ui::peniko::ImageAlphaType::AlphaPremultiplied,
                            width,
                            height,
                        });
                        self.video_dims = (width, height);
                        // Update resolution in media info
                        self.state.media_info.resolution = format!("{}x{}", width, height);
                        // Collect frame for GIF capture (downsample to 320px width)
                        if self.gif_recording {
                            // FPS limiting: skip frames to match configured FPS
                            let target_fps = self.state.gif_options.fps.max(1) as u64;
                            let frame_interval_ms = 1000 / target_fps;
                            let now_ms = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis() as u64;
                            let elapsed = self
                                .gif_last_frame_ms
                                .map(|last| now_ms - last)
                                .unwrap_or(u64::MAX);
                            if elapsed >= frame_interval_ms {
                                self.gif_last_frame_ms = Some(now_ms);
                                let max_w = 320u32;
                                let scale = if width > max_w {
                                    max_w as f64 / width as f64
                                } else {
                                    1.0
                                };
                                let sw = (width as f64 * scale) as u16;
                                let sh = (height as f64 * scale) as u16;
                                self.gif_dims = (sw, sh);
                                // Downsample RGBA pixels
                                let downsampled = downsample_rgba(&pixels, width, height, sw, sh);
                                self.gif_frames.push(downsampled);
                                // Keep max frames based on configured max duration
                                let max_frames = (self.state.gif_options.max_duration_secs
                                    as usize
                                    * target_fps as usize)
                                    .max(1);
                                while self.gif_frames.len() > max_frames {
                                    self.gif_frames.remove(0);
                                }
                                self.state.gif_state =
                                    format!("recording ({} frames)", self.gif_frames.len());
                            }
                        }
                    }
                }
                crate::gst_backend::MediaEvent::Loaded {
                    duration,
                    width,
                    height,
                } => {
                    self.state.duration = duration;
                    self.state.media_info.resolution = format!("{}x{}", width, height);
                    // Query real metadata from GStreamer pipeline
                    if let Some(ref backend) = self.backend {
                        let meta = backend.query_metadata();
                        if !meta.video_codec.is_empty() {
                            self.state.media_info.video_codec = meta.video_codec;
                        }
                        if !meta.audio_codec.is_empty() {
                            self.state.media_info.audio_codec = meta.audio_codec;
                        }
                        if !meta.bitrate.is_empty() {
                            self.state.media_info.bitrate = meta.bitrate;
                        }
                        if meta.framerate > 0.0 {
                            self.state.media_info.frame_rate = meta.framerate;
                        }
                        if !meta.title.is_empty() {
                            self.state.media_info.title = meta.title;
                            self.state.title = self.state.media_info.title.clone();
                        }
                        if !meta.artist.is_empty() {
                            self.state.media_info.artist = meta.artist;
                        }
                        if !meta.album.is_empty() {
                            self.state.media_info.album = meta.album;
                        }
                    }
                    // Auto-play after loading
                    if let Some(ref mut backend) = self.backend {
                        backend.play();
                        self.state.is_playing = true;
                    }
                    // Enumerate audio devices
                    self.state.audio_devices =
                        crate::gst_backend::PlayerBackend::enumerate_audio_devices();
                }
                crate::gst_backend::MediaEvent::Position(pos) => {
                    self.state.current_time = pos;
                    self.state.update_subtitle_for_position();
                }
                crate::gst_backend::MediaEvent::Duration(dur) => {
                    self.state.duration = dur;
                    // Query chapters from container metadata
                    if let Some(ref backend) = self.backend {
                        let chapters = backend.query_chapters();
                        if !chapters.is_empty() {
                            self.state.chapters = chapters
                                .into_iter()
                                .map(|(title, start)| state::ChapterMark {
                                    title,
                                    time: start,
                                    ai_generated: false,
                                })
                                .collect();
                        }
                    }
                }
                crate::gst_backend::MediaEvent::EndOfStream => {
                    self.state.is_playing = false;
                    // Save final position
                    if let Some(ref path) = self.state.media_path {
                        crate::platform_util::PersistentState::save_position(
                            path,
                            self.state.current_time,
                        );
                    }
                    // Auto-advance to next track
                    if !self.state.playlist.is_empty() {
                        let has_next = self
                            .state
                            .current_playlist_index
                            .is_some_and(|idx| idx + 1 < self.state.playlist.len());
                        if has_next {
                            self.state.next_track();
                            if let Some(entry) = self
                                .state
                                .current_playlist_index
                                .and_then(|idx| self.state.playlist.get(idx))
                            {
                                let path = entry.path.clone();
                                self.load_and_play(&path);
                            }
                        }
                    }
                }
                crate::gst_backend::MediaEvent::Error(msg) => {
                    self.state.show_toast(format!("Error: {msg}"));
                }
                crate::gst_backend::MediaEvent::AudioOnly => {
                    self.state.media_info.resolution = "Audio only".into();
                }
                crate::gst_backend::MediaEvent::AboutToFinish => {
                    // Gapless: set next track URI on the pipeline
                    if let Some(ref mut backend) = self.backend {
                        // Find next track
                        let has_next = self
                            .state
                            .current_playlist_index
                            .is_some_and(|idx| idx + 1 < self.state.playlist.len());
                        if has_next {
                            if let Some(next_idx) =
                                self.state.current_playlist_index.map(|idx| idx + 1)
                            {
                                if let Some(entry) = self.state.playlist.get(next_idx) {
                                    let uri = if entry.path.starts_with("file://")
                                        || entry.path.contains("://")
                                    {
                                        entry.path.clone()
                                    } else {
                                        format!("file://{}", entry.path)
                                    };
                                    backend.set_next_uri(uri);
                                }
                            }
                        }
                    }
                }
                crate::gst_backend::MediaEvent::Buffering(percent) => {
                    let was_buffering = self.state.is_buffering;
                    self.state.buffering_percent = percent;
                    self.state.is_buffering = percent < 100;
                    if was_buffering && percent == 100 {
                        // Buffering complete, resume playback
                        if let Some(ref mut backend) = self.backend {
                            backend.play();
                            self.state.is_playing = true;
                        }
                    } else if percent < 100 && self.state.is_playing {
                        // Pause while buffering
                        if let Some(ref mut backend) = self.backend {
                            backend.pause();
                        }
                    }
                }
                crate::gst_backend::MediaEvent::AudioLevels(levels) => {
                    self.state.audio_levels = levels;
                }
            }
        }

        // Poll position from backend during playback
        if let Some(ref mut backend) = self.backend {
            if self.state.is_playing {
                let (pos, dur) = backend.tick();
                if dur > 0.0 {
                    self.state.duration = dur;
                }
                if pos > 0.0 {
                    self.state.current_time = pos;
                    self.state.update_subtitle_for_position();
                }
            }
            self.state.is_playing = backend.is_playing();
            // Update built-in subtitle track count
            self.state.n_builtin_subtitle_tracks = backend.n_subtitle_streams();
        }

        // Update MPRIS state (Linux only)
        #[cfg(target_os = "linux")]
        {
            if let Some(ref mpris) = self.mpris_state {
                if let Ok(mut s) = mpris.lock() {
                    s.title = self.state.media_info.title.clone();
                    s.artist = self.state.media_info.artist.clone();
                    s.album = self.state.media_info.album.clone();
                    s.is_playing = self.state.is_playing;
                    s.position_secs = self.state.current_time;
                    s.duration_secs = self.state.duration;
                    s.can_play = !self.state.playlist.is_empty();
                    s.can_pause = !self.state.playlist.is_empty();
                    s.can_go_next = self.state.playlist.len() > 1;
                    s.can_go_previous = self.state.playlist.len() > 1;
                }
            }
            // Process MPRIS commands
            let commands: Vec<crate::mpris::MprisCommand> = self
                .mpris_cmd_rx
                .as_ref()
                .map(|rx| rx.try_iter().collect())
                .unwrap_or_default();
            for cmd in commands {
                match cmd {
                    crate::mpris::MprisCommand::Play => {
                        if let Some(ref mut b) = self.backend {
                            b.play();
                        }
                        self.state.is_playing = true;
                    }
                    crate::mpris::MprisCommand::Pause => {
                        if let Some(ref mut b) = self.backend {
                            b.pause();
                        }
                        self.state.is_playing = false;
                    }
                    crate::mpris::MprisCommand::PlayPause => {
                        if let Some(ref mut b) = self.backend {
                            if self.state.is_playing {
                                b.pause();
                            } else {
                                b.play();
                            }
                        }
                        self.state.is_playing = !self.state.is_playing;
                    }
                    crate::mpris::MprisCommand::Stop => {
                        if let Some(ref mut b) = self.backend {
                            b.stop();
                        }
                        self.state.is_playing = false;
                    }
                    crate::mpris::MprisCommand::Next => {
                        self.state.next_track();
                        self.load_current_track();
                    }
                    crate::mpris::MprisCommand::Previous => {
                        self.state.prev_track();
                        self.load_current_track();
                    }
                    crate::mpris::MprisCommand::Seek(offset) => {
                        let new_pos = (self.state.current_time + offset).max(0.0);
                        if let Some(ref mut b) = self.backend {
                            b.seek(new_pos);
                        }
                    }
                }
            }
        }

        // Update system media state (macOS/Windows only)
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            if let Some(ref sms) = self.system_media_state {
                if let Ok(mut s) = sms.lock() {
                    s.title = self.state.media_info.title.clone();
                    s.artist = self.state.media_info.artist.clone();
                    s.album = self.state.media_info.album.clone();
                    s.is_playing = self.state.is_playing;
                    s.position_secs = self.state.current_time;
                    s.duration_secs = self.state.duration;
                }
            }
            // Process system media commands
            let mut system_media_commands = Vec::new();
            if let Some(rx) = self.system_media_cmd_rx.as_ref() {
                while let Ok(cmd) = rx.try_recv() {
                    system_media_commands.push(cmd);
                }
            }

            for cmd in system_media_commands {
                match cmd {
                    crate::system_media::SystemMediaCommand::Play => {
                        if let Some(ref mut b) = self.backend {
                            b.play();
                        }
                        self.state.is_playing = true;
                    }
                    crate::system_media::SystemMediaCommand::Pause => {
                        if let Some(ref mut b) = self.backend {
                            b.pause();
                        }
                        self.state.is_playing = false;
                    }
                    crate::system_media::SystemMediaCommand::PlayPause => {
                        if let Some(ref mut b) = self.backend {
                            if self.state.is_playing {
                                b.pause();
                            } else {
                                b.play();
                            }
                        }
                        self.state.is_playing = !self.state.is_playing;
                    }
                    crate::system_media::SystemMediaCommand::Stop => {
                        if let Some(ref mut b) = self.backend {
                            b.stop();
                        }
                        self.state.is_playing = false;
                    }
                    crate::system_media::SystemMediaCommand::Next => {
                        self.state.next_track();
                        self.load_current_track();
                    }
                    crate::system_media::SystemMediaCommand::Previous => {
                        self.state.prev_track();
                        self.load_current_track();
                    }
                    crate::system_media::SystemMediaCommand::Seek(offset) => {
                        let new_pos = (self.state.current_time + offset).max(0.0);
                        if let Some(ref mut b) = self.backend {
                            b.seek(new_pos);
                        }
                    }
                }
            }
        }
    }

    /// Capture the current video frame and save as PNG.
    pub(crate) fn take_screenshot(&mut self) {
        let Some(ref frame) = self.video_frame else {
            self.state.show_toast("No video frame to capture");
            return;
        };

        let (w, h) = self.video_dims;
        if w == 0 || h == 0 {
            self.state.show_toast("No video frame to capture");
            return;
        }

        let pixels: Vec<u8> = frame.data.as_ref().to_vec();
        match save_screenshot_png(pixels, w, h) {
            Ok(path) => self.state.show_toast(format!("Screenshot saved: {path}")),
            Err(e) => {
                self.state.show_toast(format!("Screenshot failed: {e}"));
            }
        }
    }
}
