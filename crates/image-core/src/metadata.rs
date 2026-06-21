use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::path::Path;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use image::{
    ColorType, DynamicImage, GenericImageView, ImageFormat as ImageCrateFormat, ImageReader,
};
use tench_media_core::{entry_for_path, MediaKind};

use crate::{
    ColorProfileInfo, ImageCoreError, ImageDimensions, ImageFormat, ImageMetadata, MetadataTag,
    ThumbnailRequest, ThumbnailResult,
};

pub fn read_metadata(path: impl AsRef<Path>) -> Result<ImageMetadata, ImageCoreError> {
    let path = path.as_ref();
    let entry = entry_for_path(path, MediaKind::Image)?;
    let format = detect_format(path);
    let dimensions = image_dimensions(path).ok();
    let color_type = image_color_type(path).ok().map(color_type_label);
    let tags = read_exif_tags(path).unwrap_or_default();

    Ok(ImageMetadata {
        path: entry.path,
        file_name: entry.file_name,
        format,
        dimensions,
        color_type,
        color_profile: ColorProfileInfo {
            embedded: false,
            description: None,
        },
        tags,
    })
}

pub fn generate_thumbnail(request: &ThumbnailRequest) -> Result<ThumbnailResult, ImageCoreError> {
    let max_size = request.max_size.clamp(32, 1024);
    let image = decode_image(&request.path)?;
    let thumbnail = image.thumbnail(max_size, max_size);
    let (width, height) = thumbnail.dimensions();
    let mut encoded = Vec::new();
    thumbnail
        .write_to(&mut Cursor::new(&mut encoded), ImageCrateFormat::Png)
        .map_err(|error| ImageCoreError::new("thumbnail_encode_failed", error.to_string()))?;

    Ok(ThumbnailResult {
        path: request.path.clone(),
        width,
        height,
        data_url: format!("data:image/png;base64,{}", STANDARD.encode(encoded)),
    })
}

pub fn detect_format(path: impl AsRef<Path>) -> ImageFormat {
    let path = path.as_ref();
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        Some("avif") => ImageFormat::Avif,
        Some("bmp") => ImageFormat::Bmp,
        Some("gif") => ImageFormat::Gif,
        Some("heic" | "heif") => ImageFormat::Heif,
        Some("ico" | "cur") => ImageFormat::Ico,
        Some("jpg" | "jpeg") => ImageFormat::Jpeg,
        Some("jxl") => ImageFormat::JpegXl,
        Some("png") => ImageFormat::Png,
        Some("pnm" | "pbm" | "pgm" | "ppm") => ImageFormat::Pnm,
        Some("svg") => ImageFormat::Svg,
        Some("tga") => ImageFormat::Tga,
        Some("tif" | "tiff") => ImageFormat::Tiff,
        Some("webp") => ImageFormat::Webp,
        _ => ImageFormat::Unknown,
    }
}

pub(crate) fn image_dimensions(path: impl AsRef<Path>) -> Result<ImageDimensions, ImageCoreError> {
    let (width, height) = image::image_dimensions(path.as_ref()).map_err(|error| {
        ImageCoreError::new(
            "image_dimensions_failed",
            format!("Failed to read image dimensions: {error}"),
        )
    })?;
    Ok(ImageDimensions { width, height })
}

fn image_color_type(path: impl AsRef<Path>) -> Result<ColorType, ImageCoreError> {
    decode_image(path).map(|image| image.color())
}

fn decode_image(path: impl AsRef<Path>) -> Result<DynamicImage, ImageCoreError> {
    ImageReader::open(path.as_ref())
        .map_err(|error| ImageCoreError::new("image_open_failed", error.to_string()))?
        .with_guessed_format()
        .map_err(|error| ImageCoreError::new("image_format_guess_failed", error.to_string()))?
        .decode()
        .map_err(|error| ImageCoreError::new("image_decode_failed", error.to_string()))
}

fn color_type_label(color_type: ColorType) -> String {
    match color_type {
        ColorType::L8 => "L8",
        ColorType::La8 => "La8",
        ColorType::Rgb8 => "Rgb8",
        ColorType::Rgba8 => "Rgba8",
        ColorType::L16 => "L16",
        ColorType::La16 => "La16",
        ColorType::Rgb16 => "Rgb16",
        ColorType::Rgba16 => "Rgba16",
        ColorType::Rgb32F => "Rgb32F",
        ColorType::Rgba32F => "Rgba32F",
        _ => "Unknown",
    }
    .to_string()
}

fn read_exif_tags(path: impl AsRef<Path>) -> Result<Vec<MetadataTag>, ImageCoreError> {
    let file = File::open(path.as_ref())
        .map_err(|error| ImageCoreError::new("exif_open_failed", error.to_string()))?;
    let mut reader = BufReader::new(file);
    let exif = exif::Reader::new()
        .read_from_container(&mut reader)
        .map_err(|error| ImageCoreError::new("exif_read_failed", error.to_string()))?;

    Ok(exif
        .fields()
        .take(64)
        .map(|field| MetadataTag {
            group: format!("{:?}", field.ifd_num),
            name: format!("{}", field.tag),
            value: field.display_value().with_unit(&exif).to_string(),
        })
        .collect())
}
