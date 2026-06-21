/// Events emitted by the media backend, consumed by the UI thread.
#[derive(Debug, Clone)]
pub enum MediaEvent {
    /// Playback position updated (seconds).
    Position(f64),
    /// Media duration changed (seconds).
    Duration(f64),
    /// Playback reached the end.
    EndOfStream,
    /// An error occurred.
    Error(String),
    /// Media loaded successfully.
    Loaded {
        duration: f64,
        width: u32,
        height: u32,
    },
    /// New video frame available (RGBA8 pixels, width, height).
    VideoFrame {
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    /// Media has audio but no video (audio-only mode).
    AudioOnly,
    /// Buffering progress update (percentage 0-100).
    Buffering(u32),
    /// Gapless: about to finish, request next track URI.
    AboutToFinish,
    /// Audio level update for visualization (dB values per channel, typically 0.0 to -60.0).
    AudioLevels(Vec<f64>),
}

/// Metadata extracted from media tags.
#[derive(Debug, Clone, Default)]
pub struct MediaMetadata {
    pub video_codec: String,
    pub audio_codec: String,
    pub bitrate: String,
    pub framerate: f64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub container_format: String,
}
