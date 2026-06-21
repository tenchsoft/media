use super::*;

// ---------------------------------------------------------------------------
// Slideshow advanced (Phase 6: transition effects, loop)
// ---------------------------------------------------------------------------

#[test]
fn view_slideshow_transition_cycles_all_effects_ui_e2e() {
    use tench_view_lib::ui::state::SlideshowTransition;
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    let transitions = [
        SlideshowTransition::Fade,
        SlideshowTransition::Slide,
        SlideshowTransition::Dissolve,
        SlideshowTransition::None,
    ];

    let mut current = app.state_mut().slideshow_transition;
    for expected in &transitions {
        assert_eq!(current, *expected);
        current = current.cycle();
    }
    // After cycling all, should wrap back to Fade
    assert_eq!(current, SlideshowTransition::Fade);
}

#[test]
fn view_slideshow_loop_toggle_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    // Default is loop on
    assert!(app.state_mut().slideshow_loop);

    app.state_mut().slideshow_loop = false;
    assert!(!app.state_mut().slideshow_loop);

    app.state_mut().slideshow_loop = true;
    assert!(app.state_mut().slideshow_loop);
}

#[test]
fn view_slideshow_transition_fade_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    app.state_mut().slideshow_fade_alpha = 0.5;
    app.state_mut().slideshow_prev_image = app.state_mut().current_image_data.clone();

    assert_eq!(app.state_mut().slideshow_fade_alpha, 0.5);
    assert!(app.state_mut().slideshow_prev_image.is_some());
}

#[test]
fn view_slideshow_click_loop_toggles_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let initial_loop = read_state(&mut harness).slideshow_loop;

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.slideshow.loop".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click loop");

    let new_loop = read_state(&mut harness).slideshow_loop;
    assert_ne!(initial_loop, new_loop, "slideshow loop should toggle");
}

#[test]
fn view_slideshow_click_transition_cycles_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let initial = read_state(&mut harness).slideshow_transition;

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.slideshow.transition".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click transition");

    let new_transition = read_state(&mut harness).slideshow_transition;
    assert_ne!(initial, new_transition, "transition should cycle");
    assert_eq!(new_transition, initial.cycle());
}
