#[test]
fn view_and_pixel_runtime_apis_are_public() {
    let products = tench_image_runtime::image_runtime_products();
    assert_eq!(products.len(), 2);
    assert!(products
        .iter()
        .any(|product| product.product_id == "tench-view"));

    let metadata = tench_image_runtime::view::service::image_file_metadata("missing.png");
    assert_eq!(metadata.file_name, "missing.png");
    assert_eq!(metadata.format, "png");

    let report = tench_image_runtime::view::service::batch_convert_images(&[], "png");
    assert_eq!(report.attempted, 0);
}
