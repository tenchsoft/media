//! GStreamer-backed media playback engine.
//!
//! Uses `playbin` + `appsink` to decode media and extract raw RGBA video frames.
//! Frames are sent to the UI thread via an `mpsc` channel.
//!
//! This crate is designed to be shared across Tench media products (Player, Composer).

mod types;

#[cfg(not(feature = "gstreamer-backend"))]
mod fallback;
#[cfg(feature = "gstreamer-backend")]
mod gstreamer_backend;

#[cfg(not(feature = "gstreamer-backend"))]
pub use fallback::PlayerBackend;
#[cfg(feature = "gstreamer-backend")]
pub use gstreamer_backend::PlayerBackend;
pub use types::{MediaEvent, MediaMetadata};
