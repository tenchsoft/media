//! Shared color constants for the Player UI.
//!
//! All UI modules should import from here instead of defining their own
//! color constants. This ensures consistency and makes theme changes easy.

use tench_ui::prelude::*;

// --- Background colors ---

/// Main dark background for the video area.
pub const BG_DARK: Color = Color::rgb8(0x0A, 0x0A, 0x0A);
/// Light theme background.
pub const BG_LIGHT: Color = Color::rgb8(0xF5, 0xF5, 0xF2);
/// Grid overlay color for video area.
pub const GRID_COLOR: Color = Color::rgb8(0x15, 0x15, 0x20);

// --- Panel / overlay backgrounds ---

/// Semi-transparent dark background for top overlay bar.
pub const TOP_BAR_BG: Color = Color::rgba8(0, 0, 0, 160);
/// Semi-transparent dark background for controls bar.
pub const CONTROLS_BG: Color = Color::rgba8(0, 0, 0, 200);
/// Toast notification background.
pub const TOAST_BG: Color = Color::rgba8(0, 0, 0, 210);
/// Subtitle background.
pub const SUBTITLE_BG: Color = Color::rgba8(0, 0, 0, 180);

// --- Button colors ---

/// Default button background (drawer tabs, action buttons).
pub const BTN_DEFAULT: Color = Color::rgb8(0x3A, 0x3A, 0x3A);
/// Action button background.
pub const BTN_ACTION: Color = Color::rgb8(0x2A, 0x2D, 0x3D);
/// Dimmed button background (secondary actions, +/- buttons).
pub const BTN_DIM: Color = Color::rgb8(0x2A, 0x2A, 0x2A);

// --- Seekbar colors ---

/// Seekbar track color.
pub const SEEKBAR_TRACK: Color = Color::rgb8(0x3A, 0x3A, 0x3A);
/// Remembered position marker color (amber).
pub const REMEMBERED_MARKER: Color = Color::rgb8(0xF5, 0x9E, 0x0B);
/// Chapter marker color (green).
pub const CHAPTER_MARKER: Color = Color::rgb8(0x22, 0xC5, 0x5E);
/// AI-generated chapter marker color (purple).
pub const CHAPTER_AI_MARKER: Color = Color::rgb8(0xA7, 0x8B, 0xFA);
/// A-B loop highlight color (red).
pub const AB_LOOP_COLOR: Color = Color::rgb8(0xEF, 0x44, 0x44);

// --- Text colors ---

/// Faint play icon overlay.
pub const PLAY_ICON_FAINT: Color = Color::rgba8(255, 255, 255, 80);
