//! System media transport controls for macOS and Windows.
//!
//! Uses the `souvlaki` crate to integrate with the OS-level media control
//! infrastructure (MPNowPlayingInfoCenter on macOS, SystemMediaTransportControls
//! on Windows).

#![cfg(any(target_os = "macos", target_os = "windows"))]

use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig};
use std::sync::{Arc, Mutex};

/// Shared state that the UI updates and system media reads.
pub struct SystemMediaState {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_secs: f64,
    pub position_secs: f64,
    pub is_playing: bool,
    pub command_tx: std::sync::mpsc::Sender<SystemMediaCommand>,
}

/// Commands from system media controls to the player.
#[derive(Debug, Clone)]
pub enum SystemMediaCommand {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    Seek(f64),
}

/// Start the system media controls.
/// Returns a shared state handle that the UI should update.
pub fn start_system_media() -> (
    Arc<Mutex<SystemMediaState>>,
    std::sync::mpsc::Receiver<SystemMediaCommand>,
) {
    let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();
    let state = Arc::new(Mutex::new(SystemMediaState {
        title: String::new(),
        artist: String::new(),
        album: String::new(),
        duration_secs: 0.0,
        position_secs: 0.0,
        is_playing: false,
        command_tx: cmd_tx,
    }));

    let state_clone = state.clone();
    std::thread::spawn(move || {
        if let Err(e) = run_system_media_loop(state_clone) {
            eprintln!("System media control error: {e}");
        }
    });

    (state, cmd_rx)
}

fn run_system_media_loop(
    state: Arc<Mutex<SystemMediaState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let hwnd = None; // souvlaki can work without a window handle on macOS
    let config = PlatformConfig {
        dbus_name: "tench-player",
        display_name: "Tench Player",
        hwnd,
    };

    let mut controls = MediaControls::new(config)?;

    let cmd_tx = {
        let s = state.lock().unwrap();
        s.command_tx.clone()
    };

    controls.attach(move |event: MediaControlEvent| {
        let cmd = match event {
            MediaControlEvent::Play => SystemMediaCommand::Play,
            MediaControlEvent::Pause => SystemMediaCommand::Pause,
            MediaControlEvent::Toggle => SystemMediaCommand::PlayPause,
            MediaControlEvent::Next => SystemMediaCommand::Next,
            MediaControlEvent::Previous => SystemMediaCommand::Previous,
            MediaControlEvent::Stop => SystemMediaCommand::Stop,
            MediaControlEvent::Seek(direction) => {
                let offset = match direction {
                    souvlaki::SeekDirection::Forward => 10.0,
                    souvlaki::SeekDirection::Backward => -10.0,
                };
                SystemMediaCommand::Seek(offset)
            }
            _ => return,
        };
        let _ = cmd_tx.send(cmd);
    })?;

    // Periodically update metadata and playback state
    loop {
        {
            let s = state.lock().unwrap();
            let playback = if s.is_playing {
                MediaPlayback::Playing { progress: None }
            } else {
                MediaPlayback::Paused { progress: None }
            };

            let metadata = MediaMetadata {
                title: Some(&s.title),
                artist: Some(&s.artist),
                album: Some(&s.album),
                duration: Some(std::time::Duration::from_secs_f64(s.duration_secs)),
                cover_url: None,
            };

            let _ = controls.set_playback(playback);
            let _ = controls.set_metadata(metadata);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
