//! Composer state model backed by tench-composer-core.

mod actions;
mod colors;
mod init;
mod notice;
mod playback;
mod project_io;
mod queries;
#[cfg(test)]
mod tests;
mod types;
mod undo;

pub use colors::{clip_color_for_index, track_type_color};
pub use types::*;
