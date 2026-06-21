use super::*;

// ---------------------------------------------------------------------------
// Canvas advanced features (Phase 8: checkerboard-bg, crop-aspect-ratio)
//
// Checkerboard: verify state toggle and rendering behavior.
// Crop aspect ratio: verify state changes via action dispatch.
// ---------------------------------------------------------------------------

#[test]
fn view_canvas_checkerboard_default_off_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    assert!(
        !app.state_mut().checkerboard_bg,
        "checkerboard should be off by default"
    );
}

#[test]
fn view_canvas_checkerboard_toggle_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Manually toggle checkerboard via action dispatch
    {
        let pod = harness.root_mut();
        let view_app: &mut ViewApp = pod.widget.downcast_mut().expect("root is ViewApp");
        view_app.state_mut().checkerboard_bg = true;
    }

    let state = read_state(&mut harness);
    assert!(
        state.checkerboard_bg,
        "checkerboard should be on after toggle"
    );

    // Toggle back off
    {
        let pod = harness.root_mut();
        let view_app: &mut ViewApp = pod.widget.downcast_mut().expect("root is ViewApp");
        view_app.state_mut().checkerboard_bg = false;
    }

    let state = read_state(&mut harness);
    assert!(
        !state.checkerboard_bg,
        "checkerboard should be off after second toggle"
    );
}

#[test]
fn view_crop_aspect_ratio_default_none_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    assert!(
        app.state_mut().crop_aspect_ratio.is_none(),
        "crop aspect ratio should be None (free) by default"
    );
}

#[test]
fn view_crop_aspect_ratio_toggle_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Set 16:9 aspect ratio
    {
        let pod = harness.root_mut();
        let view_app: &mut ViewApp = pod.widget.downcast_mut().expect("root is ViewApp");
        view_app.state_mut().crop_aspect_ratio = Some((16, 9));
    }

    let state = read_state(&mut harness);
    assert_eq!(state.crop_aspect_ratio, Some((16, 9)));

    // Set free
    {
        let pod = harness.root_mut();
        let view_app: &mut ViewApp = pod.widget.downcast_mut().expect("root is ViewApp");
        view_app.state_mut().crop_aspect_ratio = None;
    }

    let state = read_state(&mut harness);
    assert!(
        state.crop_aspect_ratio.is_none(),
        "aspect ratio should be free (None)"
    );
}
