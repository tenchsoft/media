//! Re-export of the shared media playback backend.
//!
//! All GStreamer logic lives in `tench_media_playback` crate.
//! This module re-exports the public API so existing code continues to work.

pub use tench_media_playback::{MediaEvent, MediaMetadata, PlayerBackend};
