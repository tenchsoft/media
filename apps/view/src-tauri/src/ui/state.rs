//! View app state - mirrors the React ViewExperience component state.

use std::collections::{HashMap, HashSet};

use tench_ui::kurbo;
use tench_ui::peniko;
use tench_ui::prelude::*;

use tench_image_runtime::view::util::HistogramData;

/// Settings panel tab.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsTab {
    General,
    Image,
    Slideshow,
    FileAssociation,
}

/// An action triggered by clicking a button region.
#[derive(Debug, Clone, PartialEq)]
pub enum ClickAction {
    /// Open file dialog.
    OpenFileDialog,
    /// Open folder dialog.
    OpenFolderDialog,
    /// Open archive dialog.
    OpenArchiveDialog,
    /// Navigate to the next image.
    NavigateNext,
    /// Navigate to the previous image.
    NavigatePrev,
    /// Set fit mode to Fit.
    ZoomFit,
    /// Set fit mode to Actual (100%).
    ZoomActual,
    /// Zoom in.
    ZoomIn,
    /// Zoom out.
    ZoomOut,
    /// Toggle filmstrip.
    ToggleThumbnails,
    /// Rotate image 90 degrees.
    Rotate,
    /// Cycle background color.
    CycleBgColor,
    /// Toggle fullscreen.
    ToggleFullscreen,
    /// Toggle metadata panel.
    ToggleMetadata,
    /// Toggle quick edit.
    ToggleQuickEdit,
    /// Toggle filter panel.
    ToggleFilter,
    /// Toggle AI panel.
    ToggleAi,
    /// Toggle file info overlay.
    ToggleFileInfo,
    /// Toggle batch panel.
    ToggleBatch,
    /// Show in file manager.
    ShowInFiles,
    /// Copy file path.
    CopyPath,
    /// Copy image to clipboard.
    CopyImage,
    /// Toggle slideshow.
    ToggleSlideshow,
    /// Toggle compare mode.
    ToggleCompare,
    /// Dismiss all overlays (Escape).
    DismissAll,
    /// Open a recent file by index.
    OpenRecentFile(usize),
    /// Navigate to folder entry by index (filmstrip click).
    NavigateToIndex(usize),
    /// Context menu action.
    ContextMenuAction(String),
    /// Delete confirm: Cancel.
    DeleteCancel,
    /// Delete confirm: Delete.
    DeleteConfirm,
    /// Edit banner: Save.
    EditSave,
    /// Edit banner: Discard.
    EditDiscard,
    /// Sort by key.
    SortByKey,
    /// Toggle sort order.
    ToggleSortOrder,
    /// Batch trigger button.
    OpenBatch,
    /// Crop tool: Apply crop.
    CropApply,
    /// Crop tool: Cancel.
    CropCancel,
    /// Resize tool: Apply resize.
    ResizeApply,
    /// Resize tool: Cancel.
    ResizeCancel,
    /// Resize tool: Width minus.
    ResizeWidthMinus,
    /// Resize tool: Width plus.
    ResizeWidthPlus,
    /// Resize tool: Height minus.
    ResizeHeightMinus,
    /// Resize tool: Height plus.
    ResizeHeightPlus,
    /// Resize tool: Toggle maintain aspect.
    ResizeToggleAspect,
    /// Convert tool: Select format.
    ConvertSelectFormat(String),
    /// Convert tool: Apply.
    ConvertApply,
    /// Convert tool: Cancel.
    ConvertCancel,
    /// Filter panel: Reset filters.
    FilterReset,
    /// Filter panel: Apply filters.
    FilterApply,
    /// Batch panel: Toggle file selection at index.
    BatchToggleFile(usize),
    /// Batch panel: Select all / deselect all.
    BatchToggleSelectAll,
    /// Batch panel: Switch to resize mode.
    BatchModeResize,
    /// Batch panel: Switch to convert mode.
    BatchModeConvert,
    /// Batch panel: Select batch format.
    BatchSelectFormat(String),
    /// Batch panel: Apply batch operation.
    BatchApply,
    /// Slideshow controls: Cycle interval.
    SlideshowCycleInterval,
    /// Compare panel: Start dragging split.
    CompareDragStart,
    /// Rename dialog: Confirm rename.
    RenameConfirm,
    /// Rename dialog: Cancel.
    RenameCancel,
    /// Convert tool: Browse output path.
    ConvertBrowseOutput,
    /// Open image from URL.
    OpenFromUrl,
    /// Actually load the URL from the text input.
    LoadFromUrl,
    /// Cancel the URL dialog.
    UrlCancel,
    /// Set image rating (1-5 stars).
    SetRating(u8),
    /// Toggle a tag on the current image.
    ToggleTag(String),
    /// Print the current image.
    PrintImage,
    /// Print dialog: cancel.
    PrintCancel,
    /// Print dialog: select paper size.
    PrintSelectPaper(String),
    /// Print dialog: select orientation.
    PrintSelectOrientation(String),
    /// Print dialog: select scaling.
    PrintSelectScaling(String),
    /// Toggle slideshow shuffle mode.
    SlideshowToggleShuffle,
    /// Cancel a running batch operation.
    BatchCancel,
    /// Browse for batch output folder.
    BatchBrowseOutput,
    /// Toggle help overlay.
    ShowHelp,
    /// Select an AI feature for processing.
    SelectAiFeature(AiFeature),
    /// Run the selected AI feature.
    RunAi,
    /// Toggle search panel.
    ToggleSearch,
    /// Submit a search query.
    SearchSubmit,
    /// Toggle bookmark on current folder.
    ToggleBookmark,
    /// Open a bookmarked folder.
    OpenBookmark(usize),
    /// Select an annotation tool.
    SelectAnnotationTool(AnnotationTool),
    /// Enable quick-edit markup controls.
    QuickEditMarkup,
    /// Clear all annotations.
    ClearAnnotations,
    /// Cycle compare mode (split/side-by-side/diff).
    CycleCompareMode,
    /// Cycle slideshow transition effect.
    SlideshowCycleTransition,
    /// Toggle slideshow loop mode.
    SlideshowToggleLoop,
    /// Toggle the annotation color picker.
    ToggleAnnotationColorPicker,
    /// Set the annotation color from the color picker.
    SetAnnotationColor(Color),
    /// Open a recent file from the empty state view.
    OpenRecentFromEmpty(usize),
    /// Share the current image file.
    ShareImage,
    /// Set current image as wallpaper.
    SetWallpaperAction,
    /// Show delete confirmation from toolbar.
    DeleteFromToolbar,
    /// Annotation undo.
    AnnotationUndo,
    /// Annotation redo.
    AnnotationRedo,
    /// Save (burn) annotations into image.
    AnnotationSave,
    /// Exit annotation mode.
    AnnotationExit,
    /// Confirm exit (discard annotations).
    AnnotationExitConfirm,
    /// Cancel exit confirmation dialog.
    AnnotationExitCancel,
    /// Set annotation line width.
    AnnotationSetLineWidth(f32),
    /// Toggle eraser mode.
    AnnotationEraseMode,
    /// Confirm text annotation input.
    AnnotationTextConfirm,
    /// Toggle settings panel.
    ToggleSettings,
    /// Switch settings tab.
    SettingsTab(SettingsTab),
    /// Close settings panel.
    SettingsClose,
    /// Toggle checkerboard background for transparent images.
    ToggleCheckerboard,
    /// Set crop aspect ratio.
    CropAspectRatio(u32, u32),
    /// Set crop aspect ratio to free (no constraint).
    CropAspectRatioFree,
    /// Toggle the hamburger menu.
    ToggleMenu,
}

