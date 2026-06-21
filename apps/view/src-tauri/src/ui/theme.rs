//! Shared color constants for the View UI.
//!
//! All UI modules should import from here instead of defining their own
//! color constants. This ensures consistency and makes theme changes easy.

use tench_ui::prelude::*;

// --- Background colors ---

/// `var(--neutral-900)` - main background (dark)
pub const NEUTRAL_900: Color = Color::rgb8(0x0F, 0x0F, 0x0F);
/// `var(--neutral-700)` - gray background
pub const NEUTRAL_700: Color = Color::rgb8(0x2A, 0x2A, 0x2A);
/// `var(--neutral-50)` - white background
pub const NEUTRAL_50: Color = Color::rgb8(0xF5, 0xF5, 0xF5);

// --- Panel / overlay backgrounds ---

/// Semi-transparent dark background for panels and overlays.
pub const PANEL_BG: Color = Color::rgba8(0x0F, 0x0F, 0x0F, 240);
/// Semi-transparent dark background for overlays (toolbar, status bar).
pub const OVERLAY_BG: Color = Color::rgba8(0x0F, 0x0F, 0x0F, 200);
/// Toolbar background for edit tools.
pub const TOOLBAR_BG: Color = Color::rgba8(0x0F, 0x0F, 0x0F, 220);

// --- Accent colors ---

/// Primary accent color (blue) for highlights, selected items, and actions.
pub const ACCENT_VIEW: Color = Color::rgb8(0x60, 0xA5, 0xFA);
/// Error / danger color (red) for destructive actions.
pub const ERROR_COLOR: Color = Color::rgb8(0xEF, 0x44, 0x44);

// --- Text colors ---

/// Primary text color for labels and content.
pub const TEXT_PRIMARY: Color = Color::rgb8(0xD4, 0xD4, 0xD4);
/// Secondary text color for subtitles and descriptions.
pub const TEXT_SECONDARY: Color = Color::rgb8(0x8A, 0x8A, 0x8A);
/// Muted text color for hints and disabled labels.
pub const TEXT_MUTED: Color = Color::rgb8(0x6A, 0x6A, 0x6A);

// --- Border / divider colors ---

/// Standard border and divider color.
pub const BORDER_COLOR: Color = Color::rgb8(0x3A, 0x3A, 0x3A);

// --- Button colors ---

/// Default button background.
pub const BTN_BG: Color = Color::rgb8(0x1A, 0x1A, 0x1A);
/// Input field / histogram background.
pub const INPUT_BG: Color = Color::rgb8(0x16, 0x16, 0x16);
