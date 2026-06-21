use super::*;

// ---------------------------------------------------------------------------
// Help overlay (work plan item 158)
// ---------------------------------------------------------------------------

#[test]
fn view_help_overlay_close_button_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().show_help = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.help.close".to_string()
            }
        )
        .is_some(),
        "Expected view.help.close"
    );
}

#[test]
fn view_help_overlay_not_present_by_default_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.help.close".to_string()
            }
        )
        .is_none(),
        "Should NOT have view.help.close without show_help"
    );
}
