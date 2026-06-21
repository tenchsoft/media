use super::*;
use tench_ui_automation_core::UiAutomationKey;

// ---------------------------------------------------------------------------
// Keyboard shortcut F2 (Phase 11: rename)
//
// Tests verify F2 triggers rename mode.
// ---------------------------------------------------------------------------

#[test]
fn view_keyboard_f2_triggers_rename_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Simulate F2 key press
    let _capture = harness
        .automation_action(UiAutomationAction::KeyPress {
            key: UiAutomationKey::F2,
            modifiers: Default::default(),
        })
        .expect("press F2 key");

    let state = read_state(&mut harness);
    assert!(state.show_rename, "F2 should open rename dialog");
}

#[test]
fn view_keyboard_f2_no_op_without_document_ui_e2e() {
    let mut app = ViewApp::new();
    // No document loaded
    assert!(app.state_mut().document.is_none());
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Simulate F2 key press without document
    let _capture = harness
        .automation_action(UiAutomationAction::KeyPress {
            key: UiAutomationKey::F2,
            modifiers: Default::default(),
        })
        .expect("press F2 key");

    let state = read_state(&mut harness);
    assert!(
        !state.show_rename,
        "F2 should not open rename dialog without a document"
    );
}
