mod actions;
mod automation;
mod backend;
mod dialogs;
mod widget;

use super::state::PlayerState;
use crate::DialogResult;

// ---------------------------------------------------------------------------
// PlayerApp widget
// ---------------------------------------------------------------------------

pub struct PlayerApp {
    state: PlayerState,
    backend: Option<crate::gst_backend::PlayerBackend>,
    backend_rx: Option<std::sync::mpsc::Receiver<crate::gst_backend::MediaEvent>>,
    /// Current video frame as peniko::ImageData for rendering.
    video_frame: Option<tench_ui::peniko::ImageData>,
    /// Video frame dimensions.
    video_dims: (u32, u32),
    app_handle: Option<tauri::AppHandle>,
    dialog_rx: Option<std::sync::mpsc::Receiver<DialogResult>>,
    /// Last click time for double-click detection.
    last_click_time: Option<std::time::Instant>,
    /// Whether we're currently dragging the seekbar.
    dragging_seek: bool,
    /// Whether we're currently dragging the volume bar.
    dragging_volume: bool,
    /// GIF capture state.
    gif_frames: Vec<Vec<u8>>,
    gif_dims: (u16, u16),
    gif_recording: bool,
    /// Timestamp of the last captured GIF frame (for FPS limiting).
    gif_last_frame_ms: Option<u64>,
    /// Timestamp when GIF recording started (for elapsed time indicator).
    gif_recording_start: Option<std::time::Instant>,
    /// Toast auto-dismiss timer.
    toast_time: Option<std::time::Instant>,
    /// Last rendered toast message (to detect changes and reset timer).
    last_toast: Option<String>,
    /// Seekbar hover position for thumbnail preview (0.0-1.0 fraction of seekbar).
    seek_hover_pos: Option<f64>,
    /// Cached thumbnail image for the current hover position.
    seek_thumbnail: Option<tench_ui::peniko::ImageData>,
    /// MPRIS state for Linux media key integration.
    #[cfg(target_os = "linux")]
    mpris_state: Option<std::sync::Arc<std::sync::Mutex<crate::mpris::MprisState>>>,
    /// MPRIS command receiver.
    #[cfg(target_os = "linux")]
    mpris_cmd_rx: Option<std::sync::mpsc::Receiver<crate::mpris::MprisCommand>>,
    /// System media state for macOS/Windows.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    system_media_state:
        Option<std::sync::Arc<std::sync::Mutex<crate::system_media::SystemMediaState>>>,
    /// System media command receiver.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    system_media_cmd_rx: Option<std::sync::mpsc::Receiver<crate::system_media::SystemMediaCommand>>,
    /// Test-only: files to inject when open_file_dialog is called.
    #[cfg(test)]
    test_next_files: Vec<String>,
    /// Test-only: chapters JSON to inject when ImportChapters is called.
    #[cfg(test)]
    test_import_chapters_json: Option<String>,
}

impl PlayerApp {
    // new_without_default: PlayerApp sets up platform-specific state in new()
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            state: PlayerState::new(),
            backend: None,
            backend_rx: None,
            video_frame: None,
            video_dims: (0, 0),
            app_handle: None,
            dialog_rx: None,
            last_click_time: None,
            dragging_seek: false,
            dragging_volume: false,
            gif_frames: Vec::new(),
            gif_dims: (0, 0),
            gif_recording: false,
            gif_last_frame_ms: None,
            gif_recording_start: None,
            toast_time: None,
            last_toast: None,
            seek_hover_pos: None,
            seek_thumbnail: None,
            #[cfg(target_os = "linux")]
            mpris_state: None,
            #[cfg(target_os = "linux")]
            mpris_cmd_rx: None,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            system_media_state: None,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            system_media_cmd_rx: None,
            #[cfg(test)]
            test_next_files: Vec::new(),
            #[cfg(test)]
            test_import_chapters_json: None,
        }
    }

    pub fn with_state(state: PlayerState) -> Self {
        Self {
            state,
            ..Self::new()
        }
    }

    pub fn state_mut(&mut self) -> &mut PlayerState {
        &mut self.state
    }

    /// Inject test files for the next open_file_dialog call.
    pub fn inject_test_files(&mut self, paths: Vec<String>) {
        #[cfg(test)]
        {
            self.test_next_files = paths;
        }
        #[cfg(not(test))]
        let _ = paths;
    }

    /// Inject chapters JSON for the next ImportChapters call.
    pub fn inject_test_chapters_json(&mut self, json: String) {
        #[cfg(test)]
        {
            self.test_import_chapters_json = Some(json);
        }
        #[cfg(not(test))]
        let _ = json;
    }

    /// Set the Tauri AppHandle for native dialogs.
    pub fn set_app_handle(&mut self, handle: tauri::AppHandle) {
        self.app_handle = Some(handle);
        // Initialize MPRIS on Linux
        #[cfg(target_os = "linux")]
        {
            let (state, cmd_rx) = crate::mpris::start_mpris_service();
            self.mpris_state = Some(state);
            self.mpris_cmd_rx = Some(cmd_rx);
        }
        // Initialize system media controls on macOS/Windows
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            let (state, cmd_rx) = crate::system_media::start_system_media();
            self.system_media_state = Some(state);
            self.system_media_cmd_rx = Some(cmd_rx);
        }
    }

    /// Set the dialog result receiver.
    pub fn set_dialog_receiver(&mut self, rx: std::sync::mpsc::Receiver<DialogResult>) {
        self.dialog_rx = Some(rx);
    }

    /// Set the GStreamer backend.
    pub fn set_backend(&mut self, backend: crate::gst_backend::PlayerBackend) {
        self.backend = Some(backend);
    }

    /// Set the backend event receiver.
    pub fn set_backend_event_receiver(
        &mut self,
        rx: std::sync::mpsc::Receiver<crate::gst_backend::MediaEvent>,
    ) {
        self.backend_rx = Some(rx);
    }
}
