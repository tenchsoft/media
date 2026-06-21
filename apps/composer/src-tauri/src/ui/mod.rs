//! Composer app UI - Adobe-style video editor backed by tench-composer-core.
//!
//! Module layout:
//! - `state` — UI state model and actions
//! - `preview` — preview monitor geometry helpers
//! - `timeline` — timeline geometry helpers
//! - `inspector` — inspector tab helpers
//! - `toolbar` — top toolbar paint
//! - `left_panel` — media bin, templates, effects, transitions, subtitle editor
//! - `preview_panel` — preview monitor paint
//! - `right_panel` — inspector paint
//! - `timeline_panel` — timeline, render queue, AI panel paint

mod automation;
pub mod inspector;
pub mod left_panel;
pub mod preview;
pub mod preview_panel;
pub mod right_panel;
pub mod state;
pub mod timeline;
pub mod timeline_panel;
pub mod toolbar;
mod widget;

use std::sync::{mpsc, OnceLock};

use tench_composer_core::*;
use tench_ui::prelude::*;

use state::{ClickAction, ComposerInputFocus, ComposerState, DragKind, SplitterTarget};

// ---------------------------------------------------------------------------
// Dialog channel (same pattern as Player)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum DialogResult {
    FileOpened(String),
    FilesOpened(Vec<String>),
    FolderOpened(String),
    SavePath(String),
    Cancelled,
}

static DIALOG_SENDER: OnceLock<mpsc::Sender<DialogResult>> = OnceLock::new();
static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

/// Send a dialog result from a Tauri command into the UI loop.
pub fn send_dialog_result(result: DialogResult) {
    if let Some(tx) = DIALOG_SENDER.get() {
        let _ = tx.send(result);
    }
}

