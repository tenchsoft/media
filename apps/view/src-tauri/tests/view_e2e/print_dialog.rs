use super::*;

// ---------------------------------------------------------------------------
// Print dialog (work plan items 80-92)
// ---------------------------------------------------------------------------

#[test]
fn view_print_dialog_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_print_dialog = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.print.print".to_string()
            }
        )
        .is_some(),
        "Expected view.print.print"
    );
}

#[test]
fn view_print_dialog_not_present_by_default_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.print.print".to_string()
            }
        )
        .is_none(),
        "Should NOT have view.print.print without show_print_dialog"
    );
}
