use super::*;

// ---------------------------------------------------------------------------
// Compare panel (work plan items 155-157)
// ---------------------------------------------------------------------------

#[test]
fn view_compare_panel_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_compare = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for id in &[
        "view.compare.toggle",
        "view.compare.mode",
        "view.compare.drag",
    ] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in compare panel",
            id
        );
    }
}

#[test]
fn view_compare_not_present_by_default_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    for id in &["view.compare.toggle", "view.compare.mode"] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_none(),
            "Should NOT have {} without show_compare",
            id
        );
    }
}