/// A rectangular region that triggers a [`ClickAction`] when clicked.
#[derive(Debug, Clone)]
pub struct ClickRegion {
    pub rect: Rect,
    pub action: ClickAction,
}

/// Background color cycle for the image stage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BgColor {
    Black,
    Gray,
    White,
}

impl BgColor {
    pub fn cycle(self) -> Self {
        match self {
            Self::Black => Self::Gray,
            Self::Gray => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn as_color(self) -> Color {
        match self {
            Self::Black => Color::rgb8(0x0F, 0x0F, 0x0F),
            Self::Gray => Color::rgb8(0x2A, 0x2A, 0x2A),
            Self::White => Color::rgb8(0xF5, 0xF5, 0xF5),
        }
    }
}

/// Image fit mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FitMode {
    Fit,
    Actual,
}

/// Sort key for folder entries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortKey {
    Name,
    Modified,
    Size,
}

impl SortKey {
    pub fn cycle(self) -> Self {
        match self {
            Self::Name => Self::Modified,
            Self::Modified => Self::Size,
            Self::Size => Self::Name,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Modified => "Modified",
            Self::Size => "Size",
        }
    }
}

/// Sort order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn toggle(self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }

    pub fn arrow(self) -> &'static str {
        match self {
            Self::Asc => "\u{2191}",
            Self::Desc => "\u{2193}",
        }
    }
}

