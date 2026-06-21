//! Player app state - real state management with ClickRegion system.

use tench_media_runtime::player::SubtitleCue;
use tench_ui::prelude::{Color, Rect};

// ---------------------------------------------------------------------------
// Drawer tabs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerTab {
    Playlist,
    Chapters,
    Subtitles,
    Info,
}

impl DrawerTab {
    pub const ALL: [Self; 4] = [Self::Playlist, Self::Chapters, Self::Subtitles, Self::Info];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Playlist => "Playlist",
            Self::Chapters => "Chapters",
            Self::Subtitles => "Subtitles",
            Self::Info => "Info",
        }
    }
}

// ---------------------------------------------------------------------------
// Click actions
// ---------------------------------------------------------------------------

/// Actions triggered by clicking a button region.
#[derive(Debug, Clone, PartialEq)]
pub enum ClickAction {
    /// Toggle play/pause.
    PlayPause,
    /// Seek to an absolute position (seconds).
    SeekTo(f64),
    /// Set volume (0.0 - 1.0).
    VolumeSet(f64),
    /// Skip to next track in playlist.
    NextTrack,
    /// Skip to previous track in playlist.
    PrevTrack,
    /// Toggle fullscreen mode.
    ToggleFullscreen,
    /// Toggle mute.
    ToggleMute,
    /// Set playback speed.
    SetSpeed(f64),
    /// Toggle a drawer tab.
    ToggleDrawer(DrawerTab),
    /// Select a subtitle track by index.
    SelectSubtitleTrack(usize),
    /// Select an audio track by index.
    SelectAudioTrack(i32),
    /// Take a screenshot.
    Screenshot,
    /// Toggle A-B loop.
    ToggleABLoop,
    /// Open file dialog.
    OpenFile,
    /// Open folder dialog.
    OpenFolder,
    /// Open subtitle file dialog.
    OpenSubtitle,
    /// Show current file in file manager.
    ShowInFiles,
    /// Seek relative (seconds, negative for backward).
    SeekRelative(f64),
    /// Increase playback speed.
    SpeedUp,
    /// Decrease playback speed.
    SpeedDown,
    /// Toggle speed menu.
    ToggleSpeedMenu,
    /// Close current panel/drawer.
    ClosePanel,
    /// Toggle AI panel.
    ToggleAiPanel,
    /// Toggle GIF capture modal.
    ToggleGifCapture,
    /// Toggle fullscreen button.
    Fullscreen,
    /// Toggle theme.
    ToggleTheme,
    /// Cycle repeat mode (None → All → One).
    CycleRepeat,
    /// Toggle shuffle.
    ToggleShuffle,
    /// Cycle aspect ratio mode.
    CycleAspect,
    /// Step forward by one frame (~1/30s).
    StepForward,
    /// Step backward by one frame (~1/30s).
    StepBackward,
    /// Adjust subtitle offset (milliseconds, positive = later).
    SubtitleOffset(i32),
    /// Select a built-in subtitle track by index (-1 = disable).
    SelectBuiltinSubtitleTrack(i32),
    /// Select an audio output device by name.
    SelectAudioDevice(String),
    /// Remove a track from the playlist by index.
    RemoveFromPlaylist(usize),
    /// Play a specific track from the playlist by index.
    PlayPlaylistItem(usize),
    /// Open a recent file by index.
    OpenRecentFile(usize),
    /// Jump to a chapter by index.
    JumpToChapter(usize),
    /// Jump to the remembered position.
    JumpToRememberedPosition,
    /// Add files to playlist via file dialog.
    AddToPlaylist,
    /// Send AI prompt.
    SendAiPrompt(String),
    /// Cancel in-progress AI request.
    CancelAiRequest,
    /// Toggle subtitle style modal.
    ToggleSubtitleStyle,
    /// Open subtitle search.
    OpenSubtitleSearch,
    /// Search subtitle text and jump to next match.
    SearchSubtitleNext,
    /// Search subtitle text and jump to previous match.
    SearchSubtitlePrev,
    /// Change subtitle encoding.
    SetSubtitleEncoding(String),
    /// Open streaming URL.
    OpenUrl,
    /// Submit streaming URL.
    SubmitUrl,
    /// Toggle picture-in-picture mode.
    TogglePip,
    /// Toggle audio equalizer.
    ToggleEqualizer,
    /// Set EQ band value (band_index, value).
    SetEqBand(usize, f64),
    /// Select EQ preset by index.
    SetEqPreset(usize),
    /// Toggle audio normalization.
    ToggleNormalization,
    /// Delete a chapter by index.
    DeleteChapter(usize),
    /// Start renaming a chapter by index.
    RenameChapter(usize),
    /// Open the add chapter modal.
    ShowAddChapterModal,
    /// Export chapters to file.
    ExportChapters,
    /// Import chapters from file.
    ImportChapters,
    /// Confirm chapter rename.
    ConfirmChapterRename,
    /// Show help modal.
    ShowHelp,
    /// Cancel subtitle search.
    CancelSubtitleSearch,
    /// Submit subtitle search.
    SubmitSubtitleSearch,
    /// Start GIF capture options dialog.
    GifOptions,
    /// Set GIF FPS option.
    SetGifFps(u32),
    /// Set GIF max duration option.
    SetGifMaxDuration(u32),
    /// Start GIF recording with options.
    StartGifWithOptions,
    /// Close URL input modal.
    CancelUrl,
    /// Close chapter rename modal.
    CancelChapterRename,
    /// Start GIF recording.
    StartGifRecord,
    /// Stop GIF recording and encode.
    StopGifRecord,
    /// Seek to a percentage of the duration (0.0-1.0).
    SeekToPercent(f64),
    /// Show subtitle search modal.
    ShowSubtitleSearch,
    /// Show subtitle style modal.
    ShowSubtitleStyle,
    /// Focus URL input field.
    FocusUrlInput,
    /// Focus subtitle search input field.
    FocusSubtitleSearch,
    /// Focus chapter name input field.
    FocusChapterNameInput,
    /// Close any open modal.
    CloseModal,
    /// Adjust subtitle offset for a specific track (track_idx, delta_ms).
    SubtitleOffsetForTrack(usize, i32),
    /// Set EQ preset by name.
    SetEqPresetNamed(String),
    /// Start GIF recording (from options modal).
    StartGifRecording,
    /// Confirm add chapter (from modal).
    ConfirmAddChapter,
    /// Play next track in playlist.
    PlayNext,
    /// Play previous track in playlist.
    PlayPrevious,
    /// Focus the AI input text field.
    FocusAiInput,
    /// Adjust a subtitle style property (property index, delta).
    AdjustSubtitleStyle(usize, f32),
}

