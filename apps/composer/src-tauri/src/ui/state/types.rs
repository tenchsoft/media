use tench_composer_core::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerMode {
    Edit,
    Color,
    Audio,
    Deliver,
}

impl ComposerMode {
    pub const ALL: [Self; 4] = [Self::Edit, Self::Color, Self::Audio, Self::Deliver];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Edit => "Edit",
            Self::Color => "Color",
            Self::Audio => "Audio",
            Self::Deliver => "Deliver",
        }
    }

    /// Inspector tab index matching the ComposerMode order [Edit, Color, Audio, Deliver].
    pub const fn inspector_index(self) -> usize {
        match self {
            Self::Edit => 0,
            Self::Color => 1,
            Self::Audio => 2,
            Self::Deliver => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftPanelTab {
    Media,
    Templates,
    Effects,
    Transitions,
}

impl LeftPanelTab {
    pub const ALL: [Self; 4] = [
        Self::Media,
        Self::Templates,
        Self::Effects,
        Self::Transitions,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Media => "Media",
            Self::Templates => "Templates",
            Self::Effects => "Effects",
            Self::Transitions => "Trans.",
        }
    }
}

/// Click actions for the Composer UI.
#[derive(Debug, Clone)]
pub enum ClickAction {
    // Transport
    PlayPause,
    StepForward,
    StepBackward,
    SeekTo(f64),

    // Editing
    SplitAtPlayhead,
    DeleteClip(ClipId),
    MoveClip {
        clip_id: ClipId,
        source_track: TrackId,
        dest_track: TrackId,
        new_timeline_in: u32,
    },
    TrimClipIn {
        clip_id: ClipId,
        track_id: TrackId,
        new_in: u32,
    },
    TrimClipOut {
        clip_id: ClipId,
        track_id: TrackId,
        new_out: u32,
    },

    // Track controls
    ToggleTrackMute(TrackId),
    ToggleTrackLock(TrackId),
    ToggleTrackHidden(TrackId),
    AddTrack(TrackType),
    DeleteTrack(TrackId),

    // Panels
    SelectMode(ComposerMode),
    SelectLeftTab(LeftPanelTab),
    SelectInspectorTab(usize),
    SelectClip(Option<ClipId>),
    SelectTemplate(usize),
    FocusEffectsSearch,
    FocusTransitionsSearch,
    ApplyEffect(VideoEffectType),
    ApplyTransition(TransitionType),
    RunAiFeature(String),
    /// Drag media from bin to a specific track.
    DropMediaOnTrack {
        media_idx: usize,
        track_id: TrackId,
        timeline_in: u32,
    },

    // Actions
    ImportMedia,
    Export,
    ToggleRenderQueue,
    ToggleAiPanel,
    CloseRenderQueue,
    CancelRenderJob(u64),
    PauseRenderJob(u64),

    // Timeline tools
    ToggleSnap,
    ToggleRipple,
    ToggleMagnet,
    ZoomIn,
    ZoomOut,

    // Context menu actions
    CutClip(ClipId),
    CopyClip(ClipId),
    PasteClip,
    DuplicateClip(ClipId),
    RemoveEffectFromClip {
        clip_id: ClipId,
        effect_idx: usize,
    },

    // Inspector edits
    SetClipName(ClipId, String),
    SetClipSpeed(ClipId, f64),
    ToggleClipReversed(ClipId),
    SetTrackVolume(TrackId, f64),
    SetTrackPan(TrackId, f64),
    ToggleTrackMuted(TrackId),
    SetExportFormat(ExportFormat),
    SetExportCodec(VideoCodec),
    SetExportResolution(u32, u32),
    SetExportFps(f64),
    SetExportBitrate(u32),

    // Subtitle
    FocusSubtitleEditor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerInputFocus {
    None,
    Subtitle,
    EffectsSearch,
    TransitionsSearch,
}

/// What kind of drag is in progress.
#[derive(Debug, Clone)]
pub enum DragKind {
    /// Dragging a clip on the timeline.
    Clip {
        clip_id: ClipId,
        source_track_id: TrackId,
        offset_frames: i32,
    },
    /// Trimming the in-point of a clip.
    TrimIn {
        clip_id: ClipId,
        track_id: TrackId,
        original_in: u32,
    },
    /// Trimming the out-point of a clip.
    TrimOut {
        clip_id: ClipId,
        track_id: TrackId,
        original_out: u32,
    },
    /// Dragging media from bin onto timeline.
    MediaBin { media_idx: usize },
    /// Dragging an effect from the effects panel onto a clip.
    Effect { effect_type: VideoEffectType },
    /// Dragging a transition onto a clip edge.
    Transition { transition_type: TransitionType },
    /// Dragging a track header to reorder.
    TrackReorder {
        track_id: TrackId,
        original_index: usize,
    },
    /// Dragging a splitter to resize panels.
    Splitter(SplitterTarget),
}

/// Which splitter is being dragged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitterTarget {
    LeftPanel,
    RightPanel,
    Timeline,
}

/// Context menu state.
#[derive(Debug, Clone)]
pub struct ContextMenuState {
    pub x: f64,
    pub y: f64,
    pub items: Vec<ContextMenuItem>,
}

/// A context menu item.
#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub label: String,
    pub action: ClickAction,
    pub enabled: bool,
}