/// Active edit tool.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditTool {
    Crop,
    Resize,
    Convert,
}

/// AI feature keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiFeature {
    Enhance,
    Upscale,
    BackgroundRemoval,
    SmartCrop,
    Tag,
    Describe,
}

impl AiFeature {
    pub fn all() -> &'static [AiFeature] {
        &[
            AiFeature::Enhance,
            AiFeature::Upscale,
            AiFeature::BackgroundRemoval,
            AiFeature::SmartCrop,
            AiFeature::Tag,
            AiFeature::Describe,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Enhance => "Enhance",
            Self::Upscale => "Upscale 2x",
            Self::BackgroundRemoval => "BG Remove",
            Self::SmartCrop => "Smart Crop",
            Self::Tag => "Tag",
            Self::Describe => "Describe",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Enhance => "AI-powered image enhancement",
            Self::Upscale => "Super-resolution upscaling",
            Self::BackgroundRemoval => "Remove image background",
            Self::SmartCrop => "Intelligent cropping",
            Self::Tag => "Auto-tag image content",
            Self::Describe => "Generate image description",
        }
    }
}

/// Slideshow transition effect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlideshowTransition {
    /// Cross-fade between images.
    Fade,
    /// Slide horizontally.
    Slide,
    /// Simple dissolve.
    Dissolve,
    /// No transition (instant cut).
    None,
}

impl SlideshowTransition {
    pub fn cycle(self) -> Self {
        match self {
            Self::Fade => Self::Slide,
            Self::Slide => Self::Dissolve,
            Self::Dissolve => Self::None,
            Self::None => Self::Fade,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Fade => "Fade",
            Self::Slide => "Slide",
            Self::Dissolve => "Dissolve",
            Self::None => "None",
        }
    }
}

/// Annotation tool types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnnotationTool {
    Arrow,
    Rectangle,
    Circle,
    Text,
    Freeform,
    BlurMask,
    Eraser,
}

impl AnnotationTool {
    /// Returns the short display label used in the quick-edit toolbar.
    pub fn label(self) -> &'static str {
        match self {
            Self::Arrow => "arrow",
            Self::Rectangle => "rect",
            Self::Circle => "circle",
            Self::Text => "text",
            Self::Freeform => "draw",
            Self::BlurMask => "blur",
            Self::Eraser => "eraser",
        }
    }
}

/// An annotation placed on the image.
#[derive(Debug, Clone)]
pub struct Annotation {
    pub tool: AnnotationTool,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub text: String,
    pub color: Color,
    pub line_width: f32,
}

/// Pixel info at a given coordinate.
#[derive(Debug, Clone, Copy)]
pub struct PixelInfo {
    pub x: u32,
    pub y: u32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Compare mode for before/after view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompareMode {
    /// Split view with draggable slider.
    Split,
    /// Side-by-side view.
    SideBySide,
    /// Difference overlay (highlights pixel differences).
    Difference,
}

/// Image dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

/// Simplified image metadata for the UI.
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub file_name: String,
    pub format: String,
    pub dimensions: Option<ImageDimensions>,
    pub file_size: u64,
    pub path: String,
}

