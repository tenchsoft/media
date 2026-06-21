use super::*;

// ---------------------------------------------------------------------------
// Crop tool (work plan items 159-161)
// ---------------------------------------------------------------------------

#[test]
fn view_crop_tool_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().active_edit_tool = Some(tench_view_lib::ui::state::EditTool::Crop);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for id in &["view.crop.cancel", "view.crop.apply"] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in crop tool",
            id
        );
    }
}

#[test]
fn view_crop_tool_click_cancel_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().active_edit_tool = Some(tench_view_lib::ui::state::EditTool::Crop);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.crop.cancel".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click crop cancel");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}
