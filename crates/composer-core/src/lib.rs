//! Core timeline and project types for Tench Composer.
//!
//! Provides the data model for a non-linear video editor:
//! - [`TimeRange`] — rational time ranges for clip placement
//! - [`Clip`] — a media clip on the timeline
//! - [`Track`] — an ordered collection of clips
//! - [`Timeline`] — multi-track timeline with edit operations
//! - [`Transition`] — transitions between clips
//! - [`Effect`] — video/audio effects with keyframe animation
//! - [`ComposerProject`] — full project with timeline, media bin, settings

pub mod effect;
pub mod project;
pub mod timeline;
pub mod transition;

pub use effect::*;
pub use project::*;
pub use timeline::*;
pub use transition::*;