/// A simplified folder entry.
#[derive(Debug, Clone)]
pub struct FolderEntry {
    pub id: String,
    pub path: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub modified_unix: Option<u64>,
    pub is_archive_entry: bool,
}

/// EXIF tag from tench_image_core.
#[derive(Debug, Clone)]
pub struct MetadataTag {
    pub group: String,
    pub name: String,
    pub value: String,
}

/// Edit history entry for undo/redo.
#[derive(Debug, Clone)]
pub struct EditHistoryEntry {
    pub image_data: peniko::ImageData,
    pub label: String,
}

/// The complete view app state.
#[derive(Clone)]
pub struct ViewState {
    // Image
    pub document: Option<ImageMetadata>,
    pub folder_entries: Vec<FolderEntry>,
    pub sorted_entries: Vec<FolderEntry>,

    // Decoded image data for rendering (current display)
    pub current_image_data: Option<peniko::ImageData>,
    // Original image data (before edits, for compare mode and discard)
    pub original_image_data: Option<peniko::ImageData>,

    // EXIF metadata tags from tench_image_core
    pub exif_tags: Vec<MetadataTag>,

    // Histogram data
    pub histogram: Option<HistogramData>,

    // Dual transform matrix (nomacs pattern)
    // img_transform: image→viewport fit transform
    // user_transform: zoom/pan on top of img_transform
    pub img_transform: kurbo::Affine,
    pub user_transform: kurbo::Affine,

    // View
    pub zoom: f64,
    pub pan_x: f64,
    pub pan_y: f64,
    pub fit_mode: FitMode,
    pub rotation: u32,
    pub bg_color: BgColor,

    // Chrome
    pub show_chrome: bool,
    pub show_metadata: bool,
    pub show_thumbnails: bool,
    pub show_quick_edit: bool,
    pub show_delete_confirm: bool,
    pub show_filter: bool,
    pub show_compare: bool,
    pub show_batch: bool,
    pub show_ai: bool,
    pub show_file_info: bool,
    pub show_context_menu: bool,
    pub context_menu_x: f64,
    pub context_menu_y: f64,

    // Edit
    pub active_edit_tool: Option<EditTool>,
    pub edit_saving: bool,
    pub has_edited_image: bool,
    // Edit history stack for undo/redo
    pub edit_history: Vec<EditHistoryEntry>,
    pub edit_history_index: usize,

    // Filters
    pub filter_brightness: f64,
    pub filter_contrast: f64,
    pub filter_saturation: f64,
    pub filter_blur: f64,
    pub filter_hue_rotate: f64,

    // Sort
    pub sort_key: SortKey,
    pub sort_order: SortOrder,

    // Slideshow
    pub slideshow_playing: bool,
    pub slideshow_interval_ms: u64,

    // AI
    pub ai_running: bool,
    pub ai_selected_feature: Option<AiFeature>,

    // Loading
    pub is_loading: bool,

    // Status
    pub status_message: String,

    // Crop tool state
    pub crop_start: Option<(f64, f64)>,
    pub crop_selection: Option<(f64, f64, f64, f64)>, // x, y, w, h
    pub crop_dragging: bool,

    // Resize tool state
    pub resize_width: u32,
    pub resize_height: u32,
    pub resize_maintain_aspect: bool,
    pub resize_orig_width: u32,
    pub resize_orig_height: u32,

    // Convert tool state
    pub convert_format: String,
    /// User-specified output path for converted image. None = same directory as source.
    pub convert_output_path: Option<String>,

    // Filter drag state
    pub filter_dragging: Option<FilterSlider>,
    pub filter_dirty: bool,

    // Batch state
    pub batch_mode_resize: bool,
    pub batch_width: u32,
    pub batch_height: u32,
    pub batch_format: String,
    pub batch_selected: HashSet<usize>,

    // Compare state
    pub compare_split: f64,
    pub compare_dragging: bool,
    pub compare_mode: CompareMode,

