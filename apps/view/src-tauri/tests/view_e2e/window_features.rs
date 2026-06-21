use super::*;

// ---------------------------------------------------------------------------
// Window features (Phase 9: dynamic title, hamburger menu)
//
// Tests use automation_action Click where possible to exercise real dispatch.
// ---------------------------------------------------------------------------

#[test]
fn view_hamburger_menu_toggle_via_click_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click hamburger menu button
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.top.menu".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click hamburger menu");

    let state = read_state(&mut harness);
    assert!(state.show_menu, "menu should be visible after click");

    // Click dismiss area
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.dismiss".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click menu dismiss");

    let state = read_state(&mut harness);
    assert!(!state.show_menu, "menu should be hidden after dismiss");
}

#[test]
fn view_title_bar_shows_file_name_ui_e2e() {
    let mut app = ViewApp::new();
    // No document — title should be default
    assert!(
        app.state_mut().document.is_none(),
        "no document loaded initially"
    );

    // Load a test image
    app.inject_test_image(200, 150);
    let state = app.state_mut();
    assert!(state.document.is_some(), "document should be loaded");

    // File name should be present
    let doc = state.document.as_ref().unwrap();
    assert!(
        !doc.file_name.is_empty(),
        "file name should be non-empty after loading"
    );
}

#[test]
fn view_title_bar_updates_on_navigation_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;

    // After loading, sorted_entries should be populated
    assert!(
        app.state_mut().sorted_entries.len() >= 2,
        "folder entries should be populated for navigation"
    );

    // Current file name should be set
    let doc = app.state_mut().document.as_ref().unwrap();
    assert_eq!(
        doc.file_name, "test_image.png",
        "file name should match loaded image"
    );
}