/// Clipboard for cut/copy/paste.
#[derive(Debug, Clone, Default)]
pub struct ComposerClipboard {
    pub clip: Option<Clip>,
}

// ---------------------------------------------------------------------------
// Color helpers for track types
// ---------------------------------------------------------------------------

pub struct ComposerState {
    /// The real project data.
    pub project: ComposerProject,

    // UI state
    pub mode: ComposerMode,
    pub left_tab: LeftPanelTab,
    pub active_inspector_tab: usize,
    pub is_playing: bool,
    pub current_frame: u32,
    pub zoom: f64,
    pub snap: bool,
    pub ripple: bool,
    pub magnetic: bool,
    pub selected_clip_id: Option<ClipId>,
    /// Multi-selection: additional clip IDs selected alongside the primary.
    pub selected_clip_ids: Vec<ClipId>,
    pub selected_template_idx: Option<usize>,
    pub import_status: String,
    pub composer_notice: String,
    pub notice_expires_at: Option<std::time::Instant>,
    pub show_render_queue: bool,
    pub show_ai_panel: bool,
    pub subtitle_text: String,
    pub subtitle_focused: bool,
    pub input_focus: ComposerInputFocus,

    // In/out points (frame numbers).
    pub in_point: Option<u32>,
    pub out_point: Option<u32>,
    pub loop_playback: bool,

    // JKL shuttle state.
    pub shuttle_speed: f64,
    pub shuttle_direction: i32, // -1 reverse, 0 stopped, 1 forward
    pub j_press_count: u32,
    pub l_press_count: u32,

    // Drag state.
    pub drag: Option<DragKind>,
    pub drag_start_pos: Option<tench_ui::prelude::Point>,

    // Context menu.
    pub context_menu: Option<ContextMenuState>,

    // Clipboard.
    pub clipboard: ComposerClipboard,

    // Layout dimensions (user-adjustable).
    pub left_panel_w: f64,
    pub right_panel_w: f64,
    pub timeline_h: f64,

    // Project persistence.
    pub save_path: Option<String>,
    pub last_auto_save: Option<std::time::Instant>,
    pub recent_projects: Vec<String>,

    // Undo/redo history.
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,

    // Search/filter for effects and transitions.
    pub effects_search: String,
    pub transitions_search: String,

    // Waveform cache: media_path -> samples for rendering.
    pub waveform_cache: std::collections::HashMap<String, Vec<f32>>,

    // Auto-save interval.
    pub auto_save_interval: std::time::Duration,
}
