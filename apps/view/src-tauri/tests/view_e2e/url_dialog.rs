use super::*;

// ---------------------------------------------------------------------------
// URL dialog (work plan items 77-79)
// ---------------------------------------------------------------------------

#[test]
fn view_url_dialog_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().show_url_dialog = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.url.load".to_string()
            }
        )
        .is_some(),
        "Expected view.url.load"
    );
}

#[test]
fn view_url_dialog_not_present_by_default_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.url.load".to_string()
            }
        )
        .is_none(),
        "Should NOT have view.url.load without show_url_dialog"
    );
}