// ---------------------------------------------------------------------------
// Click regions
// ---------------------------------------------------------------------------

/// A rectangular region that triggers a [`ClickAction`] when clicked.
#[derive(Debug, Clone)]
pub struct ClickRegion {
    pub rect: Rect,
    pub action: ClickAction,
}

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// Context menu state for right-click actions.
#[derive(Debug, Clone)]
pub struct ContextMenuState {
    pub x: f64,
    pub y: f64,
    pub items: Vec<ContextMenuItem>,
}

/// Subtitle encoding options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleEncoding {
    Utf8,
    ShiftJIS,
    EucKR,
    EucJP,
    Iso8859_1,
    Cp1252,
}

impl SubtitleEncoding {
    pub const ALL: [Self; 6] = [
        Self::Utf8,
        Self::ShiftJIS,
        Self::EucKR,
        Self::EucJP,
        Self::Iso8859_1,
        Self::Cp1252,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::ShiftJIS => "Shift-JIS",
            Self::EucKR => "EUC-KR",
            Self::EucJP => "EUC-JP",
            Self::Iso8859_1 => "ISO-8859-1",
            Self::Cp1252 => "Windows-1252",
        }
    }
}

/// AI chat message.
#[derive(Debug, Clone)]
pub struct AiChatMessage {
    pub role: AiMessageRole,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiMessageRole {
    User,
    Assistant,
    System,
}

/// Subtitle style configuration.
#[derive(Debug, Clone)]
pub struct SubtitleStyle {
    pub font_size: f32,
    pub font_family: String,
    pub text_color: Color,
    pub bg_opacity: f32,
    pub stroke_width: f32,
    pub shadow_offset: f32,
    /// Vertical position: 0.0 = bottom, 1.0 = top.
    pub position: f32,
}

impl Default for SubtitleStyle {
    fn default() -> Self {
        Self {
            font_size: 20.0,
            font_family: "sans-serif".to_string(),
            text_color: Color::WHITE,
            bg_opacity: 0.7,
            stroke_width: 1.0,
            shadow_offset: 0.0,
            position: 0.0,
        }
    }
}

/// EQ preset definition.
#[derive(Debug, Clone)]
pub struct EqPreset {
    pub name: &'static str,
    /// Band gains in dB: [60Hz, 250Hz, 1kHz, 4kHz, 16kHz]
    pub bands: [f64; 5],
}

impl EqPreset {
    pub const PRESETS: [&'static EqPreset; 5] = [
        &EqPreset {
            name: "Flat",
            bands: [0.0, 0.0, 0.0, 0.0, 0.0],
        },
        &EqPreset {
            name: "Bass Boost",
            bands: [6.0, 4.0, 0.0, 0.0, 0.0],
        },
        &EqPreset {
            name: "Treble Boost",
            bands: [0.0, 0.0, 0.0, 4.0, 6.0],
        },
        &EqPreset {
            name: "Voice",
            bands: [-2.0, 0.0, 4.0, 3.0, 0.0],
        },
        &EqPreset {
            name: "Loudness",
            bands: [4.0, 2.0, 0.0, 2.0, 4.0],
        },
    ];
}

/// GIF capture options.
#[derive(Debug, Clone)]
pub struct GifOptions {
    pub fps: u32,
    pub max_duration_secs: u32,
    pub quality: u32, // 1-100
}

impl Default for GifOptions {
    fn default() -> Self {
        Self {
            fps: 15,
            max_duration_secs: 10,
            quality: 75,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
}

impl ContextMenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistEntry {
    pub title: String,
    pub duration: f64,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct ChapterMark {
    pub title: String,
    pub time: f64,
    pub ai_generated: bool,
}

#[derive(Debug, Clone)]
pub struct SubtitleTrack {
    pub language: String,
    pub active: bool,
    pub offset_ms: i32,
}

#[derive(Debug, Clone)]
pub struct MediaInfo {
    pub file_name: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub resolution: String,
    pub video_codec: String,
    pub audio_codec: String,
    pub frame_rate: f64,
    pub bitrate: String,
}

/// Aspect ratio mode for video rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectMode {
    /// Fit video within area, preserving aspect ratio (letterbox).
    Fit,
    /// Fill area, cropping video as needed.
    Fill,
    /// Display at original pixel size (1:1).
    Original,
    /// Force 16:9 aspect ratio.
    SixteenNine,
    /// Force 4:3 aspect ratio.
    FourThree,
}

impl AspectMode {
    pub const ALL: [Self; 5] = [
        Self::Fit,
        Self::Fill,
        Self::Original,
        Self::SixteenNine,
        Self::FourThree,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Fit => "Fit",
            Self::Fill => "Fill",
            Self::Original => "1:1",
            Self::SixteenNine => "16:9",
            Self::FourThree => "4:3",
        }
    }

    /// Cycle to the next aspect mode.
    pub fn next(self) -> Self {
        match self {
            Self::Fit => Self::Fill,
            Self::Fill => Self::Original,
            Self::Original => Self::SixteenNine,
            Self::SixteenNine => Self::FourThree,
            Self::FourThree => Self::Fit,
        }
    }
}

/// Repeat mode for playlist playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    /// No repeat — stop at end of playlist.
    None,
    /// Repeat the entire playlist.
    All,
    /// Repeat the current track only.
    One,
}

impl RepeatMode {
    /// Cycle to the next repeat mode.
    pub fn cycle(self) -> Self {
        match self {
            Self::None => Self::All,
            Self::All => Self::One,
            Self::One => Self::None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "\u{21BB} Off",
            Self::All => "\u{21BB} All",
            Self::One => "\u{21BB} One",
        }
    }
}

// ---------------------------------------------------------------------------
// PlayerState
// ---------------------------------------------------------------------------

pub struct PlayerState {
    // Media state
    pub has_media: bool,
    pub title: String,
    pub is_playing: bool,
    pub current_time: f64,
    pub duration: f64,
    pub volume: f64,
    pub is_muted: bool,
    pub playback_rate: f64,
    pub show_speed_menu: bool,

