use serde::{Deserialize, Serialize};
use tench_media_core::{MediaEntry, MediaError};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageFormat {
    Avif,
    Bmp,
    Gif,
    Ico,
    Jpeg,
    Png,
    Pnm,
    Tga,
    Tiff,
    Webp,
    Svg,
    Heif,
    JpegXl,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColorProfileInfo {
    pub embedded: bool,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetadataTag {
    pub group: String,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub path: String,
    pub file_name: String,
    pub format: ImageFormat,
    pub dimensions: Option<ImageDimensions>,
    pub color_type: Option<String>,
    pub color_profile: ColorProfileInfo,
    pub tags: Vec<MetadataTag>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageEntry {
    pub media: MediaEntry,
    pub format: ImageFormat,
    pub dimensions: Option<ImageDimensions>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageDocument {
    pub entry: ImageEntry,
    pub metadata: ImageMetadata,
    pub folder: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ThumbnailRequest {
    pub path: String,
    pub max_size: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ThumbnailResult {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub data_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageCoreError {
    pub code: String,
    pub message: String,
}

impl ImageCoreError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ImageCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ImageCoreError {}

impl From<MediaError> for ImageCoreError {
    fn from(error: MediaError) -> Self {
        Self::new(error.code, error.message)
    }
}
