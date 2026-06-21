use super::*;

// ---------------------------------------------------------------------------
// Edit banner (work plan items 72-73)
// ---------------------------------------------------------------------------

#[test]
fn view_edit_banner_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().has_edited_image = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for id in &["view.edit.save", "view.edit.discard"] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in edit banner",
            id
        );
    }
}

#[test]
fn view_edit_banner_not_present_without_edit_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    for id in &["view.edit.save", "view.edit.discard"] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_none(),
            "Should NOT have {} without edited image",
            id
        );
    }
}