    // Panels
    pub drawer: Option<DrawerTab>,
    pub ai_panel_open: bool,
    pub gif_capture_open: bool,
    pub gif_state: String,
    pub light_theme: bool,

    // A-B loop
    pub ab_loop: Option<(f64, f64)>,
    pub ab_stage: u8,

    // Aspect mode
    pub aspect_mode: AspectMode,

    // Subtitles
    pub subtitle_text: Option<String>,
    pub subtitle_cues: Vec<SubtitleCue>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub subtitle_font_size: f32,

    // Position memory
    pub remembered_position: Option<f64>,

    // Toast
    pub toast: Option<String>,

    // Playlist
    pub playlist: Vec<PlaylistEntry>,
    pub current_playlist_index: Option<usize>,
    pub recent_files: Vec<PlaylistEntry>,
    pub repeat_mode: RepeatMode,
    pub shuffle_enabled: bool,

    // Chapters
    pub chapters: Vec<ChapterMark>,

    // Media info
    pub media_info: MediaInfo,

    // File path of current media
    pub media_path: Option<String>,

    // Click regions (cleared each paint frame)
    pub click_regions: Vec<ClickRegion>,

    // Buffering state
    pub buffering_percent: u32,
    pub is_buffering: bool,

    // Built-in subtitle streams (from container)
    pub n_builtin_subtitle_tracks: u32,
    pub active_builtin_subtitle_track: i32,

