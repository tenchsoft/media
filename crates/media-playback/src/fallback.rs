use std::sync::mpsc;

use crate::{MediaEvent, MediaMetadata};

pub struct PlayerBackend {
    event_tx: mpsc::Sender<MediaEvent>,
    volume: f64,
    muted: bool,
    playback_rate: f64,
    media_path: Option<String>,
    buffering_percent: u32,
    is_buffering: bool,
}

impl PlayerBackend {
    pub fn new() -> (Self, mpsc::Receiver<MediaEvent>) {
        let (event_tx, event_rx) = mpsc::channel();
        (
            Self {
                event_tx,
                volume: 1.0,
                muted: false,
                playback_rate: 1.0,
                media_path: None,
                buffering_percent: 100,
                is_buffering: false,
            },
            event_rx,
        )
    }

    pub fn load(&mut self, path: &str) {
        self.media_path = Some(path.to_string());
        let _ = self.event_tx.send(MediaEvent::Error(
            "GStreamer backend is not enabled for this build".to_string(),
        ));
    }

    pub fn play(&mut self) {}

    pub fn pause(&mut self) {}

    pub fn seek(&mut self, _position: f64) {}

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    pub fn set_playback_rate(&mut self, rate: f64) {
        self.playback_rate = rate.clamp(0.1, 4.0);
    }

    pub fn stop(&mut self) {
        self.media_path = None;
    }

    pub fn position(&self) -> f64 {
        0.0
    }

    pub fn duration(&self) -> f64 {
        0.0
    }

    pub fn is_playing(&self) -> bool {
        false
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }

    pub fn playback_rate(&self) -> f64 {
        self.playback_rate
    }

    pub fn has_media(&self) -> bool {
        self.media_path.is_some()
    }

    pub fn tick(&mut self) -> (f64, f64) {
        (0.0, 0.0)
    }

    pub fn query_metadata(&self) -> MediaMetadata {
        MediaMetadata::default()
    }

    pub fn buffering_percent(&self) -> u32 {
        self.buffering_percent
    }

    pub fn is_buffering(&self) -> bool {
        self.is_buffering
    }

    pub fn set_next_uri(&mut self, uri: String) {
        self.media_path = Some(uri);
    }

    pub fn n_audio_streams(&self) -> u32 {
        0
    }

    pub fn n_video_streams(&self) -> u32 {
        0
    }

    pub fn set_audio_track(&mut self, _index: i32) {}

    pub fn current_audio_track(&self) -> i32 {
        -1
    }

    pub fn set_video_track(&mut self, _index: i32) {}

    pub fn set_subtitle_track(&mut self, _index: i32) {}

    pub fn n_subtitle_streams(&self) -> u32 {
        0
    }

    pub fn enumerate_audio_devices() -> Vec<(String, String)> {
        Vec::new()
    }

    pub fn set_audio_device(&mut self, _device_name: &str) {}

    pub fn query_chapters(&self) -> Vec<(String, f64)> {
        Vec::new()
    }

    pub fn generate_thumbnail(&mut self, _position_secs: f64) -> Option<Vec<u8>> {
        None
    }

    pub fn get_cached_thumbnail(&self, _position_secs: f64) -> Option<&[u8]> {
        None
    }

    pub fn query_subtitle_labels(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn set_eq_bands(&mut self, _bands: &[f64; 5]) {}
}
