use tench_pixel_core::{BrushStroke, Document, DocumentLayer, PixelBuffer};
use tench_ui::prelude::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Persona {
    Edit,
    AI,
    Adjust,
    Export,
}

impl Persona {
    pub const ALL: [Persona; 4] = [Persona::Edit, Persona::AI, Persona::Adjust, Persona::Export];

    pub fn label(self) -> &'static str {
        match self {
            Persona::Edit => "Edit",
            Persona::AI => "AI",
            Persona::Adjust => "Adjust",
            Persona::Export => "Export",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Move,
    Select,
    Brush,
    Eraser,
    Crop,
    Text,
    Shape,
    Fill,
    Gradient,
    Hand,
    Eyedropper,
}

impl Tool {
    pub const ALL: [Tool; 11] = [
        Tool::Move,
        Tool::Select,
        Tool::Brush,
        Tool::Eraser,
        Tool::Crop,
        Tool::Text,
        Tool::Shape,
        Tool::Fill,
        Tool::Gradient,
        Tool::Hand,
        Tool::Eyedropper,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Tool::Move => "Move",
            Tool::Select => "Select",
            Tool::Brush => "Brush",
            Tool::Eraser => "Eraser",
            Tool::Crop => "Crop",
            Tool::Text => "Text",
            Tool::Shape => "Shape",
            Tool::Fill => "Fill",
            Tool::Gradient => "Gradient",
            Tool::Hand => "Hand",
            Tool::Eyedropper => "Eyedropper",
        }
    }

