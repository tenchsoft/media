//! Unified theme colors for Pixel Design UI.
//!
//! Eliminates hex color repetition across modules.

use tench_ui::prelude::Color;

// Surface / background colors
pub const BG_APP: Color = Color::rgb8(0x0F, 0x0F, 0x0F);
pub const BG_SURFACE: Color = Color::rgb8(0x20, 0x22, 0x2B);
pub const BG_PANEL: Color = Color::rgb8(0x20, 0x22, 0x2B);
pub const BG_BUTTON: Color = Color::rgb8(0x2A, 0x2A, 0x2A);
pub const BG_ROW_ACTIVE: Color = Color::rgb8(0x31, 0x49, 0x68);
pub const BG_ROW_INACTIVE: Color = Color::rgb8(0x27, 0x2A, 0x33);
pub const BG_TAB_ACTIVE: Color = Color::rgb8(0x60, 0xA5, 0xFA);
pub const BG_TAB_INACTIVE: Color = Color::rgb8(0x2A, 0x2D, 0x37);
pub const BG_INPUT: Color = Color::rgb8(0x27, 0x2A, 0x33);
pub const BG_BAR: Color = Color::rgb8(0x31, 0x34, 0x40);
pub const BG_THUMBNAIL: Color = Color::rgb8(0x3A, 0x3E, 0x4A);

// Canvas background
pub const CANVAS_BG: Color = Color::rgb8(0x0F, 0x0F, 0x0F);

// Checkerboard colors
pub const CHECKER_LIGHT: Color = Color::rgb8(0x1E, 0x1E, 0x1E);
pub const CHECKER_DARK: Color = Color::rgb8(0x16, 0x16, 0x16);

// Border colors
pub const BORDER: Color = Color::rgb8(0x35, 0x38, 0x45);

// Accent colors
pub const ACCENT: Color = Color::rgb8(0x60, 0xA5, 0xFA);
pub const ACCENT_GREEN: Color = Color::rgb8(0x22, 0xC5, 0x5E);
pub const ACCENT_YELLOW: Color = Color::rgb8(0xF5, 0x9E, 0x0B);
pub const ACCENT_RED: Color = Color::rgb8(0xEF, 0x44, 0x44);

// Text colors
pub const TEXT_ON_ACCENT: Color = Color::rgb8(0x11, 0x18, 0x27);
pub const TEXT_ON_GREEN: Color = Color::rgb8(0x10, 0x19, 0x18);
pub const TEXT_DISABLED: Color = Color::rgb8(0xBA, 0xBD, 0xC8);

// Selection
pub const SELECTION_COLOR: Color = Color::rgb8(0xFF, 0xFF, 0xFF);

// Canvas cursors
pub const CROSSHAIR: Color = Color::rgb8(0xFF, 0xFF, 0xFF);

// Grid overlay
pub const GRID_COLOR: Color = Color::rgb8(0x40, 0x40, 0x40);

// Status bar zoom button
pub const ZOOM_BUTTON: Color = Color::rgb8(0x2A, 0x2A, 0x2A);
