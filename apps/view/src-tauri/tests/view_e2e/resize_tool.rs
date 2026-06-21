use super::*;

// ---------------------------------------------------------------------------
// Resize tool (work plan items 162-168)
// ---------------------------------------------------------------------------

#[test]
fn view_resize_tool_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().active_edit_tool = Some(tench_view_lib::ui::state::EditTool::Resize);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    let expected = [
        "view.resize.width_minus",
        "view.resize.width_plus",
        "view.resize.height_minus",
        "view.resize.height_plus",
        "view.resize.aspect",
        "view.resize.cancel",
        "view.resize.apply",
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
            "Expected {} in resize tool",
            id
        );
    }
}

#[test]
fn view_resize_tool_click_width_plus_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().active_edit_tool = Some(tench_view_lib::ui::state::EditTool::Resize);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.resize.width_plus".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click width plus");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}