    pub fn glyph(self) -> &'static str {
        match self {
            Tool::Move => "MV",
            Tool::Select => "SE",
            Tool::Brush => "BR",
            Tool::Eraser => "ER",
            Tool::Crop => "CR",
            Tool::Text => "T",
            Tool::Shape => "SH",
            Tool::Fill => "FL",
            Tool::Gradient => "GR",
            Tool::Hand => "HD",
            Tool::Eyedropper => "ED",
        }
    }

    pub fn shortcut(self) -> &'static str {
        match self {
            Tool::Move => "V",
            Tool::Select => "M",
            Tool::Brush => "B",
            Tool::Eraser => "E",
            Tool::Crop => "C",
            Tool::Text => "T",
            Tool::Shape => "U",
            Tool::Fill => "G",
            Tool::Gradient => "D",
            Tool::Hand => "H",
            Tool::Eyedropper => "I",
        }
    }

    pub fn from_shortcut(input: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|tool| tool.shortcut().eq_ignore_ascii_case(input))
    }

    pub fn uses_brush(self) -> bool {
        matches!(self, Tool::Brush | Tool::Eraser)
    }

    pub fn context_options(
        self,
        brush_size: u32,
        brush_opacity: u32,
        brush_hardness: u32,
    ) -> Vec<String> {
        match self {
            Tool::Move => vec!["Auto-select: Layer".into(), "Transform: On".into()],
            Tool::Select => vec![
                "Mode: New".into(),
                "Feather: 0px".into(),
                "Anti-alias: On".into(),
            ],
            Tool::Brush => vec![
                format!("Size: {brush_size}px"),
                format!("Hardness: {brush_hardness}%"),
                format!("Opacity: {brush_opacity}%"),
            ],
            Tool::Eraser => vec![
                format!("Size: {brush_size}px"),
                format!("Opacity: {brush_opacity}%"),
            ],
            Tool::Crop => vec!["Ratio: Free".into(), "Delete cropped: On".into()],
            Tool::Text => vec![
                "Font: Inter".into(),
                "Size: 16px".into(),
                "Color: foreground".into(),
            ],
            Tool::Shape => vec![
                "Type: Rectangle".into(),
                "Fill: accent".into(),
                "Stroke: 2px".into(),
            ],
            Tool::Fill => vec![
                "Mode: Foreground".into(),
                "Tolerance: 32".into(),
                "Anti-alias: On".into(),
            ],
            Tool::Gradient => vec!["Type: Linear".into(), "Colors: FG to BG".into()],
            Tool::Hand => vec!["Scroll All: Off".into()],
            Tool::Eyedropper => vec!["Sample: All Layers".into()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Rectangle,
    Ellipse,
    Line,
}

impl ShapeType {
    pub const ALL: [ShapeType; 3] = [ShapeType::Rectangle, ShapeType::Ellipse, ShapeType::Line];
    pub fn label(self) -> &'static str {
        match self {
            ShapeType::Rectangle => "Rectangle",
            ShapeType::Ellipse => "Ellipse",
            ShapeType::Line => "Line",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiTool {
    Inpaint,
    Outpaint,
    BgRemove,
    Upscale,
    Denoise,
    GenFill,
    StyleTransfer,
}

impl AiTool {
    pub const ALL: [AiTool; 7] = [
        AiTool::Inpaint,
        AiTool::Outpaint,
        AiTool::BgRemove,
        AiTool::Upscale,
        AiTool::Denoise,
        AiTool::GenFill,
        AiTool::StyleTransfer,
    ];

    pub fn label(self) -> &'static str {
        match self {
            AiTool::Inpaint => "Inpaint",
            AiTool::Outpaint => "Outpaint",
            AiTool::BgRemove => "BG Remove",
            AiTool::Upscale => "Upscale",
            AiTool::Denoise => "Denoise",
            AiTool::GenFill => "Gen Fill",
            AiTool::StyleTransfer => "Style Transfer",
        }
    }

    pub fn glyph(self) -> &'static str {
        match self {
            AiTool::Inpaint => "IP",
            AiTool::Outpaint => "OP",
            AiTool::BgRemove => "BG",
            AiTool::Upscale => "UP",
            AiTool::Denoise => "DN",
            AiTool::GenFill => "GF",
            AiTool::StyleTransfer => "ST",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelTab {
    Layers,
    Properties,
    History,
}

impl PanelTab {
    pub const ALL: [PanelTab; 3] = [PanelTab::Layers, PanelTab::Properties, PanelTab::History];

    pub fn label(self) -> &'static str {
        match self {
            PanelTab::Layers => "Layers",
            PanelTab::Properties => "Properties",
            PanelTab::History => "History",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Done,
    Failed,
}

impl JobStatus {
    pub fn label(self, progress: u32) -> String {
        match self {
            JobStatus::Queued => "Queued".into(),
            JobStatus::Running => format!("{progress}%"),
            JobStatus::Done => "Done".into(),
            JobStatus::Failed => "Failed".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AiJob {
    pub id: String,
    pub tool: AiTool,
    pub label: String,
    pub status: JobStatus,
    pub progress: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct BrushPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub size: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct CanvasPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct SelectionRect {
    pub start: CanvasPoint,
    pub end: CanvasPoint,
}

/// Extended adjust values with additional filters.
#[derive(Debug, Clone, Copy)]
pub struct AdjustValues {
    pub brightness: i32,
    pub contrast: i32,
    pub saturation: i32,
    pub temperature: i32,
    pub hue: i32,
    pub sharpness: i32,
    pub blur: i32,
    pub levels: i32,
}

impl AdjustValues {
    pub fn nudge(&mut self, index: usize, delta: i32) {
        let slot = match index {
            0 => &mut self.brightness,
            1 => &mut self.contrast,
            2 => &mut self.saturation,
            3 => &mut self.temperature,
            4 => &mut self.hue,
            5 => &mut self.sharpness,
            6 => &mut self.blur,
            7 => &mut self.levels,
            _ => return,
        };
        *slot = (*slot + delta).clamp(-100, 100);
    }

    pub fn rows(self) -> [(&'static str, i32); 8] {
        [
            ("Brightness", self.brightness),
            ("Contrast", self.contrast),
            ("Saturation", self.saturation),
            ("Temperature", self.temperature),
            ("Hue", self.hue),
            ("Sharpness", self.sharpness),
            ("Blur", self.blur),
            ("Levels", self.levels),
        ]
    }
}

/// A snapshot for undo/redo.
#[derive(Debug, Clone)]
pub struct DocumentSnapshot {
    pub document: Document,
    pub label: String,
}

/// Per-layer adjustment storage.
#[derive(Debug, Clone)]
pub struct LayerAdjustments {
    pub brightness: i32,
    pub contrast: i32,
    pub saturation: i32,
    pub temperature: i32,
    pub hue: i32,
    pub sharpness: i32,
    pub blur: i32,
    pub levels: i32,
}

impl Default for LayerAdjustments {
    // derivable_impls: keep manual impl for clarity with explicit zero values
    #[allow(clippy::derivable_impls)]
    fn default() -> Self {
        Self {
            brightness: 0,
            contrast: 0,
            saturation: 0,
            temperature: 0,
            hue: 0,
            sharpness: 0,
            blur: 0,
            levels: 0,
        }
    }
}

/// Modal dialog type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    None,
    ColorPicker,
    BrushSettings,
    NewDocument,
    LayerRename,
    BlendModeSelect,
}

#[derive(Debug, Clone)]
pub struct PixelDesignState {
    pub persona: Persona,
    pub active_tool: Tool,
    pub document: Document,
    pub composited_image: Option<PixelBuffer>,
    pub zoom: u32,
    pub brush_preset: &'static str,
    pub brush_size: u32,
    pub brush_opacity: u32,
    pub brush_hardness: u32,
    pub fg_color: Color,
    pub bg_color: Color,
    pub export_format: String,
    pub export_quality: u32,
    pub export_scale: u32,
    pub status_msg: String,
    pub selection: Option<SelectionRect>,
    pub is_drawing: bool,
    pub mouse_pos: CanvasPoint,
    pub ai_jobs: Vec<AiJob>,
    pub expanded_ai: AiTool,
    pub ai_prompt: String,
    pub ai_prompt_focused: bool,
    pub active_adjust: Option<String>,
    pub adjust_values: AdjustValues,
    pub text_input: String,
    pub show_text_input: bool,
    pub text_pos: CanvasPoint,
    pub panel_tab: PanelTab,
    pub current_stroke: Option<BrushStroke>,

    // Phase 1: Real undo/redo
    pub history: Vec<DocumentSnapshot>,
    pub history_index: usize,

    // Phase 1: Shape tool
    pub shape_type: ShapeType,
    pub shape_drag_start: Option<CanvasPoint>,

    // Phase 1: Move tool drag
    pub move_drag_start: Option<CanvasPoint>,
    pub move_layer_start_offset: Option<(i32, i32)>,

    // Phase 1: Hand tool / viewport pan
    pub viewport_offset_x: f64,
    pub viewport_offset_y: f64,
    pub is_panning: bool,
    pub pan_start: Option<CanvasPoint>,
    pub space_held: bool,

    // Phase 2: Color picker
    pub show_color_picker: bool,
    pub color_picker_target_fg: bool,
    pub color_picker_original: Color,
    pub color_picker_preview: Color,
    pub color_hue: f32,
    pub color_saturation: f32,
    pub color_value: f32,
    pub color_palette: Vec<Color>,
    pub recent_colors: Vec<Color>,

    // Phase 3: Layer management
    pub renaming_layer_idx: Option<usize>,
    pub rename_input: String,
    pub blend_mode_dropdown_open: bool,
    pub layer_context_menu_idx: Option<usize>,

    // Phase 4: Per-layer adjustments
    pub layer_adjustments: std::collections::HashMap<String, LayerAdjustments>,

    // Phase 5: File paths
    pub pending_file_action: Option<FileAction>,

    // Phase 6: AI
    pub ai_cancel_requested: bool,
    pub ai_model: String,

    // Phase 7: UI state
    pub panels_visible: bool,
    pub fullscreen: bool,
    pub canvas_flipped_h: bool,
    pub canvas_flipped_v: bool,
    pub canvas_rotation: f64,
    pub show_grid: bool,
    pub grid_size: u32,
    pub show_rulers: bool,
    pub dragging_zoom_slider: bool,

    // Phase 8: Modal
    pub active_modal: ModalType,

    // Layer thumbnails
    pub layer_thumbnails: std::collections::HashMap<String, PixelBuffer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAction {
    Open,
    SaveAs,
    Export,
}

impl PixelDesignState {
    // new_without_default: PixelDesignState has complex initialization with document snapshots
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let doc = Document::new(800, 600);
        let initial_snapshot = DocumentSnapshot {
            document: doc.clone(),
            label: "Initial".into(),
        };
        Self {
            persona: Persona::Edit,
            active_tool: Tool::Brush,
            document: doc,
            zoom: 100,
            brush_preset: "bp2",
            brush_size: 24,
            brush_opacity: 100,
            brush_hardness: 80,
            fg_color: Color::rgb8(0xFF, 0xFF, 0xFF),
            bg_color: Color::rgb8(0x00, 0x00, 0x00),
            export_format: "PNG".into(),
            export_quality: 92,
            export_scale: 100,
            composited_image: None,
            status_msg: "Ready".into(),
            selection: None,
            is_drawing: false,
            mouse_pos: CanvasPoint { x: 0.0, y: 0.0 },
            ai_jobs: vec![],
            expanded_ai: AiTool::Inpaint,
            ai_prompt: "Repair selected area".into(),
            ai_prompt_focused: false,
            active_adjust: None,
            adjust_values: AdjustValues {
                brightness: 0,
                contrast: 0,
                saturation: 0,
                temperature: 0,
                hue: 0,
                sharpness: 0,
                blur: 0,
                levels: 0,
            },
            text_input: String::new(),
            show_text_input: false,
            text_pos: CanvasPoint { x: 0.0, y: 0.0 },
            panel_tab: PanelTab::Layers,
            current_stroke: None,
            history: vec![initial_snapshot],
            history_index: 0,
            shape_type: ShapeType::Rectangle,
            shape_drag_start: None,
            move_drag_start: None,
            move_layer_start_offset: None,
            viewport_offset_x: 0.0,
            viewport_offset_y: 0.0,
            is_panning: false,
            pan_start: None,
            space_held: false,
            show_color_picker: false,
            color_picker_target_fg: true,
            color_picker_original: Color::rgb8(0xFF, 0xFF, 0xFF),
            color_picker_preview: Color::rgb8(0xFF, 0xFF, 0xFF),
            color_hue: 0.0,
            color_saturation: 1.0,
            color_value: 1.0,
            color_palette: vec![
                Color::rgb8(0xFF, 0xFF, 0xFF),
                Color::rgb8(0x00, 0x00, 0x00),
                Color::rgb8(0xFF, 0x00, 0x00),
                Color::rgb8(0x00, 0xFF, 0x00),
                Color::rgb8(0x00, 0x00, 0xFF),
                Color::rgb8(0xFF, 0xFF, 0x00),
                Color::rgb8(0xFF, 0x00, 0xFF),
                Color::rgb8(0x00, 0xFF, 0xFF),
                Color::rgb8(0x60, 0xA5, 0xFA),
                Color::rgb8(0x22, 0xC5, 0x5E),
                Color::rgb8(0xF5, 0x9E, 0x0B),
                Color::rgb8(0xEF, 0x44, 0x44),
            ],
            recent_colors: vec![],
            renaming_layer_idx: None,
            rename_input: String::new(),
            blend_mode_dropdown_open: false,
            layer_context_menu_idx: None,
            layer_adjustments: std::collections::HashMap::new(),
            pending_file_action: None,
            ai_cancel_requested: false,
            ai_model: "Default".into(),
            panels_visible: true,
            fullscreen: false,
            canvas_flipped_h: false,
            canvas_flipped_v: false,
            canvas_rotation: 0.0,
            show_grid: false,
            grid_size: 16,
            show_rulers: false,
            dragging_zoom_slider: false,
            active_modal: ModalType::None,
            layer_thumbnails: std::collections::HashMap::new(),
        }
    }

    pub fn brush_presets() -> &'static [BrushPreset] {
        &[
            BrushPreset {
                id: "bp1",
                name: "Round",
                size: 12,
            },
            BrushPreset {
                id: "bp2",
                name: "Soft",
                size: 24,
            },
            BrushPreset {
                id: "bp3",
                name: "Flat",
                size: 36,
            },
            BrushPreset {
                id: "bp4",
                name: "Detail",
                size: 4,
            },
            BrushPreset {
                id: "bp5",
                name: "Texture",
                size: 18,
            },
            BrushPreset {
                id: "bp6",
                name: "Airbrush",
                size: 30,
            },
        ]
    }

    pub fn adjust_presets() -> &'static [&'static str] {
        &[
            "Warm", "Cool", "B&W", "Vintage", "Vivid", "Muted", "Film", "HDR",
        ]
    }

    pub fn active_layer_index(&self) -> usize {
        self.document.active_layer_index()
    }

    pub fn active_layer(&self) -> Option<&DocumentLayer> {
        self.document.active_layer()
    }

    pub fn select_persona(&mut self, persona: Persona) {
        self.persona = persona;
        self.status_msg = format!("{} workspace", persona.label());
    }

    pub fn set_active_tool(&mut self, tool: Tool) {
        self.persona = Persona::Edit;
        self.active_tool = tool;
        self.status_msg = format!("Tool: {}", tool.label());
        if !matches!(tool, Tool::Text) {
            self.show_text_input = false;
        }
        // Close any open dropdowns
        self.blend_mode_dropdown_open = false;
        self.layer_context_menu_idx = None;
    }

    pub fn set_brush_preset(&mut self, id: &'static str) {
        if let Some(preset) = Self::brush_presets().iter().find(|preset| preset.id == id) {
            self.brush_preset = preset.id;
            self.brush_size = preset.size;
            self.status_msg = format!("Brush preset: {}", preset.name);
        }
    }
}
