use super::super::state::{AiChatMessage, AiMessageRole, ClickAction, EqPreset, SubtitleEncoding};
use super::PlayerApp;
use crate::{dialog_sender, DialogResult};
use tauri::Manager;
use tench_media_runtime::player::{capture_output_path, encode_gif, encode_gif_to_path};
use tench_ui::prelude::*;

impl PlayerApp {
    pub(crate) fn dispatch_click_action(&mut self, action: &ClickAction, ctx: &mut EventCtx) {
        match action {
            ClickAction::PlayPause => {
                if let Some(ref mut backend) = self.backend {
                    if backend.is_playing() {
                        backend.pause();
                    } else {
                        backend.play();
                    }
                }
                self.state.toggle_playback();
                ctx.request_paint();
            }
            ClickAction::SeekTo(pos) => {
                self.state.seek_to(*pos);
                if let Some(ref mut backend) = self.backend {
                    backend.seek(*pos);
                }
                ctx.request_paint();
            }
            ClickAction::VolumeSet(vol) => {
                self.state.set_volume(*vol);
                if let Some(ref mut backend) = self.backend {
                    backend.set_volume(*vol);
                }
                ctx.request_paint();
            }
            ClickAction::NextTrack => {
                self.state.next_track();
                if let Some(entry) = self
                    .state
                    .current_playlist_index
                    .and_then(|idx| self.state.playlist.get(idx))
                {
                    let path = entry.path.clone();
                    self.load_and_play(&path);
                }
                ctx.request_paint();
            }
            ClickAction::PrevTrack => {
                self.state.prev_track();
                if let Some(entry) = self
                    .state
                    .current_playlist_index
                    .and_then(|idx| self.state.playlist.get(idx))
                {
                    let path = entry.path.clone();
                    self.load_and_play(&path);
                }
                ctx.request_paint();
            }
            ClickAction::ToggleFullscreen => {
                if let Some(ref handle) = self.app_handle {
                    if let Some(wvw) = handle.get_webview_window("main") {
                        let is_fullscreen = wvw.is_fullscreen().unwrap_or(false);
                        let _ = wvw.set_fullscreen(!is_fullscreen);
                    }
                }
                ctx.request_paint();
            }
            ClickAction::ToggleMute => {
                self.state.toggle_mute();
                if let Some(ref mut backend) = self.backend {
                    backend.set_muted(self.state.is_muted);
                }
                ctx.request_paint();
            }
            ClickAction::SetSpeed(rate) => {
                self.state.set_playback_rate(*rate);
                if let Some(ref mut backend) = self.backend {
                    backend.set_playback_rate(*rate);
                }
                self.state.show_speed_menu = false;
                ctx.request_paint();
            }
            ClickAction::ToggleDrawer(tab) => {
                self.state.toggle_drawer(*tab);
                if self.state.drawer.is_some() {
                    self.state.ai_panel_open = false;
                }
                ctx.request_paint();
            }
            ClickAction::SelectSubtitleTrack(idx) => {
                for (i, track) in self.state.subtitle_tracks.iter_mut().enumerate() {
                    track.active = i == *idx;
                }
                ctx.request_paint();
            }
            ClickAction::SelectAudioTrack(idx) => {
                if let Some(ref mut backend) = self.backend {
                    backend.set_audio_track(*idx);
                }
                self.state
                    .show_toast(format!("Audio track {} selected", idx + 1));
                ctx.request_paint();
            }
            ClickAction::Screenshot => {
                self.take_screenshot();
                ctx.request_paint();
            }
            ClickAction::ToggleABLoop => {
                self.state.toggle_ab_loop();
                ctx.request_paint();
            }
            ClickAction::OpenFile => {
                self.open_file_dialog();
            }
            ClickAction::OpenFolder => {
                self.open_folder_dialog();
            }
            ClickAction::OpenSubtitle => {
                self.open_subtitle_dialog();
            }
            ClickAction::ShowInFiles => {
                if let Some(ref path) = self.state.media_path.clone() {
                    #[cfg(test)]
                    {
                        self.state.show_toast(format!("Show in files: {}", path));
                    }
                    #[cfg(not(test))]
                    {
                        if self.app_handle.is_none() {
                            // Test mode: show toast instead of opening file manager
                            self.state.show_toast(format!("Show in files: {}", path));
                        } else {
                            let p = path.clone();
                            std::thread::spawn(move || {
                                let _ = crate::platform_util::show_in_file_manager(&p);
                            });
                        }
                    }
                }
            }
            ClickAction::SeekRelative(delta) => {
                self.state.seek_by(*delta);
                if let Some(ref mut backend) = self.backend {
                    backend.seek(self.state.current_time);
                }
                ctx.request_paint();
            }
            ClickAction::SpeedUp => {
                self.state.speed_up();
                ctx.request_paint();
            }
            ClickAction::SpeedDown => {
                self.state.speed_down();
                ctx.request_paint();
            }
            ClickAction::ToggleSpeedMenu => {
                self.state.show_speed_menu = !self.state.show_speed_menu;
                ctx.request_paint();
            }
            ClickAction::ClosePanel => {
                self.state.close_all_panels();
                ctx.request_paint();
            }
            ClickAction::ToggleAiPanel => {
                self.state.ai_panel_open = !self.state.ai_panel_open;
                if self.state.ai_panel_open {
                    self.state.drawer = None;
                }
                ctx.request_paint();
            }
            ClickAction::ToggleGifCapture => {
                if self.gif_recording {
                    // Stop recording and encode GIF
                    self.gif_recording = false;
                    self.state.gif_state = "encoding".into();
                    let frames = std::mem::take(&mut self.gif_frames);
                    let dims = self.gif_dims;
                    if !frames.is_empty() && dims.0 > 0 && dims.1 > 0 {
                        let _handle = self.app_handle.clone();
                        std::thread::spawn(move || {
                            let result = encode_gif(&frames, dims);
                            if let Ok(_path) = result {}
                        });
                    }
                    self.state.gif_capture_open = !self.state.gif_capture_open;
                } else {
                    self.state.gif_capture_open = !self.state.gif_capture_open;
                    self.state.gif_state = if self.state.gif_capture_open {
                        "ready".into()
                    } else {
                        "idle".into()
                    };
                }
                ctx.request_paint();
            }
            ClickAction::StartGifRecord => {
                self.gif_recording = true;
                self.gif_frames.clear();
                self.gif_last_frame_ms = None;
                self.gif_recording_start = Some(std::time::Instant::now());
                self.state.gif_state = "recording".into();
                ctx.request_paint();
            }
            ClickAction::StopGifRecord => {
                self.gif_recording = false;
                self.gif_last_frame_ms = None;
                self.gif_recording_start = None;
                self.state.gif_state = "encoding".into();
                let frames = std::mem::take(&mut self.gif_frames);
                let dims = self.gif_dims;
                if !frames.is_empty() && dims.0 > 0 && dims.1 > 0 {
                    let out_path = capture_output_path("tench_capture", "gif");
                    let path_str = out_path.to_string_lossy().to_string();
                    std::thread::spawn(move || {
                        let _ = encode_gif_to_path(&frames, dims, &out_path);
                    });
                    self.state.show_toast(format!("GIF saved: {}", path_str));
                } else {
                    self.state.show_toast("No frames captured");
                }
                self.state.gif_capture_open = false;
                self.state.gif_state = "idle".into();
                ctx.request_paint();
            }
            ClickAction::Fullscreen => {
                if let Some(ref handle) = self.app_handle {
                    if let Some(wvw) = handle.get_webview_window("main") {
                        let is_fullscreen = wvw.is_fullscreen().unwrap_or(false);
                        let _ = wvw.set_fullscreen(!is_fullscreen);
                    }
                }
                ctx.request_paint();
            }
            ClickAction::ToggleTheme => {
                self.state.toggle_theme();
                ctx.request_paint();
            }
            ClickAction::CycleAspect => {
                self.state.cycle_aspect();
                ctx.request_paint();
            }
            ClickAction::CycleRepeat => {
                self.state.cycle_repeat_mode();
                ctx.request_paint();
            }
            ClickAction::ToggleShuffle => {
                self.state.toggle_shuffle();
                ctx.request_paint();
            }
            ClickAction::StepForward => {
                // Step forward by ~1 frame (1/30s at 30fps)
                self.state.seek_by(1.0 / 30.0);
                if let Some(ref mut backend) = self.backend {
                    backend.seek(self.state.current_time);
                }
                ctx.request_paint();
            }
            ClickAction::StepBackward => {
                // Step backward by ~1 frame
                self.state.seek_by(-1.0 / 30.0);
                if let Some(ref mut backend) = self.backend {
                    backend.seek(self.state.current_time);
                }
                ctx.request_paint();
            }
            ClickAction::SubtitleOffset(delta_ms) => {
                let offset_msg;
                if let Some(track) = self.state.subtitle_tracks.iter_mut().find(|t| t.active) {
                    track.offset_ms += delta_ms;
                    offset_msg = format!("Subtitle offset: {}ms", track.offset_ms);
                } else {
                    offset_msg = "No active subtitle track".to_string();
                }
                self.state.show_toast(offset_msg);
                self.state.update_subtitle_for_position();
                ctx.request_paint();
            }
            ClickAction::RemoveFromPlaylist(idx) => {
                self.state.remove_from_playlist(*idx);
                ctx.request_paint();
            }
            ClickAction::SelectBuiltinSubtitleTrack(idx) => {
                if let Some(ref mut backend) = self.backend {
                    if *idx < 0 {
                        // Disable built-in subtitles
                        backend.set_subtitle_track(0);
                    } else {
                        backend.set_subtitle_track(*idx);
                    }
                }
                self.state.active_builtin_subtitle_track = *idx;
                if *idx < 0 {
                    self.state.show_toast("Built-in subtitles disabled");
                } else {
                    self.state
                        .show_toast(format!("Built-in subtitle track {} selected", idx + 1));
                }
                ctx.request_paint();
            }
            ClickAction::SelectAudioDevice(name) => {
                if let Some(ref mut backend) = self.backend {
                    backend.set_audio_device(name);
                    self.state.selected_audio_device = Some(name.clone());
                    self.state.show_toast(format!("Audio device: {}", name));
                }
                ctx.request_paint();
            }
            ClickAction::PlayPlaylistItem(idx) => {
                if let Some(entry) = self.state.playlist.get(*idx) {
                    let path = entry.path.clone();
                    let title = entry.title.clone();
                    let duration = entry.duration;
                    self.state.current_playlist_index = Some(*idx);
                    self.load_media_for_action(&path, Some(title), Some(duration));
                }
                ctx.request_paint();
            }
            ClickAction::OpenRecentFile(idx) => {
                if let Some(entry) = self.state.recent_files.get(*idx) {
                    let path = entry.path.clone();
                    let title = entry.title.clone();
                    let duration = entry.duration;
                    self.load_media_for_action(&path, Some(title), Some(duration));
                }
                ctx.request_paint();
            }
            ClickAction::JumpToChapter(idx) => {
                let (time, title) = self
                    .state
                    .chapters
                    .get(*idx)
                    .map(|ch| (ch.time, ch.title.clone()))
                    .unwrap_or((0.0, String::new()));
                if !title.is_empty() {
                    self.state.seek_to(time);
                    if let Some(ref mut backend) = self.backend {
                        backend.seek(time);
                    }
                    self.state.show_toast(format!("Jumped to: {}", title));
                }
                ctx.request_paint();
            }
            ClickAction::JumpToRememberedPosition => {
                if let Some(pos) = self.state.remembered_position {
                    self.state.seek_to(pos);
                    if let Some(ref mut backend) = self.backend {
                        backend.seek(pos);
                    }
                    self.state.show_toast("Jumped to remembered position");
                }
                ctx.request_paint();
            }
            ClickAction::AddToPlaylist => {
                self.open_file_dialog();
            }
            ClickAction::SendAiPrompt(prompt) => {
                let prompt_text = prompt.clone();
                self.state.ai_chat_log.push(AiChatMessage {
                    role: AiMessageRole::User,
                    text: prompt_text.clone(),
                });
                self.state.ai_request_pending = true;
                self.state.ai_input_text.clear();
                self.state.ai_chat_log.push(AiChatMessage {
                    role: AiMessageRole::System,
                    text: "Tench Engine IPC is not configured for Player on this build."
                        .to_string(),
                });
                self.state.ai_request_pending = false;
                ctx.request_paint();
            }
            ClickAction::CancelAiRequest => {
                self.state.ai_request_pending = false;
                self.state.show_toast("AI request cancelled");
                ctx.request_paint();
            }
            ClickAction::ToggleSubtitleStyle => {
                self.state.subtitle_style_open = !self.state.subtitle_style_open;
                ctx.request_paint();
            }
            ClickAction::OpenSubtitleSearch => {
                self.state.subtitle_search_open = true;
                self.state.subtitle_search_text.clear();
                self.state.subtitle_search_results.clear();
                ctx.request_paint();
            }
            ClickAction::SearchSubtitleNext => {
                self.state.subtitle_search_next();
                if let Some(ref mut backend) = self.backend {
                    backend.seek(self.state.current_time);
                }
                ctx.request_paint();
            }
            ClickAction::SearchSubtitlePrev => {
                self.state.subtitle_search_prev();
                if let Some(ref mut backend) = self.backend {
                    backend.seek(self.state.current_time);
                }
                ctx.request_paint();
            }
            ClickAction::SetSubtitleEncoding(enc_name) => {
                self.state.subtitle_encoding = match enc_name.as_str() {
                    "Shift-JIS" | "shift-jis" => SubtitleEncoding::ShiftJIS,
                    "EUC-KR" | "euc-kr" => SubtitleEncoding::EucKR,
                    "EUC-JP" | "euc-jp" => SubtitleEncoding::EucJP,
                    "ISO-8859-1" | "iso-8859-1" => SubtitleEncoding::Iso8859_1,
                    "CP1252" | "Windows-1252" | "windows-1252" => SubtitleEncoding::Cp1252,
                    _ => SubtitleEncoding::Utf8,
                };
                self.state.show_toast(format!(
                    "Encoding: {}",
                    self.state.subtitle_encoding.label()
                ));
                // Reload subtitles with new encoding if path is known
                ctx.request_paint();
            }
            ClickAction::OpenUrl => {
                self.state.url_input_open = true;
                self.state.url_input_text.clear();
                ctx.request_paint();
            }
            ClickAction::SubmitUrl => {
                let url = self.state.url_input_text.trim().to_string();
                if !url.is_empty() {
                    self.load_media_for_action(&url, Some(url.clone()), Some(0.0));
                }
                self.state.url_input_open = false;
                self.state.url_input_focused = false;
                ctx.request_paint();
            }
            ClickAction::CancelUrl => {
                self.state.url_input_open = false;
                ctx.request_paint();
            }
            ClickAction::TogglePip => {
                self.state.pip_mode = !self.state.pip_mode;
                self.state.show_toast(if self.state.pip_mode {
                    "Picture-in-picture on"
                } else {
                    "Picture-in-picture off"
                });
                ctx.request_paint();
            }
            ClickAction::ToggleEqualizer => {
                self.state.eq_open = !self.state.eq_open;
                ctx.request_paint();
            }
            ClickAction::SetEqBand(band_idx, value) => {
                if *band_idx < 5 {
                    self.state.eq_bands[*band_idx] = *value;
                    if let Some(ref mut backend) = self.backend {
                        backend.set_eq_bands(&self.state.eq_bands);
                    }
                }
                ctx.request_paint();
            }
            ClickAction::SetEqPreset(preset_idx) => {
                if *preset_idx < EqPreset::PRESETS.len() {
                    self.state.eq_preset_idx = *preset_idx;
                    self.state.eq_bands = EqPreset::PRESETS[*preset_idx].bands;
                    if let Some(ref mut backend) = self.backend {
                        backend.set_eq_bands(&self.state.eq_bands);
                    }
                    self.state
                        .show_toast(format!("EQ: {}", EqPreset::PRESETS[*preset_idx].name));
                }
                ctx.request_paint();
            }
            ClickAction::ToggleNormalization => {
                self.state.normalization_enabled = !self.state.normalization_enabled;
                self.state.show_toast(if self.state.normalization_enabled {
                    "Audio normalization on"
                } else {
                    "Audio normalization off"
                });
                // Audio normalization is applied via the GStreamer audio-filter chain.
                // When enabled, the backend will use an audiocheblimit or similar element.
                // For now the state toggle is persisted and the EQ bin handles it on next load.
                ctx.request_paint();
            }
            ClickAction::DeleteChapter(idx) => {
                self.state.delete_chapter(*idx);
                ctx.request_paint();
            }
            ClickAction::RenameChapter(idx) => {
                self.state.chapter_rename_idx = Some(*idx);
                self.state.chapter_rename_text = self
                    .state
                    .chapters
                    .get(*idx)
                    .map(|c| c.title.clone())
                    .unwrap_or_default();
                ctx.request_paint();
            }
            ClickAction::ShowAddChapterModal => {
                self.state.show_add_chapter_modal = true;
                self.state.chapter_name_input.clear();
                self.state.chapter_name_input_focused = true;
                ctx.request_paint();
            }
            ClickAction::ExportChapters => {
                let json = self.state.export_chapters_json();
                if let Some(ref handle) = self.app_handle {
                    use tauri_plugin_dialog::DialogExt;
                    let Some(tx) = dialog_sender() else { return };
                    let tx = tx.clone();
                    handle
                        .dialog()
                        .file()
                        .add_filter("JSON", &["json"])
                        .set_file_name("chapters.json")
                        .save_file(move |path| {
                            if let Some(p) = path {
                                let _ = std::fs::write(
                                    p.as_path()
                                        .unwrap_or_else(|| panic!("URL path not supported")),
                                    json.clone(),
                                );
                                let _ =
                                    tx.send(DialogResult::File("Chapters exported".to_string()));
                            }
                        });
                } else {
                    self.state.show_toast("Chapters exported (test)");
                }
            }
            ClickAction::ImportChapters => {
                if let Some(ref handle) = self.app_handle {
                    use tauri_plugin_dialog::DialogExt;
                    let Some(tx) = dialog_sender() else { return };
                    let tx = tx.clone();
                    handle
                        .dialog()
                        .file()
                        .add_filter("JSON", &["json"])
                        .set_title("Import Chapters")
                        .pick_file(move |path| {
                            if let Some(p) = path {
                                if let Ok(path) = p.into_path() {
                                    if let Ok(data) = std::fs::read_to_string(path) {
                                        let _ = tx.send(DialogResult::File(format!(
                                            "import_chapters:{}",
                                            data
                                        )));
                                    }
                                }
                            }
                        });
                } else {
                    #[cfg(test)]
                    if let Some(json) = self.test_import_chapters_json.take() {
                        self.state.import_chapters_json(&json);
                        self.state.show_toast("Chapters imported (test)");
                    }
                }
            }
            ClickAction::ConfirmChapterRename => {
                if let Some(idx) = self.state.chapter_rename_idx {
                    let name = self.state.chapter_rename_text.clone();
                    self.state.rename_chapter(idx, name);
                    self.state.chapter_rename_idx = None;
                    self.state.show_toast("Chapter renamed");
                }
                ctx.request_paint();
            }
            ClickAction::CancelChapterRename => {
                self.state.chapter_rename_idx = None;
                ctx.request_paint();
            }
            ClickAction::ShowHelp => {
                self.state.help_open = !self.state.help_open;
                ctx.request_paint();
            }
            ClickAction::CancelSubtitleSearch => {
                self.state.subtitle_search_open = false;
                ctx.request_paint();
            }
            ClickAction::SubmitSubtitleSearch => {
                self.state
                    .search_subtitles(&self.state.subtitle_search_text.clone());
                if let Some(ref mut backend) = self.backend {
                    backend.seek(self.state.current_time);
                }
                ctx.request_paint();
            }
            ClickAction::GifOptions => {
                self.state.gif_options_open = !self.state.gif_options_open;
                ctx.request_paint();
            }
            ClickAction::SetGifFps(fps) => {
                self.state.gif_options.fps = *fps;
                ctx.request_paint();
            }
            ClickAction::SetGifMaxDuration(secs) => {
                self.state.gif_options.max_duration_secs = *secs;
                ctx.request_paint();
            }
            ClickAction::StartGifWithOptions => {
                self.gif_recording = true;
                self.gif_frames.clear();
                self.gif_last_frame_ms = None;
                self.gif_recording_start = Some(std::time::Instant::now());
                self.state.gif_state = "recording".into();
                self.state.gif_options_open = false;
                self.state.gif_capture_open = false;
                ctx.request_paint();
            }
            ClickAction::SeekToPercent(pct) => {
                let target = self.state.duration * pct;
                self.state.seek_to(target);
                if let Some(ref mut backend) = self.backend {
                    backend.seek(target);
                }
                ctx.request_paint();
            }
            ClickAction::ShowSubtitleSearch => {
                self.state.subtitle_search_open = true;
                self.state.subtitle_search_focused = true;
                self.state.subtitle_search_text.clear();
                ctx.request_paint();
            }
            ClickAction::ShowSubtitleStyle => {
                self.state.subtitle_style_open = true;
                ctx.request_paint();
            }
            ClickAction::FocusUrlInput => {
                self.state.url_input_focused = true;
                self.state.ai_focused = false;
                self.state.subtitle_search_focused = false;
                self.state.chapter_name_input_focused = false;
                ctx.request_paint();
            }
            ClickAction::FocusSubtitleSearch => {
                self.state.subtitle_search_focused = true;
                self.state.ai_focused = false;
                self.state.url_input_focused = false;
                self.state.chapter_name_input_focused = false;
                ctx.request_paint();
            }
            ClickAction::FocusChapterNameInput => {
                self.state.chapter_name_input_focused = true;
                self.state.ai_focused = false;
                self.state.url_input_focused = false;
                self.state.subtitle_search_focused = false;
                ctx.request_paint();
            }
            ClickAction::CloseModal => {
                self.state.help_open = false;
                self.state.url_input_open = false;
                self.state.subtitle_style_open = false;
                self.state.subtitle_search_open = false;
                self.state.gif_options_open = false;
                self.state.eq_open = false;
                self.state.show_add_chapter_modal = false;
                self.state.ai_focused = false;
                self.state.url_input_focused = false;
                self.state.subtitle_search_focused = false;
                self.state.chapter_name_input_focused = false;
                ctx.request_paint();
            }
            ClickAction::SubtitleOffsetForTrack(idx, delta_ms) => {
                let new_offset = if let Some(track) = self.state.subtitle_tracks.get_mut(*idx) {
                    track.offset_ms += delta_ms;
                    track.offset_ms
                } else {
                    0
                };
                self.state
                    .show_toast(format!("Track {} offset: {}ms", idx + 1, new_offset));
                self.state.update_subtitle_for_position();
                ctx.request_paint();
            }
            ClickAction::SetEqPresetNamed(name) => {
                let preset = EqPreset::PRESETS
                    .iter()
                    .find(|p| p.name == name.as_str())
                    .map(|p| (*p).clone())
                    .unwrap_or(EqPreset {
                        name: "Flat",
                        bands: [0.0; 5],
                    });
                self.state.eq_bands = preset.bands;
                if let Some(ref mut backend) = self.backend {
                    backend.set_eq_bands(&self.state.eq_bands);
                }
                self.state.show_toast(format!("EQ: {}", preset.name));
                ctx.request_paint();
            }
            ClickAction::StartGifRecording => {
                self.gif_recording = true;
                self.gif_frames.clear();
                self.gif_last_frame_ms = None;
                self.gif_recording_start = Some(std::time::Instant::now());
                self.state.gif_state = "recording".into();
                self.state.gif_options_open = false;
                ctx.request_paint();
            }
            ClickAction::ConfirmAddChapter => {
                let name = if self.state.chapter_name_input.is_empty() {
                    format!("Chapter {}", self.state.chapters.len() + 1)
                } else {
                    std::mem::take(&mut self.state.chapter_name_input)
                };
                self.state.add_chapter_at_current(name);
                self.state.show_add_chapter_modal = false;
                self.state.chapter_name_input_focused = false;
                self.state.show_toast("Chapter added");
                ctx.request_paint();
            }
            ClickAction::PlayNext => {
                self.state.next_track();
                if let Some(entry) = self
                    .state
                    .current_playlist_index
                    .and_then(|idx| self.state.playlist.get(idx))
                {
                    let path = entry.path.clone();
                    self.load_and_play(&path);
                }
                ctx.request_paint();
            }
            ClickAction::PlayPrevious => {
                self.state.prev_track();
                if let Some(entry) = self
                    .state
                    .current_playlist_index
                    .and_then(|idx| self.state.playlist.get(idx))
                {
                    let path = entry.path.clone();
                    self.load_and_play(&path);
                }
                ctx.request_paint();
            }
            ClickAction::FocusAiInput => {
                self.state.ai_focused = true;
                self.state.url_input_focused = false;
                self.state.subtitle_search_focused = false;
                self.state.chapter_name_input_focused = false;
                ctx.request_paint();
            }
            ClickAction::AdjustSubtitleStyle(prop_idx, delta) => {
                match *prop_idx {
                    0 => {
                        self.state.subtitle_style.font_size =
                            (self.state.subtitle_style.font_size + delta).clamp(8.0, 72.0)
                    }
                    1 => {
                        // font_family: cycle through available families
                        let families = ["sans-serif", "serif", "monospace"];
                        let current = &self.state.subtitle_style.font_family;
                        let next_idx = families
                            .iter()
                            .position(|f| *f == current.as_str())
                            .map(|i| (i + 1) % families.len())
                            .unwrap_or(0);
                        self.state.subtitle_style.font_family = families[next_idx].to_string();
                    }
                    2 => {
                        // text_color: cycle through preset colors
                        let colors = [
                            Color::WHITE,
                            Color::rgb8(255, 255, 0),
                            Color::rgba8(0, 255, 128, 255),
                            Color::rgba8(128, 200, 255, 255),
                            Color::rgba8(255, 180, 128, 255),
                        ];
                        let current = self.state.subtitle_style.text_color;
                        let next_idx = colors
                            .iter()
                            .position(|c| {
                                (c.r(), c.g(), c.b()) == (current.r(), current.g(), current.b())
                            })
                            .map(|i| (i + 1) % colors.len())
                            .unwrap_or(0);
                        self.state.subtitle_style.text_color = colors[next_idx];
                    }
                    3 => {
                        self.state.subtitle_style.bg_opacity =
                            (self.state.subtitle_style.bg_opacity + delta / 100.0).clamp(0.0, 1.0)
                    }
                    4 => {
                        self.state.subtitle_style.position =
                            (self.state.subtitle_style.position + delta / 100.0).clamp(0.0, 1.0)
                    }
                    5 => {
                        self.state.subtitle_style.stroke_width =
                            (self.state.subtitle_style.stroke_width + delta).clamp(0.0, 5.0)
                    }
                    6 => {
                        self.state.subtitle_style.shadow_offset =
                            (self.state.subtitle_style.shadow_offset + delta).clamp(0.0, 10.0)
                    }
                    _ => {}
                }
                ctx.request_paint();
            }
        }
    }

    /// Handle context menu item selection.
    pub(crate) fn handle_context_menu_action(&mut self, id: &str, ctx: &mut EventCtx) {
        match id {
            "play_pause" => self.dispatch_click_action(&ClickAction::PlayPause, ctx),
            "stop" => {
                if let Some(ref mut backend) = self.backend {
                    backend.stop();
                }
                self.state.is_playing = false;
                ctx.request_paint();
            }
            "screenshot" => self.dispatch_click_action(&ClickAction::Screenshot, ctx),
            "fullscreen" => self.dispatch_click_action(&ClickAction::Fullscreen, ctx),
            "open_file" => self.open_file_dialog(),
            "show_in_files" => self.dispatch_click_action(&ClickAction::ShowInFiles, ctx),
            "cycle_aspect" => {
                self.state.cycle_aspect();
                ctx.request_paint();
            }
            "cycle_repeat" => {
                self.state.cycle_repeat_mode();
                ctx.request_paint();
            }
            "toggle_shuffle" => {
                self.state.toggle_shuffle();
                ctx.request_paint();
            }
            _ => {}
        }
    }
}