    // Slideshow transition state
    pub slideshow_timer: Option<tench_ui::anim::AnimInterval>,
    /// Previous image data for fade transition during slideshow.
    pub slideshow_prev_image: Option<peniko::ImageData>,
    /// Fade alpha for cross-fade transition (0.0 = show previous, 1.0 = show current).
    pub slideshow_fade_alpha: f64,
    /// Timer for the fade transition.
    pub slideshow_fade_timer: Option<tench_ui::anim::AnimInterval>,
    /// Batch progress tracking
    pub batch_progress: Option<(usize, usize)>, // (completed, total)
    pub batch_running: bool,
    /// Custom output folder for batch operations (empty = same as source).
    pub batch_output_folder: String,

    // Drag state for panning
    pub drag_state: Option<DragState>,

    // Thumbnail size
    pub thumbnail_size: u32,

    // Recent files
    pub recent_files: Vec<String>,

    // Folder bookmarks (pinned folders)
    pub folder_bookmarks: Vec<String>,

    // Search
    pub show_search: bool,
    pub search_query: String,

    // Animated playing
    pub animated_playing: bool,

    // Double-click detection
    pub last_click_time: Option<std::time::Instant>,
    pub last_click_pos: Option<(f64, f64)>,

    // Rename dialog state
    pub show_rename: bool,
    pub rename_input_text: String,
    pub rename_original_name: String,

    // URL dialog state
    pub show_url_dialog: bool,
    pub url_input_text: String,

    // Print dialog state
    pub show_print_dialog: bool,
    pub print_paper_size: String,
    pub print_orientation: String,
    pub print_scaling: String,

    // Help overlay state
    pub show_help: bool,

    // AI result text (for Tag/Describe)
    pub ai_result_text: Option<String>,

    // Image rating (0 = unrated, 1-5 stars)
    pub image_rating: u8,

    // Image tags
    pub image_tags: Vec<String>,

    // Slideshow shuffle mode
    pub slideshow_shuffle: bool,
    /// Slideshow transition effect.
    pub slideshow_transition: SlideshowTransition,
    /// Whether slideshow loops back to start after the last image.
    pub slideshow_loop: bool,

    // Status message auto-dismiss timer
    pub status_message_time: Option<std::time::Instant>,

    // Pixel info on hover
    pub pixel_info: Option<PixelInfo>,

    // Annotations
    pub annotations: Vec<Annotation>,
    pub active_annotation_tool: Option<AnnotationTool>,
    pub annotation_color: Color,
    /// Whether the annotation color picker is visible.
    pub show_annotation_color_picker: bool,
    /// Line width for annotation drawing.
    pub annotation_line_width: f32,
    /// Undo stack for annotations (each entry is a snapshot of the full annotation list).
    pub annotation_undo_stack: Vec<Vec<Annotation>>,
    /// Redo stack for annotations.
    pub annotation_redo_stack: Vec<Vec<Annotation>>,
    /// Whether an annotation drag is in progress.
    pub annotation_dragging: bool,
    /// Start position for annotation drag (image pixel coords).
    pub annotation_drag_start: Option<(f64, f64)>,
    /// Text input for text annotation tool.
    pub annotation_text_input: Option<String>,
    /// Whether the annotation exit confirmation dialog is visible.
    pub show_annotation_exit_confirm: bool,

    // Thumbnail scroll offset for virtual scrolling
    pub thumbnail_scroll_offset: f64,

    // Thumbnail image cache: path -> small peniko::ImageData for filmstrip
    pub thumbnail_cache: HashMap<String, peniko::ImageData>,

    // Settings panel state
    pub show_settings: bool,
    pub settings_tab: SettingsTab,

    // Checkerboard background for transparent images
    pub checkerboard_bg: bool,

    // Crop aspect ratio constraint
    pub crop_aspect_ratio: Option<(u32, u32)>,

    // Hamburger menu
    pub show_menu: bool,

    // Click regions registered during the last paint frame.
    /// Cleared at the start of each paint, populated during paint.
    pub click_regions: Vec<ClickRegion>,
}
/// Filter slider identifier for drag tracking.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterSlider {
    Brightness,
    Contrast,
    Saturation,
    Blur,
    HueRotate,
}

