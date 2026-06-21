use super::*;

// ---------------------------------------------------------------------------
// Print backend (Phase 5: actual printer output integration)
//
// Tests verify print dialog state changes and action dispatch.
// The print button is in the hamburger menu, not the bottom toolbar.
// ---------------------------------------------------------------------------

#[test]
fn view_print_dialog_default_hidden_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    assert!(
        !app.state_mut().show_print_dialog,
        "print dialog should be hidden by default"
    );
}

#[test]
fn view_print_dialog_shows_via_hamburger_menu_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    // Open hamburger menu first
    app.state_mut().show_menu = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click print in hamburger menu
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.print.print".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click print button");

    let state = read_state(&mut harness);
    assert!(
        state.show_print_dialog,
        "print dialog should be visible after clicking print button"
    );
}

#[test]
fn view_print_dialog_closes_on_cancel_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_print_dialog = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click cancel in print dialog
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.print.cancel".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click print cancel");

    let state = read_state(&mut harness);
    assert!(
        !state.show_print_dialog,
        "print dialog should close after cancel"
    );
}