    // Audio devices
    pub audio_devices: Vec<(String, String)>,
    pub selected_audio_device: Option<String>,

    // Audio visualization levels (dB values per channel)
    pub audio_levels: Vec<f64>,

    // Context menu
    pub context_menu: Option<ContextMenuState>,

    // Drawer scroll offset
    pub drawer_scroll_y: f64,

    // AI panel state
    pub ai_input_text: String,
    pub ai_chat_log: Vec<AiChatMessage>,
    pub ai_request_pending: bool,
    pub ai_focused: bool,

    // Subtitle style
    pub subtitle_style: SubtitleStyle,
    pub subtitle_style_open: bool,

    // Subtitle search
    pub subtitle_search_open: bool,
    pub subtitle_search_text: String,
    pub subtitle_search_results: Vec<usize>, // indices into subtitle_cues
    pub subtitle_search_current: Option<usize>,

    // Subtitle encoding
    pub subtitle_encoding: SubtitleEncoding,

    // URL input modal
    pub url_input_open: bool,
    pub url_input_text: String,

    // Picture-in-picture
    pub pip_mode: bool,

    // Audio equalizer
    pub eq_open: bool,
    pub eq_bands: [f64; 5], // dB values: [60Hz, 250Hz, 1kHz, 4kHz, 16kHz]
    pub eq_preset_idx: usize,

    // Audio normalization
    pub normalization_enabled: bool,

    // Chapter rename
    pub chapter_rename_idx: Option<usize>,
    pub chapter_rename_text: String,

    // Help modal
    pub help_open: bool,

    // GIF options
    pub gif_options_open: bool,
    pub gif_options: GifOptions,

    // Context menu hover index
    pub context_menu_hover: Option<usize>,

    // Mouse position for hover effects
    pub mouse_pos: (f64, f64),

    // Built-in subtitle language tags
    pub builtin_subtitle_labels: Vec<String>,

    // Speed menu anchor position
    pub speed_menu_anchor: (f64, f64),

    // Add chapter modal
    pub show_add_chapter_modal: bool,
    pub chapter_name_input: String,
    pub chapter_name_input_focused: bool,

    // Input focus tracking
    pub url_input_focused: bool,
    pub subtitle_search_focused: bool,

