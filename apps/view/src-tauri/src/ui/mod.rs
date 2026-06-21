//! View app - full Rust-native UI using tench-ui, matching the original React UI.
//!
//! This module re-implements the complete View image viewer experience,
//! including image display, overlays, panels, tools, and all interactions.

mod app;
pub mod controls;
pub mod image_stage;
pub mod overlays;
pub mod panels;
pub mod state;
pub mod theme;
pub mod tools;

pub use app::ViewApp;
