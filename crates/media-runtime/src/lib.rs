use serde::{Deserialize, Serialize};

pub mod composer;
pub mod player;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MediaRuntimeCapabilities {
    pub product_id: &'static str,
    pub can_scan_folders: bool,
    pub can_parse_subtitles: bool,
    pub can_render_timelines: bool,
}

pub const PLAYER_RUNTIME: MediaRuntimeCapabilities = MediaRuntimeCapabilities {
    product_id: "tench-player",
    can_scan_folders: true,
    can_parse_subtitles: true,
    can_render_timelines: false,
};

pub const COMPOSER_RUNTIME: MediaRuntimeCapabilities = MediaRuntimeCapabilities {
    product_id: "tench-composer",
    can_scan_folders: true,
    can_parse_subtitles: true,
    can_render_timelines: true,
};

pub fn media_runtime_products() -> [MediaRuntimeCapabilities; 2] {
    [PLAYER_RUNTIME, COMPOSER_RUNTIME]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composer_extends_player_media_capabilities() {
        const { assert!(PLAYER_RUNTIME.can_parse_subtitles) };
        const { assert!(!PLAYER_RUNTIME.can_render_timelines) };
        const { assert!(COMPOSER_RUNTIME.can_render_timelines) };
    }
}