    // Subtitle search result time
    pub subtitle_search_result_time: Option<f64>,
}

impl PlayerState {
    /// Create a new empty state (no media loaded).
    pub fn new() -> Self {
        Self {
            has_media: false,
            title: "No media loaded".into(),
            is_playing: false,
            current_time: 0.0,
            duration: 0.0,
            volume: 0.75,
            is_muted: false,
            playback_rate: 1.0,
            show_speed_menu: false,
            drawer: None,
            ai_panel_open: false,
            gif_capture_open: false,
            gif_state: "idle".into(),
            light_theme: false,
            ab_loop: None,
            ab_stage: 0,
            subtitle_text: None,
            subtitle_cues: Vec::new(),
            subtitle_tracks: Vec::new(),
            subtitle_font_size: 20.0,
            remembered_position: None,
            toast: None,
            aspect_mode: AspectMode::Fit,
            playlist: Vec::new(),
            current_playlist_index: None,
            recent_files: Vec::new(),
            repeat_mode: RepeatMode::None,
            shuffle_enabled: false,
            chapters: Vec::new(),
            media_info: MediaInfo {
                file_name: String::new(),
                title: String::new(),
                artist: String::new(),
                album: String::new(),
                resolution: String::new(),
                video_codec: String::new(),
                audio_codec: String::new(),
                frame_rate: 0.0,
                bitrate: String::new(),
            },
            media_path: None,
            click_regions: Vec::new(),
            buffering_percent: 100,
            is_buffering: false,
            drawer_scroll_y: 0.0,
            n_builtin_subtitle_tracks: 0,
            active_builtin_subtitle_track: -1,
            audio_devices: Vec::new(),
            selected_audio_device: None,
            audio_levels: Vec::new(),
            context_menu: None,
            ai_input_text: String::new(),
            ai_chat_log: Vec::new(),
            ai_request_pending: false,
            ai_focused: false,
            subtitle_style: SubtitleStyle::default(),
            subtitle_style_open: false,
            subtitle_search_open: false,
            subtitle_search_text: String::new(),
            subtitle_search_results: Vec::new(),
            subtitle_search_current: None,
            subtitle_encoding: SubtitleEncoding::Utf8,
            url_input_open: false,
            url_input_text: String::new(),
            pip_mode: false,
            eq_open: false,
            eq_bands: [0.0; 5],
            eq_preset_idx: 0,
            normalization_enabled: false,
            chapter_rename_idx: None,
            chapter_rename_text: String::new(),
            help_open: false,
            gif_options_open: false,
            gif_options: GifOptions::default(),
            context_menu_hover: None,
            mouse_pos: (0.0, 0.0),
            builtin_subtitle_labels: Vec::new(),
            speed_menu_anchor: (0.0, 0.0),
            show_add_chapter_modal: false,
            chapter_name_input: String::new(),
            chapter_name_input_focused: false,
            url_input_focused: false,
            subtitle_search_focused: false,
            subtitle_search_result_time: None,
        }
    }

    pub fn example() -> Self {
        let mut state = Self::new();
        state.has_media = true;
        state.title = "Tench Player Demo.mp4".into();
        state.current_time = 42.0;
        state.duration = 180.0;
        state.volume = 0.65;
        state.media_path = Some("/tmp/tench-player-demo.mp4".into());
        state.media_info = MediaInfo {
            file_name: "tench-player-demo.mp4".into(),
            title: "Tench Player Demo".into(),
            artist: "Tench".into(),
            album: "Plan E2E".into(),
            resolution: "1920x1080".into(),
            video_codec: "H.264".into(),
            audio_codec: "AAC".into(),
            frame_rate: 24.0,
            bitrate: "8.0 Mbps".into(),
        };
        state.playlist = vec![
            PlaylistEntry {
                title: "Opening Scene".into(),
                duration: 180.0,
                path: "/tmp/tench-player-demo.mp4".into(),
            },
            PlaylistEntry {
                title: "Credits".into(),
                duration: 75.0,
                path: "/tmp/tench-player-credits.mp4".into(),
            },
        ];
        state.current_playlist_index = Some(0);
        state.recent_files = vec![
            PlaylistEntry {
                title: "Recent Lecture".into(),
                duration: 240.0,
                path: "/tmp/tench-player-recent.mp4".into(),
            },
            PlaylistEntry {
                title: "Recent Clip".into(),
                duration: 90.0,
                path: "/tmp/tench-player-clip.mp4".into(),
            },
        ];
        state.chapters = vec![
            ChapterMark {
                title: "Intro".into(),
                time: 12.0,
                ai_generated: false,
            },
            ChapterMark {
                title: "AI Generated Beat".into(),
                time: 90.0,
                ai_generated: true,
            },
        ];
        state.subtitle_cues = vec![
            SubtitleCue {
                id: "demo-1".into(),
                start: 40.0,
                end: 45.0,
                text: "The gate opens".into(),
            },
            SubtitleCue {
                id: "demo-2".into(),
                start: 80.0,
                end: 88.0,
                text: "A second line appears".into(),
            },
        ];
        state.subtitle_tracks = vec![SubtitleTrack {
            language: "English".into(),
            active: true,
            offset_ms: 0,
        }];
        state.subtitle_text = Some("The gate opens".into());
        state.n_builtin_subtitle_tracks = 2;
        state.builtin_subtitle_labels = vec!["eng".into(), "kor".into()];
        state.audio_devices = vec![
            ("System Default".into(), "sink".into()),
            ("Studio Monitor".into(), "sink".into()),
        ];
        state.selected_audio_device = Some("System Default".into());
        state.audio_levels = vec![-12.0, -18.0, -24.0, -30.0];
        state.remembered_position = Some(72.0);
        state.buffering_percent = 64;
        state.is_buffering = true;
        state.ab_loop = Some((30.0, 75.0));
        state.ai_request_pending = true;
        state.toast = Some("Demo ready".into());
        state
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}
