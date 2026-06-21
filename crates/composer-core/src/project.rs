//! Composer project: persistence, settings, media bin.

use crate::*;
use serde::{Deserialize, Serialize};

/// A media file in the project's media bin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: u64,
    pub path: String,
    pub name: String,
    pub media_type: MediaType,
    pub duration_frames: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<f64>,
    pub audio_channels: Option<u32>,
    pub audio_sample_rate: Option<u32>,
    pub file_size: u64,
}

/// Media type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    Video,
    Audio,
    Image,
}

/// Export settings for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub format: ExportFormat,
    pub codec: VideoCodec,
    pub audio_codec: AudioCodec,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub bitrate_kbps: u32,
    pub audio_bitrate_kbps: u32,
    pub two_pass: bool,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            format: ExportFormat::Mp4,
            codec: VideoCodec::H264,
            audio_codec: AudioCodec::Aac,
            width: 1920,
            height: 1080,
            fps: 24.0,
            bitrate_kbps: 8000,
            audio_bitrate_kbps: 192,
            two_pass: false,
        }
    }
}

/// Export container format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    Mp4,
    Mov,
    WebM,
    Avi,
    Mkv,
}

impl ExportFormat {
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Mp4 => "mp4",
            Self::Mov => "mov",
            Self::WebM => "webm",
            Self::Avi => "avi",
            Self::Mkv => "mkv",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Mp4 => "MP4",
            Self::Mov => "MOV",
            Self::WebM => "WebM",
            Self::Avi => "AVI",
            Self::Mkv => "MKV",
        }
    }
}

/// Video codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    H264,
    H265,
    Vp9,
    Av1,
    ProRes,
}

impl VideoCodec {
    pub const fn label(self) -> &'static str {
        match self {
            Self::H264 => "H.264",
            Self::H265 => "H.265 (HEVC)",
            Self::Vp9 => "VP9",
            Self::Av1 => "AV1",
            Self::ProRes => "ProRes",
        }
    }
}

/// Audio codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioCodec {
    Aac,
    Opus,
    Flac,
    Pcm,
}

impl AudioCodec {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Aac => "AAC",
            Self::Opus => "Opus",
            Self::Flac => "FLAC",
            Self::Pcm => "PCM",
        }
    }
}

/// Project template preset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub name: String,
    pub category: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub description: String,
}

impl ProjectTemplate {
    pub fn built_in() -> Vec<Self> {
        vec![
            Self {
                name: "Full HD 24fps".into(),
                category: "Film".into(),
                width: 1920,
                height: 1080,
                fps: 24.0,
                description: "Standard film at 1920x1080, 24fps".into(),
            },
            Self {
                name: "Full HD 30fps".into(),
                category: "Web".into(),
                width: 1920,
                height: 1080,
                fps: 30.0,
                description: "Web video at 1920x1080, 30fps".into(),
            },
            Self {
                name: "4K 60fps".into(),
                category: "High Quality".into(),
                width: 3840,
                height: 2160,
                fps: 60.0,
                description: "Ultra HD at 3840x2160, 60fps".into(),
            },
            Self {
                name: "Vertical 9:16".into(),
                category: "Social".into(),
                width: 1080,
                height: 1920,
                fps: 30.0,
                description: "Vertical video for social media".into(),
            },
            Self {
                name: "Square 1:1".into(),
                category: "Social".into(),
                width: 1080,
                height: 1080,
                fps: 30.0,
                description: "Square video for Instagram".into(),
            },
        ]
    }
}

/// A render job in the queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderJob {
    pub id: u64,
    pub name: String,
    pub settings: ExportSettings,
    pub progress: u8,
    pub status: RenderStatus,
}

/// Render job status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderStatus {
    Queued,
    Rendering,
    Completed,
    Failed,
}

/// The full Composer project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposerProject {
    pub name: String,
    pub timeline: Timeline,
    pub media_bin: Vec<MediaAsset>,
    pub effects: Vec<Effect>,
    pub transitions: Vec<Transition>,
    pub export_settings: ExportSettings,
    pub templates: Vec<ProjectTemplate>,
    pub render_queue: Vec<RenderJob>,
    /// Next unique ID for auto-increment.
    pub next_id: u64,
}

impl ComposerProject {
    pub fn new(name: String) -> Self {
        let mut next_id = 1u64;
        let timeline = Timeline::default_tracks(&mut next_id);
        Self {
            name,
            timeline,
            media_bin: Vec::new(),
            effects: Vec::new(),
            transitions: Vec::new(),
            export_settings: ExportSettings::default(),
            templates: ProjectTemplate::built_in(),
            render_queue: Vec::new(),
            next_id,
        }
    }

    /// Generate a new unique ID.
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Serialize project to JSON string.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// Deserialize project from JSON string.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    /// Add a media asset to the bin.
    pub fn add_media(&mut self, asset: MediaAsset) {
        self.media_bin.push(asset);
    }

    /// Remove a media asset by ID.
    pub fn remove_media(&mut self, id: u64) -> bool {
        let before = self.media_bin.len();
        self.media_bin.retain(|a| a.id != id);
        self.media_bin.len() != before
    }

    /// Add a render job to the queue.
    pub fn enqueue_render(&mut self, name: String, settings: ExportSettings) -> u64 {
        let id = self.next_id();
        self.render_queue.push(RenderJob {
            id,
            name,
            settings,
            progress: 0,
            status: RenderStatus::Queued,
        });
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_project_has_default_tracks() {
        let project = ComposerProject::new("Test".into());
        assert_eq!(project.timeline.tracks.len(), 5);
        assert_eq!(project.name, "Test");
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let project = ComposerProject::new("Roundtrip".into());
        let json = project.to_json().unwrap();
        let restored = ComposerProject::from_json(&json).unwrap();
        assert_eq!(restored.name, "Roundtrip");
        assert_eq!(restored.timeline.tracks.len(), 5);
    }

    #[test]
    fn next_id_increments() {
        let mut project = ComposerProject::new("Test".into());
        let id1 = project.next_id();
        let id2 = project.next_id();
        assert!(id2 > id1);
    }

    #[test]
    fn add_and_remove_media() {
        let mut project = ComposerProject::new("Test".into());
        let id = project.next_id();
        project.add_media(MediaAsset {
            id,
            path: "/test.mp4".into(),
            name: "test.mp4".into(),
            media_type: MediaType::Video,
            duration_frames: Some(240),
            width: Some(1920),
            height: Some(1080),
            fps: Some(24.0),
            audio_channels: Some(2),
            audio_sample_rate: Some(48000),
            file_size: 1024,
        });
        assert_eq!(project.media_bin.len(), 1);
        assert!(project.remove_media(id));
        assert!(project.media_bin.is_empty());
    }

    #[test]
    fn enqueue_render() {
        let mut project = ComposerProject::new("Test".into());
        let id = project.enqueue_render("output.mp4".into(), ExportSettings::default());
        assert_eq!(project.render_queue.len(), 1);
        assert_eq!(project.render_queue[0].id, id);
        assert_eq!(project.render_queue[0].status, RenderStatus::Queued);
    }

    #[test]
    fn built_in_templates() {
        let templates = ProjectTemplate::built_in();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.name.contains("Full HD")));
    }

    #[test]
    fn export_format_extensions() {
        assert_eq!(ExportFormat::Mp4.extension(), "mp4");
        assert_eq!(ExportFormat::WebM.extension(), "webm");
    }
}