/// Drag state for image panning.
#[derive(Debug, Clone, Copy)]
pub struct DragState {
    pub start_x: f64,
    pub start_y: f64,
    pub pan_x: f64,
    pub pan_y: f64,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            document: None,
            folder_entries: Vec::new(),
            sorted_entries: Vec::new(),
            current_image_data: None,
            original_image_data: None,
            exif_tags: Vec::new(),
            histogram: None,
            img_transform: kurbo::Affine::IDENTITY,
            user_transform: kurbo::Affine::IDENTITY,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            fit_mode: FitMode::Fit,
            rotation: 0,
            bg_color: BgColor::Black,
            show_chrome: false,
            show_metadata: false,
            show_thumbnails: true,
            show_quick_edit: false,
            show_delete_confirm: false,
            show_filter: false,
            show_compare: false,
            show_batch: false,
            show_ai: false,
            show_file_info: false,
            show_context_menu: false,
            context_menu_x: 0.0,
            context_menu_y: 0.0,
            active_edit_tool: None,
            edit_saving: false,
            has_edited_image: false,
            edit_history: Vec::new(),
            edit_history_index: 0,
            filter_brightness: 100.0,
            filter_contrast: 100.0,
            filter_saturation: 100.0,
            filter_blur: 0.0,
            filter_hue_rotate: 0.0,
            sort_key: SortKey::Name,
            sort_order: SortOrder::Asc,
            slideshow_playing: false,
            slideshow_interval_ms: 3000,
            ai_running: false,
            ai_selected_feature: None,
            is_loading: false,
            status_message: "Ready".to_string(),
            crop_start: None,
            crop_selection: None,
            crop_dragging: false,
            resize_width: 0,
            resize_height: 0,
            resize_maintain_aspect: true,
            resize_orig_width: 0,
            resize_orig_height: 0,
            convert_format: "png".to_string(),
            convert_output_path: None,
            batch_mode_resize: true,
            batch_width: 1920,
            batch_height: 1080,
            batch_format: "png".to_string(),
            batch_selected: HashSet::new(),
            compare_split: 50.0,
            compare_dragging: false,
            compare_mode: CompareMode::Split,
            slideshow_timer: None,
            slideshow_prev_image: None,
            slideshow_fade_alpha: 1.0,
            slideshow_fade_timer: None,
            batch_progress: None,
            batch_running: false,
            batch_output_folder: String::new(),
            drag_state: None,
            thumbnail_size: 120,
            recent_files: Vec::new(),
            folder_bookmarks: Vec::new(),
            show_search: false,
            search_query: String::new(),
            animated_playing: true,
            last_click_time: None,
            last_click_pos: None,
            thumbnail_scroll_offset: 0.0,
            show_rename: false,
            rename_input_text: String::new(),
            rename_original_name: String::new(),
            show_url_dialog: false,
            url_input_text: String::new(),
            show_print_dialog: false,
            print_paper_size: "A4".to_string(),
            print_orientation: "Portrait".to_string(),
            print_scaling: "Fit to page".to_string(),
            show_help: false,
            ai_result_text: None,
            image_rating: 0,
            image_tags: Vec::new(),
            slideshow_shuffle: false,
            slideshow_transition: SlideshowTransition::Fade,
            slideshow_loop: true,
            status_message_time: None,
            pixel_info: None,
            annotations: Vec::new(),
            active_annotation_tool: None,
            annotation_color: Color::rgb8(0xFF, 0x00, 0x00),
            show_annotation_color_picker: false,
            annotation_line_width: 2.0,
            annotation_undo_stack: Vec::new(),
            annotation_redo_stack: Vec::new(),
            annotation_dragging: false,
            annotation_drag_start: None,
            annotation_text_input: None,
            show_annotation_exit_confirm: false,
            thumbnail_cache: HashMap::new(),
            click_regions: Vec::new(),
            filter_dragging: None,
            filter_dirty: false,
            show_settings: false,
            settings_tab: SettingsTab::General,
            checkerboard_bg: false,
            crop_aspect_ratio: None,
            show_menu: false,
        }
    }
}

mod ops;

pub use ops::{bytes_label, dimensions_label};

#[cfg(test)]
mod tests;
