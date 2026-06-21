//! Player app state - real state management with ClickRegion system.

mod chapters;
mod click_regions;
mod media;
mod misc;
mod model;
mod panels;
mod playback;
mod playlist;
mod subtitles;
#[cfg(test)]
mod tests;

pub use model::{
    AiChatMessage, AiMessageRole, AspectMode, ChapterMark, ClickAction, ClickRegion,
    ContextMenuItem, ContextMenuState, DrawerTab, EqPreset, GifOptions, MediaInfo, PlayerState,
    PlaylistEntry, RepeatMode, SubtitleEncoding, SubtitleStyle, SubtitleTrack,
};
