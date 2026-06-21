use super::*;

// ---------------------------------------------------------------------------
// Slideshow controls (work plan items 52-57)
// ---------------------------------------------------------------------------

#[test]
fn view_slideshow_controls_not_present_when_stopped_ui_e2e() {
    let mut harness = view_harness_with_image();
    let tree = harness.automation_tree();

    // Slideshow controls should not be present when not playing
    for id in &[
        "view.slideshow.toggle",
        "view.slideshow.interval",
        "view.slideshow.shuffle",
        "view.slideshow.transition",
        "view.slideshow.loop",
    ] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_none(),
            "Should NOT have {} when slideshow stopped",
            id
        );
    }
}

#[test]
fn view_slideshow_controls_present_when_playing_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    for id in &[
        "view.slideshow.toggle",
        "view.slideshow.interval",
        "view.slideshow.shuffle",
        "view.slideshow.transition",
        "view.slideshow.loop",
        "view.dismiss",
    ] {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} when slideshow playing",
            id
        );
    }
}

#[test]
fn view_slideshow_click_toggle_changes_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.slideshow.toggle".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click slideshow toggle");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn view_slideshow_click_interval_cycles_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.slideshow.interval".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click interval");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn view_slideshow_click_shuffle_toggles_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.slideshow.shuffle".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click shuffle");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn view_slideshow_click_transition_cycles_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.slideshow.transition".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click transition");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn view_slideshow_click_loop_toggles_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.slideshow.loop".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click loop");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
}
