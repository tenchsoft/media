use super::*;

// ---------------------------------------------------------------------------
// Filter panel (work plan items 122-129)
// ---------------------------------------------------------------------------

#[test]
fn view_filter_panel_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_filter = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for id in &[
        "view.filter.close",
        "view.filter.reset",
        "view.filter.apply",
    ] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in filter panel",
            id
        );
    }
}

#[test]
fn view_filter_panel_not_present_by_default_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    for id in &[
        "view.filter.close",
        "view.filter.reset",
        "view.filter.apply",
    ] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_none(),
            "Should NOT have {} without show_filter",
            id
        );
    }
}
