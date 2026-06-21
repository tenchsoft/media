#[test]
fn player_and_composer_runtime_apis_are_public() {
    let products = tench_media_runtime::media_runtime_products();
    assert_eq!(products.len(), 2);
    assert!(products
        .iter()
        .any(|product| product.product_id == "tench-composer"));

    let pixels = [
        255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
    ];
    let downsampled = tench_media_runtime::player::downsample_rgba(&pixels, 2, 2, 1, 1);
    assert_eq!(downsampled, vec![255, 0, 0, 255]);

    let project = tench_media_runtime::composer::new_project("Contract".to_string());
    assert_eq!(project.name, "Contract");
}
