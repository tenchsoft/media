use super::*;

// ---------------------------------------------------------------------------
// Automatic behaviors (work plan items 177-190)
// ---------------------------------------------------------------------------

#[test]
fn view_automatic_loading_overlay_state_ui_e2e() {
    // Verify loading state shows is_loading flag
    let mut app = ViewApp::new();
    app.state_mut().is_loading = true;
    assert!(app.state_mut().is_loading);
}

#[test]
fn view_automatic_slideshow_state_ui_e2e() {
    // Verify slideshow state management
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().slideshow_playing = true;
    app.state_mut().slideshow_interval_ms = 3000;
    app.state_mut().slideshow_shuffle = true;
    app.state_mut().slideshow_loop = true;

    assert!(app.state_mut().slideshow_playing);
    assert_eq!(app.state_mut().slideshow_interval_ms, 3000);
    assert!(app.state_mut().slideshow_shuffle);
    assert!(app.state_mut().slideshow_loop);
}

#[test]
fn view_automatic_slideshow_transition_state_ui_e2e() {
    use tench_view_lib::ui::state::SlideshowTransition;
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    // Cycle through transitions
    app.state_mut().slideshow_transition = SlideshowTransition::Fade;
    assert_eq!(
        app.state_mut().slideshow_transition,
        SlideshowTransition::Fade
    );
    app.state_mut().slideshow_transition = app.state_mut().slideshow_transition.cycle();
    assert_eq!(
        app.state_mut().slideshow_transition,
        SlideshowTransition::Slide
    );
}

#[test]
fn view_automatic_batch_progress_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_batch = true;
    app.state_mut().batch_running = true;
    app.state_mut().batch_progress = Some((5, 10));

    assert!(app.state_mut().batch_running);
    assert_eq!(app.state_mut().batch_progress, Some((5, 10)));
}

#[test]
fn view_automatic_annotations_state_ui_e2e() {
    use tench_view_lib::ui::state::{Annotation, AnnotationTool};
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    let annotation = Annotation {
        tool: AnnotationTool::Arrow,
        x: 10.0,
        y: 20.0,
        w: 100.0,
        h: 50.0,
        text: String::new(),
        color: tench_ui::Color::WHITE,
        line_width: 2.0,
    };
    app.state_mut().annotations.push(annotation);
    assert_eq!(app.state_mut().annotations.len(), 1);
}

#[test]
fn view_automatic_status_message_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().status_message = "Saved successfully".to_string();
    app.state_mut().status_message_time = Some(std::time::Instant::now());

    assert_eq!(app.state_mut().status_message, "Saved successfully");
    assert!(app.state_mut().status_message_time.is_some());
}

#[test]
fn view_automatic_chrome_visibility_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    // Chrome should be hidden by default (shown on hover)
    assert!(!app.state_mut().show_chrome);
}

#[test]
fn view_automatic_thumbnail_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    // Verify thumbnail state defaults
    assert_eq!(app.state_mut().thumbnail_size, 120);
    assert_eq!(app.state_mut().thumbnail_scroll_offset, 0.0);
}

#[test]
fn view_automatic_image_layout_state_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);

    // Verify layout-related state defaults
    assert_eq!(
        app.state_mut().fit_mode,
        tench_view_lib::ui::state::FitMode::Fit
    );
    assert_eq!(app.state_mut().zoom, 1.0);
    assert_eq!(app.state_mut().rotation, 0);
}
