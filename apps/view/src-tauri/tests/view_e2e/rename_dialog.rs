use super::*;

// ---------------------------------------------------------------------------
// Rename dialog (work plan items 74-76)
// ---------------------------------------------------------------------------

#[test]
fn view_rename_dialog_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_rename = true;
    app.state_mut().rename_input_text = "new_name.png".to_string();
    app.state_mut().rename_original_name = "old_name.png".to_string();
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.rename.cancel".to_string()
            }
        )
        .is_some(),
        "Expected view.rename.cancel"
    );
    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.rename.confirm".to_string()
            }
        )
        .is_some(),
        "Expected view.rename.confirm when input is valid"
    );
}

#[test]
fn view_rename_confirm_disabled_when_input_empty_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_rename = true;
    app.state_mut().rename_input_text = String::new();
    app.state_mut().rename_original_name = "old_name.png".to_string();
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.rename.confirm".to_string()
            }
        )
        .is_none(),
        "Should NOT have view.rename.confirm when input is empty"
    );
}

#[test]
fn view_rename_confirm_disabled_when_same_name_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_rename = true;
    app.state_mut().rename_input_text = "same.png".to_string();
    app.state_mut().rename_original_name = "same.png".to_string();
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.rename.confirm".to_string()
            }
        )
        .is_none(),
        "Should NOT have view.rename.confirm when name unchanged"
    );
}
