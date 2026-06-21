use super::*;

// ---------------------------------------------------------------------------
// Convert tool (work plan items 169-176)
// ---------------------------------------------------------------------------

#[test]
fn view_convert_tool_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().active_edit_tool = Some(tench_view_lib::ui::state::EditTool::Convert);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    let expected = [
        "view.convert.format.png",
        "view.convert.format.jpg",
        "view.convert.format.webp",
        "view.convert.format.bmp",
        "view.convert.format.tiff",
        "view.convert.browse_output",
        "view.convert.cancel",
        "view.convert.apply",
    ];
    for id in &expected {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in convert tool",
            id
        );
    }
}

#[test]
fn view_convert_tool_click_cancel_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().active_edit_tool = Some(tench_view_lib::ui::state::EditTool::Convert);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.convert.cancel".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click convert cancel");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}
