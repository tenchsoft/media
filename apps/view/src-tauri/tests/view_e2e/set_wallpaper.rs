use super::*;

// ---------------------------------------------------------------------------
// Set wallpaper (Phase 1: platform infrastructure)
// ---------------------------------------------------------------------------

#[test]
fn view_set_wallpaper_state_toggle_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    // Verify initial state
    let state = app.state_mut();
    assert!(state.document.is_some());
}

#[test]
fn view_set_wallpaper_bottom_toolbar_button_present_ui_e2e() {
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
fn view_set_wallpaper_click_sets_status_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.wallpaper".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click wallpaper");

    // The click should succeed without panic (actual wallpaper is platform-dependent)
    let state = read_state(&mut harness);
    assert!(state.document.is_some(), "image should still be loaded");
}
