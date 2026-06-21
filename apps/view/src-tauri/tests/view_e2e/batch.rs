use super::*;

// ---------------------------------------------------------------------------
// Batch panel (work plan items 58-68)
// ---------------------------------------------------------------------------

#[test]
fn view_batch_trigger_present_with_multiple_entries_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    // inject_test_image creates 3 entries, so batch trigger should appear
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.batch.trigger".to_string()
            }
        )
        .is_some(),
        "Expected view.batch.trigger with multiple entries"
    );
}

#[test]
fn view_batch_panel_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    let expected = [
        "view.batch.close",
        "view.batch.mode_resize",
        "view.batch.mode_convert",
        "view.batch.select_all",
        "view.batch.browse_output",
        "view.batch.apply",
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
            "Expected {} in batch panel",
            id
        );
    }
}

#[test]
fn view_batch_convert_mode_shows_format_buttons_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    app.state_mut().batch_mode_resize = false; // convert mode
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for fmt in &["png", "jpg", "webp"] {
        let id = format!("view.batch.format.{}", fmt);
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.clone()
                }
            )
            .is_some(),
            "Expected {} in convert mode",
            id
        );
    }
}

#[test]
fn view_batch_file_entries_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    // 3 entries from inject_test_image
    for i in 0..3 {
        let id = format!("view.batch.file.{}", i);
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.clone()
                }
            )
            .is_some(),
            "Expected {} in batch panel",
            id
        );
    }
}

#[test]
fn view_batch_click_close_dismisses_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.batch.close".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click batch close");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn view_batch_click_mode_resize_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.batch.mode_resize".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click batch resize mode");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn view_batch_click_mode_convert_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.batch.mode_convert".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click batch convert mode");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn view_batch_click_select_all_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.batch.select_all".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click select all");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}