/// Open a native file dialog for media import.
///
/// Spawns a background thread so the UI is not blocked while the dialog
/// is open. Selected file paths arrive through [`DIALOG_SENDER`] as
/// [`DialogResult::FilesOpened`] and are processed by
/// [`ComposerApp::process_dialog_results`].
fn request_media_import() {
    let handle = match APP_HANDLE.get() {
        Some(h) => h.clone(),
        None => return,
    };
    let sender = match DIALOG_SENDER.get() {
        Some(s) => s.clone(),
        None => return,
    };
    std::thread::spawn(move || {
        use tauri_plugin_dialog::DialogExt;
        let result = handle
            .dialog()
            .file()
            .add_filter(
                "Media",
                &[
                    "mp4", "avi", "mov", "mkv", "webm", "mp3", "wav", "flac", "jpg", "png",
                ],
            )
            .blocking_pick_files();
        match result {
            Some(paths) => {
                let path_strs: Vec<String> = paths.iter().map(|p| p.to_string()).collect();
                let _ = sender.send(DialogResult::FilesOpened(path_strs));
            }
            None => {
                let _ = sender.send(DialogResult::Cancelled);
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Click region system
// ---------------------------------------------------------------------------

pub(crate) struct ClickRegion {
    rect: Rect,
    action: ClickAction,
}

// ---------------------------------------------------------------------------
// ComposerApp widget
// ---------------------------------------------------------------------------

pub struct ComposerApp {
    state: ComposerState,
    click_regions: Vec<ClickRegion>,
    dialog_rx: Option<mpsc::Receiver<DialogResult>>,
    text_cache: TextCache,
    has_media: bool,
    /// Test injection: next media path for import dialog.
    test_next_media: Option<String>,
}

impl ComposerApp {
    // new_without_default: ComposerApp sets up channel communication in new()
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let _ = DIALOG_SENDER.set(tx);
        Self {
            state: ComposerState::new(),
            click_regions: Vec::new(),
            dialog_rx: Some(rx),
            text_cache: TextCache::new(),
            has_media: false,
            test_next_media: None,
        }
    }

    pub fn with_state(state: ComposerState) -> Self {
        let (tx, rx) = mpsc::channel();
        let _ = DIALOG_SENDER.set(tx);
        Self {
            state,
            click_regions: Vec::new(),
            dialog_rx: Some(rx),
            text_cache: TextCache::new(),
            has_media: true,
            test_next_media: None,
        }
    }

    /// Mutable access to the internal ComposerState (for testing).
    pub fn state_mut(&mut self) -> &mut ComposerState {
        &mut self.state
    }

    /// Inject a media file path for the next import dialog (test only).
    pub fn inject_test_media(&mut self, path: String) {
        self.test_next_media = Some(path);
    }

    pub fn set_app_handle(&self, handle: tauri::AppHandle) {
        let _ = APP_HANDLE.set(handle);
    }

    // -- Click region helpers --

    fn click_action_at(&self, pos: Point) -> Option<&ClickAction> {
        self.click_regions
            .iter()
            .rev()
            .find(|r| r.rect.contains(pos))
            .map(|r| &r.action)
    }

    fn clear_click_regions(&mut self) {
        self.click_regions.clear();
    }

    // -- Dialog processing --

    fn process_dialog_results(&mut self) {
        let Some(rx) = &self.dialog_rx else { return };
        while let Ok(result) = rx.try_recv() {
            match result {
                DialogResult::FilesOpened(paths) => {
                    for path in paths {
                        self.state.import_media(path);
                    }
                    self.has_media = true;
                }
                DialogResult::FileOpened(path) => {
                    self.state.import_media(path);
                    self.has_media = true;
                }
                DialogResult::SavePath(path) => {
                    self.state.save_project_as(path);
                }
                _ => {}
            }
        }
    }

    // -- Dispatch click action --

    fn dispatch_click_action(&mut self, action: &ClickAction) {
        match action {
            ClickAction::PlayPause => self.state.toggle_playback(),
            ClickAction::StepForward => self.state.step_frame(1),
            ClickAction::StepBackward => self.state.step_frame(-1),
            ClickAction::SeekTo(frame) => self.state.seek_to_frame(*frame as u32),
            ClickAction::SplitAtPlayhead => {
                self.state.split_at_playhead();
            }
            ClickAction::DeleteClip(id) => {
                self.state.selected_clip_id = Some(*id);
                self.state.delete_selected_clip();
            }
            ClickAction::SelectMode(mode) => self.state.select_mode(*mode),
            ClickAction::SelectLeftTab(tab) => self.state.select_left_tab(*tab),
            ClickAction::SelectInspectorTab(idx) => self.state.select_inspector_tab(*idx),
            ClickAction::SelectClip(Some(id)) => {
                self.state.selected_clip_id = Some(*id);
            }
            ClickAction::SelectClip(None) => {
                self.state.selected_clip_id = None;
            }
            ClickAction::SelectTemplate(idx) => {
                self.state.select_template(*idx);
            }
            ClickAction::FocusEffectsSearch => {
                self.state.input_focus = ComposerInputFocus::EffectsSearch;
                self.state.subtitle_focused = false;
            }
            ClickAction::FocusTransitionsSearch => {
                self.state.input_focus = ComposerInputFocus::TransitionsSearch;
                self.state.subtitle_focused = false;
            }
            ClickAction::ApplyEffect(effect) => {
                if let Some(clip_id) = self.state.selected_clip_id {
                    self.state.add_effect_to_clip(clip_id, *effect);
                } else {
                    self.state
                        .set_notice(format!("{} selected", effect.label()));
                }
            }
            ClickAction::ApplyTransition(transition) => {
                self.state
                    .set_notice(format!("{} transition selected", transition.label()));
            }
            ClickAction::RunAiFeature(name) => {
                self.state.set_notice(format!("{name} queued"));
            }
            ClickAction::ImportMedia => {
                if let Some(path) = self.test_next_media.take() {
                    self.state.import_media(path);
                    self.has_media = true;
                } else {
                    request_media_import();
                }
            }
            ClickAction::Export => {
                self.state.enqueue_render();
            }
            ClickAction::ToggleRenderQueue => {
                self.state.show_render_queue = !self.state.show_render_queue;
            }
            ClickAction::ToggleAiPanel => {
                self.state.show_ai_panel = !self.state.show_ai_panel;
            }
            ClickAction::CloseRenderQueue => {
                self.state.show_render_queue = false;
            }
            ClickAction::CancelRenderJob(id) => {
                if let Some(job) = self
                    .state
                    .project
                    .render_queue
                    .iter_mut()
                    .find(|j| j.id == *id)
                {
                    job.status = RenderStatus::Failed;
                    self.state.set_notice("Render cancelled");
                }
            }
            ClickAction::PauseRenderJob(id) => {
                if let Some(job) = self
                    .state
                    .project
                    .render_queue
                    .iter_mut()
                    .find(|j| j.id == *id)
                {
                    job.status = RenderStatus::Queued;
                    self.state.set_notice("Render paused");
                }
            }
            ClickAction::ToggleSnap => self.state.snap = !self.state.snap,
            ClickAction::ToggleRipple => self.state.ripple = !self.state.ripple,
            ClickAction::ToggleMagnet => self.state.magnetic = !self.state.magnetic,
            ClickAction::ZoomIn => self.state.zoom = (self.state.zoom + 10.0).min(200.0),
            ClickAction::ZoomOut => self.state.zoom = (self.state.zoom - 10.0).max(10.0),
            ClickAction::ToggleTrackMute(id) => {
                if let Some(track) = self.state.project.timeline.track_mut(*id) {
                    track.muted = !track.muted;
                }
            }
            ClickAction::ToggleTrackLock(id) => {
                if let Some(track) = self.state.project.timeline.track_mut(*id) {
                    track.locked = !track.locked;
                }
            }
            ClickAction::ToggleTrackHidden(id) => {
                if let Some(track) = self.state.project.timeline.track_mut(*id) {
                    track.hidden = !track.hidden;
                }
            }
            ClickAction::AddTrack(kind) => {
                self.state.add_track(*kind);
            }
            ClickAction::DeleteTrack(id) => {
                self.state.delete_track(*id);
            }
            ClickAction::MoveClip {
                clip_id,
                source_track,
                dest_track,
                new_timeline_in,
            } => {
                self.state
                    .move_clip(*clip_id, *source_track, *dest_track, *new_timeline_in);
            }
            ClickAction::TrimClipIn {
                clip_id,
                track_id,
                new_in,
            } => {
                self.state.trim_clip_in(*clip_id, *track_id, *new_in);
            }
            ClickAction::TrimClipOut {
                clip_id,
                track_id,
                new_out,
            } => {
                self.state.trim_clip_out(*clip_id, *track_id, *new_out);
            }
            ClickAction::DropMediaOnTrack {
                media_idx,
                track_id,
                timeline_in,
            } => {
                self.state
                    .drop_media_on_track(*media_idx, *track_id, *timeline_in);
            }
            ClickAction::CutClip(id) => {
                self.state.cut_clip(*id);
            }
            ClickAction::CopyClip(id) => {
                self.state.copy_clip(*id);
            }
            ClickAction::PasteClip => {
                self.state.paste_clip();
            }
            ClickAction::DuplicateClip(id) => {
                self.state.duplicate_clip(*id);
            }
            ClickAction::FocusSubtitleEditor => {
                self.state.subtitle_focused = !self.state.subtitle_focused;
                self.state.input_focus = if self.state.subtitle_focused {
                    ComposerInputFocus::Subtitle
                } else {
                    ComposerInputFocus::None
                };
            }
            ClickAction::SetClipName(clip_id, name) => {
                if let Some(clip) = self.state.find_clip_mut(*clip_id) {
                    clip.name = name.clone();
                    self.state.set_notice("Clip renamed");
                }
            }
            ClickAction::SetClipSpeed(clip_id, speed) => {
                if let Some(clip) = self.state.find_clip_mut(*clip_id) {
                    clip.speed = *speed;
                    self.state.set_notice("Clip speed changed");
                }
            }
            ClickAction::ToggleClipReversed(clip_id) => {
                if let Some(clip) = self.state.find_clip_mut(*clip_id) {
                    clip.reversed = !clip.reversed;
                    self.state.set_notice("Clip reverse toggled");
                }
            }
            ClickAction::SetTrackVolume(track_id, volume) => {
                if let Some(track) = self.state.project.timeline.track_mut(*track_id) {
                    track.volume = *volume;
                    self.state.set_notice("Track volume changed");
                }
            }
            ClickAction::SetTrackPan(track_id, pan) => {
                if let Some(track) = self.state.project.timeline.track_mut(*track_id) {
                    track.pan = *pan;
                    self.state.set_notice("Track pan changed");
                }
            }
            ClickAction::ToggleTrackMuted(track_id) => {
                if let Some(track) = self.state.project.timeline.track_mut(*track_id) {
                    track.muted = !track.muted;
                    self.state.set_notice("Track muted toggled");
                }
            }
            ClickAction::SetExportFormat(format) => {
                self.state.project.export_settings.format = *format;
                self.state.set_notice("Export format changed");
            }
            ClickAction::SetExportCodec(codec) => {
                self.state.project.export_settings.codec = *codec;
                self.state.set_notice("Export codec changed");
            }
            ClickAction::SetExportResolution(width, height) => {
                self.state.project.export_settings.width = *width;
                self.state.project.export_settings.height = *height;
                self.state.set_notice("Export resolution changed");
            }
            ClickAction::SetExportFps(fps) => {
                self.state.project.export_settings.fps = *fps;
                self.state.set_notice("Export FPS changed");
            }
            ClickAction::SetExportBitrate(bitrate) => {
                self.state.project.export_settings.bitrate_kbps = *bitrate;
                self.state.set_notice("Export bitrate changed");
            }
            _ => {}
        }
    }

    // -- Handle pointer down for drag start --

    fn handle_pointer_down(&mut self, pos: Point, size: Size) {
        let left_w = self.state.left_panel_w;
        let timeline_h = self.state.timeline_h;
        let toolbar_h = 48.0;
        let tl_y = size.height - timeline_h;

        // Check splitter hit zones (5px strips at panel edges)
        // Left panel right edge splitter
        let splitter_zone = 5.0;
        if (pos.x - left_w).abs() < splitter_zone && pos.y > toolbar_h && pos.y < tl_y {
            self.state.drag = Some(DragKind::Splitter(SplitterTarget::LeftPanel));
            self.state.drag_start_pos = Some(pos);
            return;
        }
        let right_w = self.state.right_panel_w;
        let right_edge = size.width - right_w;
        if (pos.x - right_edge).abs() < splitter_zone && pos.y > toolbar_h && pos.y < tl_y {
            self.state.drag = Some(DragKind::Splitter(SplitterTarget::RightPanel));
            self.state.drag_start_pos = Some(pos);
            return;
        }
        if (pos.y - tl_y).abs() < splitter_zone {
            self.state.drag = Some(DragKind::Splitter(SplitterTarget::Timeline));
            self.state.drag_start_pos = Some(pos);
            return;
        }

        // Timeline area: check for clip drag start
        if pos.y >= tl_y && pos.x >= left_w {
            let tl_content_w = timeline::content_width(size.width, left_w);
            let total_frames = self.state.total_frames();
            if let Some(track_idx) =
                timeline::hit_test_track(pos, tl_y, timeline_h, self.state.tracks().len())
            {
                // Collect clip info without holding a borrow on self.state
                let clip_info: Vec<(ClipId, TrackId, u32, u32)> = {
                    if let Some(track) = self.state.tracks().get(track_idx) {
                        track
                            .clips
                            .iter()
                            .map(|c| (c.id, track.id, c.timeline_in, c.timeline_out()))
                            .collect()
                    } else {
                        Vec::new()
                    }
                };

                let track_y = tl_y
                    + timeline::HEADER_H
                    + timeline::TOOLBAR_H
                    + (track_idx as f64)
                        * timeline::track_height(timeline_h, self.state.tracks().len());
                let th = timeline::track_height(timeline_h, self.state.tracks().len());

                for (clip_id, track_id, tl_in, tl_out) in clip_info {
                    let crect = timeline::clip_rect(
                        left_w,
                        tl_content_w,
                        total_frames,
                        track_y,
                        th,
                        tl_in,
                        tl_out,
                    );

                    // Check trim handle first
                    if let Some(is_in) = timeline::hit_test_trim_handle(pos, crect) {
                        if is_in {
                            self.state.drag = Some(DragKind::TrimIn {
                                clip_id,
                                track_id,
                                original_in: tl_in,
                            });
                        } else {
                            self.state.drag = Some(DragKind::TrimOut {
                                clip_id,
                                track_id,
                                original_out: tl_out,
                            });
                        }
                        self.state.drag_start_pos = Some(pos);
                        self.state.selected_clip_id = Some(clip_id);
                        return;
                    }

                    // Clip body: start move drag
                    if crect.contains(pos) {
                        let frame_at_pos =
                            timeline::x_to_frame(pos.x, left_w, tl_content_w, total_frames);
                        let offset = tl_in as i32 - frame_at_pos as i32;
                        self.state.drag = Some(DragKind::Clip {
                            clip_id,
                            source_track_id: track_id,
                            offset_frames: offset,
                        });
                        self.state.drag_start_pos = Some(pos);
                        self.state.selected_clip_id = Some(clip_id);
                        return;
                    }
                }
            }
        }
    }

    // -- Handle pointer move during drag --

    fn handle_pointer_move(&mut self, pos: Point, size: Size) {
        let Some(ref drag) = self.state.drag else {
            return;
        };
        let _left_w = self.state.left_panel_w;
        let _timeline_h = self.state.timeline_h;

        match drag {
            DragKind::Splitter(SplitterTarget::LeftPanel) => {
                self.state.left_panel_w = pos.x.clamp(120.0, 400.0);
            }
            DragKind::Splitter(SplitterTarget::RightPanel) => {
                self.state.right_panel_w = (size.width - pos.x).clamp(180.0, 400.0);
            }
            DragKind::Splitter(SplitterTarget::Timeline) => {
                self.state.timeline_h = (size.height - pos.y).clamp(120.0, size.height * 0.6);
            }
            _ => {
                // Clip/trim drags: update cursor position for visual feedback.
                // The actual move/trim is applied on pointer up.
            }
        }
    }

    // -- Handle pointer up: finalize drag --

    fn handle_pointer_up(&mut self, pos: Point, size: Size) {
        let left_w = self.state.left_panel_w;
        let timeline_h = self.state.timeline_h;
        let tl_y = size.height - timeline_h;
        let tl_content_w = timeline::content_width(size.width, left_w);
        let total_frames = self.state.total_frames();

        if let Some(drag) = self.state.drag.take() {
            match drag {
                DragKind::Clip {
                    clip_id,
                    source_track_id,
                    offset_frames,
                } => {
                    // Determine destination track and new timeline_in
                    if let Some(track_idx) =
                        timeline::hit_test_track(pos, tl_y, timeline_h, self.state.tracks().len())
                    {
                        if let Some(dest_track) = self.state.tracks().get(track_idx) {
                            let dest_track_id = dest_track.id;
                            let frame =
                                timeline::x_to_frame(pos.x, left_w, tl_content_w, total_frames);
                            let new_in = if offset_frames >= 0 {
                                frame.saturating_add(offset_frames as u32)
                            } else {
                                frame.saturating_sub(offset_frames.unsigned_abs())
                            };
                            let snapped = self.state.snap_position(new_in, 5);
                            self.state
                                .move_clip(clip_id, source_track_id, dest_track_id, snapped);
                        }
                    }
                }
                DragKind::TrimIn {
                    clip_id,
                    track_id,
                    original_in: _,
                } => {
                    let new_in = timeline::x_to_frame(pos.x, left_w, tl_content_w, total_frames);
                    let snapped = self.state.snap_position(new_in, 3);
                    self.state.trim_clip_in(clip_id, track_id, snapped);
                }
                DragKind::TrimOut {
                    clip_id,
                    track_id,
                    original_out: _,
                } => {
                    let new_out = timeline::x_to_frame(pos.x, left_w, tl_content_w, total_frames);
                    let snapped = self.state.snap_position(new_out, 3);
                    self.state.trim_clip_out(clip_id, track_id, snapped);
                }
                DragKind::MediaBin { media_idx } => {
                    // Drop media on timeline track
                    if let Some(track_idx) =
                        timeline::hit_test_track(pos, tl_y, timeline_h, self.state.tracks().len())
                    {
                        if let Some(dest_track) = self.state.tracks().get(track_idx) {
                            let dest_track_id = dest_track.id;
                            let frame =
                                timeline::x_to_frame(pos.x, left_w, tl_content_w, total_frames);
                            self.state
                                .drop_media_on_track(media_idx, dest_track_id, frame);
                        }
                    }
                }
                _ => {}
            }
        }
        self.state.drag_start_pos = None;
    }
}
