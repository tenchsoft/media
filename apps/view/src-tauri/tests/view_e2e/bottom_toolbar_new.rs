use super::*;

// ---------------------------------------------------------------------------
// Bottom toolbar new items (Phase 4: share, wallpaper, delete)
// ---------------------------------------------------------------------------

#[test]
fn view_bottom_toolbar_share_button_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.share".to_string(),
            }
        )
        .is_some(),
        "Expected view.bottom.share in bottom toolbar"
    );
}

#[test]
fn view_bottom_toolbar_wallpaper_button_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.wallpaper".to_string(),
            }
        )
        .is_some(),
        "Expected view.bottom.wallpaper in bottom toolbar"
    );
}

#[test]
fn view_bottom_toolbar_delete_button_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.delete".to_string(),
            }
        )
        .is_some(),
        "Expected view.bottom.delete in bottom toolbar"
    );
}

#[test]
fn view_bottom_toolbar_delete_click_shows_confirm_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.delete".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click delete");

    let state = read_state(&mut harness);
    assert!(
        state.show_delete_confirm,
        "delete confirm should appear after clicking delete from toolbar"
    );
}
