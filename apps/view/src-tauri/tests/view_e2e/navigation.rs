use super::*;

// ---------------------------------------------------------------------------
// Navigation (work plan items 23-24)
// ---------------------------------------------------------------------------

#[test]
fn view_navigation_buttons_exist_with_image_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    // With an image loaded in the middle of 3 entries, both nav buttons exist
    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.nav.prev".to_string(),
            }
        )
        .is_some(),
        "Expected view.nav.prev in tree with image loaded"
    );
    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.nav.next".to_string(),
            }
        )
        .is_some(),
        "Expected view.nav.next in tree with image loaded"
    );
}

#[test]
fn view_navigation_buttons_not_present_without_image_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.nav.prev".to_string(),
            }
        )
        .is_none(),
        "Should NOT have view.nav.prev without image"
    );
    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.nav.next".to_string(),
            }
        )
        .is_none(),
        "Should NOT have view.nav.next without image"
    );
}

#[test]
fn view_with_image_shows_additional_top_buttons_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    // With image loaded, these buttons should now exist
    let image_only_selectors = ["view.top.files", "view.top.copy_path", "view.top.copy_img"];
    for selector in &image_only_selectors {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: selector.to_string(),
                }
            )
            .is_some(),
            "Expected automation node with debug_id={selector} with image loaded"
        );
    }
}

#[test]
fn view_with_image_renders_nonblank_ui_e2e() {
    let mut harness = view_harness_with_image();
    let capture = harness
        .automation_action(UiAutomationAction::Capture {
            request: UiAutomationCaptureRequest {
                include_png: true,
                include_tree: true,
            },
        })
        .expect("capture with image");

    let img = image::load_from_memory(&capture.png_bytes)
        .expect("decode PNG")
        .into_rgba8();
    assert!(
        is_nonblank(&img, 32),
        "render with image should contain painted pixels"
    );
}
