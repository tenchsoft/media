use tench_image_core::{
    detect_format, metadata, types, ColorProfileInfo, ImageCoreError, ImageDimensions, ImageFormat,
    ImageMetadata, MetadataTag, ThumbnailRequest,
};

#[test]
fn root_reexports_legacy_image_types_and_helpers() {
    let dimensions = ImageDimensions {
        width: 1920,
        height: 1080,
    };
    let metadata = ImageMetadata {
        path: "photo.png".to_string(),
        file_name: "photo.png".to_string(),
        format: ImageFormat::Png,
        dimensions: Some(dimensions),
        color_type: Some("rgba8".to_string()),
        color_profile: ColorProfileInfo {
            embedded: false,
            description: None,
        },
        tags: vec![MetadataTag {
            group: "File".to_string(),
            name: "Name".to_string(),
            value: "photo.png".to_string(),
        }],
    };
    let request = ThumbnailRequest {
        path: metadata.path.clone(),
        max_size: 256,
    };
    let error = ImageCoreError::new("sample", "sample message");

    assert_eq!(metadata.format, detect_format(&request.path));
    assert_eq!(error.code, "sample");
}

#[test]
fn canonical_modules_export_the_same_public_types() {
    let root_format = ImageFormat::Jpeg;
    let module_format = types::ImageFormat::Jpeg;

    assert_eq!(root_format, module_format);
    assert_eq!(metadata::detect_format("cover.webp"), ImageFormat::Webp);
}
