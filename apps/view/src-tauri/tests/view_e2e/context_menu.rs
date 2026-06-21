use super::*;

// ---------------------------------------------------------------------------
// Context menu (work plan items 41-51)
// ---------------------------------------------------------------------------

#[test]
fn view_context_menu_not_present_by_default_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    // Context menu items should NOT be present by default
    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.open_image".to_string(),
            }
        )
        .is_none(),
        "Context menu should not be present by default"
    );
}

#[test]
fn view_context_menu_items_present_when_shown_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    // Without image, only Open Image and Open Folder
    let expected = ["view.ctx.open_image", "view.ctx.open_folder"];
    for selector in &expected {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: selector.to_string(),
                }
            )
            .is_some(),
            "Expected {} in context menu",
            selector
        );
    }
}

#[test]
fn view_context_menu_full_items_with_image_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    // With image loaded, all context menu items should be present
    let expected = [
        "view.ctx.open_image",
        "view.ctx.open_folder",
        "view.ctx.filters",
        "view.ctx.metadata",
        "view.ctx.show_in_files",
        "view.ctx.copy_path",
        "view.ctx.copy_image",
        "view.ctx.rename",
        "view.ctx.print",
        "view.ctx.delete",
    ];
    for selector in &expected {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: selector.to_string(),
                }
            )
            .is_some(),
            "Expected {} in context menu with image",
            selector
        );
    }
}
