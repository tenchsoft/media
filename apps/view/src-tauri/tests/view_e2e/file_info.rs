use super::*;

// ---------------------------------------------------------------------------
// File info overlay (work plan item 154)
// ---------------------------------------------------------------------------

#[test]
fn view_file_info_close_button_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_file_info = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.top.info".to_string()
            }
        )
        .is_some(),
        "Expected file info close button"
    );
}
