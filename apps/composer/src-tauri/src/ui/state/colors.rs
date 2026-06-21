use tench_composer_core::TrackType;
use tench_ui::prelude::Color;

pub fn track_type_color(kind: TrackType) -> Color {
    match kind {
        TrackType::Video => Color::rgb8(0x60, 0xA5, 0xFA),
        TrackType::Audio => Color::rgb8(0x22, 0xC5, 0x5E),
        TrackType::Subtitle => Color::rgb8(0xA7, 0x8B, 0xFA),
    }
}

pub fn clip_color_for_index(idx: usize) -> Color {
    const PALETTE: [Color; 8] = [
        Color::rgb8(0x60, 0xA5, 0xFA),
        Color::rgb8(0x74, 0xC7, 0xEC),
        Color::rgb8(0x94, 0xE2, 0xD5),
        Color::rgb8(0xF5, 0x9E, 0x0B),
        Color::rgb8(0xFA, 0xB3, 0x87),
        Color::rgb8(0x22, 0xC5, 0x5E),
        Color::rgb8(0xEF, 0x44, 0x44),
        Color::rgb8(0xA7, 0x8B, 0xFA),
    ];
    PALETTE[idx % PALETTE.len()]
}

// ---------------------------------------------------------------------------
// ComposerState — UI state backed by ComposerProject
// ---------------------------------------------------------------------------
