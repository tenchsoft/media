use super::*;

// ---------------------------------------------------------------------------
// Quick edit overlay (work plan items 138-153)
// ---------------------------------------------------------------------------

#[test]
fn view_quick_edit_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_quick_edit = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    // Close button is mapped via ToggleQuickEdit
    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.top.edit".to_string()
            }
        )
        .is_some(),
        "Expected quick edit close button"
    );
}

#[test]
fn view_quick_edit_annotation_tools_present_ui_e2e() {
    use tench_view_lib::ui::state::AnnotationTool;
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Arrow);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    let tools = [
        "view.quick_edit.annotation.arrow",
        "view.quick_edit.annotation.rect",
        "view.quick_edit.annotation.circle",
        "view.quick_edit.annotation.text",
        "view.quick_edit.annotation.draw",
        "view.quick_edit.annotation.blur",
        "view.quick_edit.clear_annotations",
    ];
    for id in &tools {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in quick edit",
            id
        );
    }
}
