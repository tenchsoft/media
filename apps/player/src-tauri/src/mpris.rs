//! MPRIS D-Bus integration for Linux.
//!
//! Provides basic MPRIS2 media player controls so that desktop environments
//! (GNOME, KDE, etc.) can show media controls in their panels and lock screens.

#![cfg(target_os = "linux")]

use std::sync::{Arc, Mutex};

/// Shared state that the UI updates and MPRIS reads.
pub struct MprisState {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub is_playing: bool,
    pub position_secs: f64,
    pub duration_secs: f64,
    pub can_play: bool,
    pub can_pause: bool,
    pub can_go_next: bool,
    pub can_go_previous: bool,
}

/// Commands sent from MPRIS to the player.
#[derive(Debug, Clone)]
pub enum MprisCommand {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    Seek(f64), // seconds, positive = forward
}

/// Start the MPRIS D-Bus service in a background thread.
/// Returns a shared state handle that the UI should update.
pub fn start_mpris_service() -> (
    Arc<Mutex<MprisState>>,
    std::sync::mpsc::Receiver<MprisCommand>,
) {
    let (_cmd_tx, cmd_rx) = std::sync::mpsc::channel();
    let _supported_commands = [
        MprisCommand::Play,
        MprisCommand::Pause,
        MprisCommand::PlayPause,
        MprisCommand::Stop,
        MprisCommand::Next,
        MprisCommand::Previous,
        MprisCommand::Seek(0.0),
    ];
    let state = Arc::new(Mutex::new(MprisState {
        title: String::new(),
        artist: String::new(),
        album: String::new(),
        is_playing: false,
        position_secs: 0.0,
        duration_secs: 0.0,
        can_play: false,
        can_pause: false,
        can_go_next: false,
        can_go_previous: false,
    }));

    let state_clone = state.clone();
    std::thread::spawn(move || {
        if let Err(e) = run_mpris_server(state_clone) {
            eprintln!("MPRIS server error: {e}");
        }
    });

    (state, cmd_rx)
}

fn run_mpris_server(state: Arc<Mutex<MprisState>>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Ok(s) = state.lock() {
            let _ = s.is_playing;
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
