use super::*;

// ---------------------------------------------------------------------------
// Metadata panel (work plan items 110-121)
// ---------------------------------------------------------------------------

#[test]
fn view_metadata_panel_close_button_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_metadata = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.top.files".to_string()
            }
        )
        .is_some(),
        "Expected view.top.files (metadata close button)"
    );
}

#[test]
fn view_metadata_rating_stars_present_with_image_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_metadata = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for r in 1..=5 {
        let id = format!("view.metadata.rating.{}", r);
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.clone()
                }
            )
            .is_some(),
            "Expected {} in metadata panel",
            id
        );
    }
}

#[test]
fn view_metadata_tags_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_metadata = true;
    app.state_mut().image_tags = vec!["favorite".to_string(), "landscape".to_string()];
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for tag in &["favorite", "landscape"] {
        let id = format!("view.metadata.tag.{}", tag);
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.clone()
                }
            )
            .is_some(),
            "Expected {} in metadata panel",
            id
        );
    }
}
