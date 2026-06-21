use super::*;

// ---------------------------------------------------------------------------
// Delete confirm dialog (work plan items 69-71)
// ---------------------------------------------------------------------------

#[test]
fn view_delete_confirm_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_delete_confirm = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for id in &["view.delete.cancel", "view.delete.confirm"] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in delete confirm dialog",
            id
        );
    }
}

#[test]
fn view_delete_confirm_not_present_by_default_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    for id in &["view.delete.cancel", "view.delete.confirm"] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_none(),
            "Should NOT have {} without show_delete_confirm",
            id
        );
    }
}

#[test]
fn view_delete_confirm_click_cancel_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_delete_confirm = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.delete.cancel".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click delete cancel");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}
