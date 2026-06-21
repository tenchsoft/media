//! Plan-backed View E2E tests using the Rust-native UI harness.

use tench_ui::core::events::WindowEvent;
use tench_ui::prelude::Color;
use tench_ui_automation_core::{
    UiAutomationAction, UiAutomationCapture, UiAutomationKey, UiAutomationModifiers,
    UiAutomationPoint, UiAutomationSelector,
};
use tench_ui_test::{harness::HarnessConfig, CaptureAssertions, TestHarness};
use tench_view_lib::ui::{
    state::{BgColor, CompareMode, EditTool, FitMode, SlideshowTransition, ViewState},
    ViewApp,
};

fn selector(debug_id: &str) -> UiAutomationSelector {
    UiAutomationSelector::debug_id(debug_id)
}

fn harness_empty_with_recent() -> TestHarness {
    let mut app = ViewApp::new();
    app.inject_test_recent_files(5);
    TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0))
}

fn harness_with_image() -> TestHarness {
    let mut app = ViewApp::new();
    app.inject_test_image(240, 180);
    app.inject_test_recent_files(5);
    TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0))
}

fn state(harness: &mut TestHarness) -> ViewState {
    let pod = harness.root_mut();
    let app: &mut ViewApp = pod.widget.downcast_mut().expect("root is ViewApp");
    app.state_mut().clone()
}

fn capture(harness: &mut TestHarness) -> UiAutomationCapture {
    harness.automation_capture(Default::default())
}

fn click(harness: &mut TestHarness, debug_id: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::Click {
            selector: selector(debug_id),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|error| panic!("click {debug_id}: {error:?}"))
}

fn click_point(harness: &mut TestHarness, x: f64, y: f64) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::point(x, y),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|error| panic!("click point {x},{y}: {error:?}"))
}

fn right_click_point(harness: &mut TestHarness, x: f64, y: f64) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::RightClick {
            selector: UiAutomationSelector::point(x, y),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|error| panic!("right click point {x},{y}: {error:?}"))
}

fn key(harness: &mut TestHarness, key: UiAutomationKey) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::KeyPress {
            key,
            modifiers: UiAutomationModifiers::default(),
        })
        .expect("key press")
}

fn drag_points(
    harness: &mut TestHarness,
    start: UiAutomationPoint,
    end: UiAutomationPoint,
) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::Drag {
            start,
            end,
            steps: 6,
        })
        .expect("drag points")
}

fn drag_selector_fraction(
    harness: &mut TestHarness,
    debug_id: &str,
    start_fx: f64,
    end_fx: f64,
) -> UiAutomationCapture {
    let bounds = harness
        .automation_bounds(&selector(debug_id))
        .unwrap_or_else(|error| panic!("bounds for {debug_id}: {error:?}"));
    let y = bounds.y + bounds.height / 2.0;
    drag_points(
        harness,
        UiAutomationPoint {
            x: bounds.x + bounds.width * start_fx,
            y,
        },
        UiAutomationPoint {
            x: bounds.x + bounds.width * end_fx,
            y,
        },
    )
}

fn assert_present(capture: &UiAutomationCapture, debug_ids: &[&str]) {
    for debug_id in debug_ids {
        capture.assert_selector_present(&selector(debug_id));
    }
}

fn show_chrome(harness: &mut TestHarness) -> UiAutomationCapture {
    click_point(harness, 640.0, 360.0)
}

#[test]
fn view_plan_empty_state_recent_slots_and_automatic_nodes_use_real_events_ui_e2e() {
    let mut harness = harness_empty_with_recent();
    let initial = capture(&mut harness);
    initial.assert_png_size(1280, 720);
    initial.assert_nonblank();
    assert_present(
        &initial,
        &[
            "view.top.open",
            "view.top.folder",
            "view.top.archive",
            "view.top.url",
            "view.top.info",
            "view.top.edit",
            "view.top.sort_key",
            "view.top.sort_order",
            "view.top.bookmark",
            "view.top.search",
            "view.empty.open_image",
            "view.empty.open_folder",
            "view.empty.open_archive",
            "view.empty.search",
            "view.empty.recent.0",
            "view.empty.recent.1",
            "view.empty.recent.2",
            "view.empty.recent.3",
            "view.empty.recent.4",
            "view.overlay.open_file",
            "view.overlay.open_folder",
            "view.overlay.recent.0",
            "view.overlay.recent.1",
            "view.overlay.recent.2",
            "view.overlay.recent.3",
            "view.overlay.recent.4",
            "view.automatic.loading_overlay",
            "view.automatic.slideshow_auto_advance",
            "view.automatic.slideshow_transition",
            "view.automatic.adjacent_prefetch",
            "view.automatic.batch_progress",
            "view.automatic.pixel_hover_info",
            "view.automatic.drag_and_drop_open",
            "view.automatic.image_placeholder",
            "view.automatic.annotations_overlay",
            "view.automatic.status_message_lifecycle",
            "view.automatic.chrome_visibility",
            "view.automatic.thumbnail_generation",
            "view.automatic.thumbnail_virtual_window",
            "view.automatic.image_layout",
        ],
    );

    click(&mut harness, "view.empty.search");
    assert!(state(&mut harness).show_search);

    // Plan 02: Open folder dialog injection (must be before file dialog, which loads image)
    {
        let pod = harness.root_mut();
        let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
        app.inject_next_folder("/test/images/".to_string());
    }
    click(&mut harness, "view.overlay.open_folder");

    // Plan 03: Open archive dialog injection
    {
        let pod = harness.root_mut();
        let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
        app.inject_next_file("/test/archive.zip".to_string());
    }
    click(&mut harness, "view.empty.open_archive");

    // Plan 01: Open file dialog injection (loads image, exits empty state)
    {
        let pod = harness.root_mut();
        let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
        app.inject_next_file("/test/sample.png".to_string());
    }
    click(&mut harness, "view.overlay.open_file");
    let s = state(&mut harness);
    assert!(s.document.is_some());
    assert_eq!(s.document.as_ref().unwrap().path, "/test/sample.png");

    let mut recent_harness = harness_empty_with_recent();
    capture(&mut recent_harness);
    click(&mut recent_harness, "view.empty.recent.0");
    assert_eq!(
        state(&mut recent_harness)
            .document
            .as_ref()
            .map(|doc| doc.path.as_str()),
        Some("/test/recent_1.png")
    );
}

#[test]
fn view_plan_image_chrome_context_dialogs_metadata_filter_and_print_use_real_events_ui_e2e() {
    let mut harness = harness_with_image();
    show_chrome(&mut harness);
    let initial = capture(&mut harness);
    assert_present(
        &initial,
        &[
            "view.top.files",
            "view.top.copy_path",
            "view.top.copy_img",
            "view.nav.prev",
            "view.nav.next",
            "view.bottom.fit",
            "view.bottom.100",
            "view.bottom.zoom_out",
            "view.bottom.current_zoom",
            "view.bottom.zoom_in",
            "view.bottom.filmstrip",
            "view.bottom.rotate",
            "view.bottom.background",
            "view.bottom.fullscreen",
            "view.batch.trigger",
        ],
    );

    click(&mut harness, "view.bottom.zoom_in");
    assert!(state(&mut harness).zoom > 1.0);
    click(&mut harness, "view.bottom.current_zoom");
    assert_eq!(state(&mut harness).fit_mode, FitMode::Fit);
    click(&mut harness, "view.bottom.100");
    assert_eq!(state(&mut harness).fit_mode, FitMode::Actual);

    click(&mut harness, "view.top.sort_key");
    click(&mut harness, "view.top.sort_order");
    click(&mut harness, "view.bottom.background");
    assert_ne!(state(&mut harness).bg_color, BgColor::Black);
    click(&mut harness, "view.bottom.filmstrip");
    assert!(!state(&mut harness).show_thumbnails);
    click(&mut harness, "view.top.bookmark");
    assert_eq!(
        state(&mut harness).folder_bookmarks,
        vec!["/test".to_string()]
    );

    click(&mut harness, "view.top.info");
    assert_present(&capture(&mut harness), &["view.file_info.close"]);
    click(&mut harness, "view.file_info.close");

    click(&mut harness, "view.top.files");
    let metadata = capture(&mut harness);
    assert_present(
        &metadata,
        &[
            "view.metadata.close",
            "view.metadata.rating.1",
            "view.metadata.rating.2",
            "view.metadata.rating.3",
            "view.metadata.rating.4",
            "view.metadata.rating.5",
            "view.metadata.tag.favorite",
            "view.metadata.tag.landscape",
            "view.metadata.tag.portrait",
            "view.metadata.tag.nature",
            "view.metadata.tag.urban",
            "view.metadata.tag.art",
        ],
    );
    for rating in 1..=5 {
        click(&mut harness, &format!("view.metadata.rating.{rating}"));
    }
    for tag in [
        "favorite",
        "landscape",
        "portrait",
        "nature",
        "urban",
        "art",
    ] {
        click(&mut harness, &format!("view.metadata.tag.{tag}"));
    }
    assert_eq!(state(&mut harness).image_rating, 5);
    assert_eq!(state(&mut harness).image_tags.len(), 6);
    click(&mut harness, "view.metadata.close");

    click(&mut harness, "view.top.url");
    assert_present(
        &capture(&mut harness),
        &["view.url.load", "view.url.cancel"],
    );
    // Plan 09: URL load - verify dialog state
    assert!(state(&mut harness).show_url_dialog);
    click(&mut harness, "view.url.cancel");

    let menu = right_click_point(&mut harness, 640.0, 360.0);
    assert_present(
        &menu,
        &[
            "view.ctx.open_image",
            "view.ctx.open_folder",
            "view.ctx.filters",
            "view.ctx.metadata",
            "view.ctx.show_in_files",
            "view.ctx.copy_path",
            "view.ctx.copy_image",
            "view.ctx.rename",
            "view.ctx.print",
            "view.ctx.delete",
            "view.ctx.dismiss",
        ],
    );
    click(&mut harness, "view.ctx.dismiss");
    assert!(!state(&mut harness).show_context_menu);

    right_click_point(&mut harness, 640.0, 360.0);
    click(&mut harness, "view.ctx.filters");
    let filter = capture(&mut harness);
    assert_present(
        &filter,
        &[
            "view.filter.close",
            "view.filter.brightness",
            "view.filter.contrast",
            "view.filter.saturation",
            "view.filter.blur",
            "view.filter.hue_rotate",
            "view.filter.reset",
            "view.filter.apply",
        ],
    );
    drag_selector_fraction(&mut harness, "view.filter.brightness", 0.2, 0.9);
    assert!(state(&mut harness).filter_dirty);
    click(&mut harness, "view.filter.reset");
    assert_eq!(state(&mut harness).filter_brightness, 100.0);
    click(&mut harness, "view.filter.close");

    right_click_point(&mut harness, 640.0, 360.0);
    click(&mut harness, "view.ctx.print");
    let print = capture(&mut harness);
    assert_present(
        &print,
        &[
            "view.print.paper.a4",
            "view.print.paper.a3",
            "view.print.paper.letter",
            "view.print.paper.legal",
            "view.print.paper.4x6",
            "view.print.paper.5x7",
            "view.print.orientation.portrait",
            "view.print.orientation.landscape",
            "view.print.scaling.fit_to_page",
            "view.print.scaling.fill_page",
            "view.print.scaling.actual_size",
            "view.print.scaling.50_percent",
            "view.print.scaling.25_percent",
            "view.print.print",
            "view.print.cancel",
        ],
    );
    click(&mut harness, "view.print.paper.legal");
    click(&mut harness, "view.print.orientation.landscape");
    click(&mut harness, "view.print.scaling.50_percent");
    let print_state = state(&mut harness);
    assert_eq!(print_state.print_paper_size, "Legal");
    assert_eq!(print_state.print_orientation, "Landscape");
    assert_eq!(print_state.print_scaling, "50%");
    click(&mut harness, "view.print.cancel");

    right_click_point(&mut harness, 640.0, 360.0);
    click(&mut harness, "view.ctx.rename");
    assert_present(&capture(&mut harness), &["view.rename.cancel"]);
    click(&mut harness, "view.rename.cancel");
    right_click_point(&mut harness, 640.0, 360.0);
    click(&mut harness, "view.ctx.rename");
    key(&mut harness, UiAutomationKey::Character("2".to_string()));
    assert_present(&capture(&mut harness), &["view.rename.confirm"]);
    click(&mut harness, "view.rename.confirm");
    assert!(!state(&mut harness).show_rename);

    key(&mut harness, UiAutomationKey::Delete);
    assert_present(
        &capture(&mut harness),
        &["view.delete.cancel", "view.delete.confirm"],
    );
    click(&mut harness, "view.delete.cancel");
    key(&mut harness, UiAutomationKey::Delete);
    click(&mut harness, "view.delete.confirm");
    assert!(!state(&mut harness).show_delete_confirm);
}

#[test]
fn view_plan_slideshow_batch_quick_edit_tools_ai_and_compare_use_real_events_ui_e2e() {
    let mut harness = harness_with_image();
    show_chrome(&mut harness);

    key(&mut harness, UiAutomationKey::Character("s".to_string()));
    let slideshow = capture(&mut harness);
    assert_present(
        &slideshow,
        &[
            "view.slideshow.toggle",
            "view.slideshow.interval",
            "view.slideshow.shuffle",
            "view.slideshow.transition",
            "view.slideshow.loop",
            "view.dismiss",
        ],
    );
    click(&mut harness, "view.slideshow.interval");
    assert_eq!(state(&mut harness).slideshow_interval_ms, 5000);
    click(&mut harness, "view.slideshow.shuffle");
    assert!(state(&mut harness).slideshow_shuffle);
    click(&mut harness, "view.slideshow.transition");
    assert_eq!(
        state(&mut harness).slideshow_transition,
        SlideshowTransition::Slide
    );
    click(&mut harness, "view.slideshow.loop");
    assert!(!state(&mut harness).slideshow_loop);
    click(&mut harness, "view.slideshow.toggle");
    assert!(!state(&mut harness).slideshow_playing);

    click(&mut harness, "view.batch.trigger");
    let batch = capture(&mut harness);
    assert_present(
        &batch,
        &[
            "view.batch.close",
            "view.batch.mode_resize",
            "view.batch.mode_convert",
            "view.batch.select_all",
            "view.batch.file.0",
            "view.batch.browse_output",
            "view.batch.apply",
        ],
    );
    click(&mut harness, "view.batch.mode_convert");
    assert_present(
        &capture(&mut harness),
        &[
            "view.batch.format.png",
            "view.batch.format.jpg",
            "view.batch.format.webp",
        ],
    );
    click(&mut harness, "view.batch.format.jpg");
    // Plan 05: Batch format state verification
    assert_eq!(state(&mut harness).batch_format, "jpg");
    click(&mut harness, "view.batch.format.webp");
    assert_eq!(state(&mut harness).batch_format, "webp");
    click(&mut harness, "view.batch.select_all");
    assert_eq!(state(&mut harness).batch_selected.len(), 3);
    click(&mut harness, "view.batch.file.0");
    assert_eq!(state(&mut harness).batch_selected.len(), 2);
    // Plan 04: Batch browse output dialog injection
    {
        let pod = harness.root_mut();
        let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
        app.inject_batch_output_path("/test/output/".to_string());
    }
    click(&mut harness, "view.batch.browse_output");
    assert_eq!(state(&mut harness).batch_output_folder, "/test/output/");
    click(&mut harness, "view.batch.apply");
    assert!(state(&mut harness).status_message.contains("Batch convert"));
    click(&mut harness, "view.batch.close");

    click(&mut harness, "view.top.edit");
    let quick = capture(&mut harness);
    assert_present(
        &quick,
        &[
            "view.quick_edit.close",
            "view.ctx.crop",
            "view.ctx.resize",
            "view.bottom.rotate",
            "view.ctx.convert",
            "view.quick_edit.markup",
            "view.top.copy_img",
            "view.ctx.delete",
        ],
    );
    click(&mut harness, "view.quick_edit.markup");
    let markup = capture(&mut harness);
    assert_present(
        &markup,
        &[
            "view.quick_edit.annotation.arrow",
            "view.quick_edit.annotation.rect",
            "view.quick_edit.annotation.circle",
            "view.quick_edit.annotation.text",
            "view.quick_edit.annotation.draw",
            "view.quick_edit.annotation.blur",
            "view.quick_edit.color_swatch",
            "view.quick_edit.clear_annotations",
        ],
    );
    for tool in ["rect", "circle", "text", "draw", "blur", "arrow"] {
        click(&mut harness, &format!("view.quick_edit.annotation.{tool}"));
        // Plan 07: Annotation tool state verification
        let s = state(&mut harness);
        assert_eq!(s.active_annotation_tool.map(|t| t.label()), Some(tool));
    }
    click(&mut harness, "view.quick_edit.color_swatch");
    assert_present(
        &capture(&mut harness),
        &[
            "view.annotation.color.r255.g0.b0",
            "view.annotation.color.r255.g128.b0",
            "view.annotation.color.r255.g255.b0",
            "view.annotation.color.r0.g255.b0",
            "view.annotation.color.r0.g255.b255",
            "view.annotation.color.r0.g128.b255",
            "view.annotation.color.r0.g0.b255",
            "view.annotation.color.r128.g0.b255",
            "view.annotation.color.r255.g0.b255",
            "view.annotation.color.r255.g128.b128",
            "view.annotation.color.r255.g255.b255",
            "view.annotation.color.r192.g192.b192",
            "view.annotation.color.r128.g128.b128",
            "view.annotation.color.r64.g64.b64",
            "view.annotation.color.r0.g0.b0",
            "view.annotation.color.r128.g64.b0",
            "view.annotation.color_picker.close",
        ],
    );
    click(&mut harness, "view.annotation.color.r255.g128.b0");
    // Plan 08: Annotation color state verification
    {
        let s = state(&mut harness);
        assert_eq!(s.annotation_color, Color::rgb8(255, 128, 0));
    }
    click(&mut harness, "view.annotation.color_picker.close");
    click(&mut harness, "view.quick_edit.clear_annotations");
    click(&mut harness, "view.ctx.delete");
    click(&mut harness, "view.delete.cancel");
    click(&mut harness, "view.top.copy_img");
    click(&mut harness, "view.bottom.rotate");
    assert!(state(&mut harness).has_edited_image);
    click(&mut harness, "view.edit.discard");
    assert!(!state(&mut harness).has_edited_image);

    click(&mut harness, "view.ctx.crop");
    assert_eq!(state(&mut harness).active_edit_tool, Some(EditTool::Crop));
    assert_present(
        &capture(&mut harness),
        &["view.crop.cancel", "view.crop.apply"],
    );
    drag_points(
        &mut harness,
        UiAutomationPoint { x: 590.0, y: 330.0 },
        UiAutomationPoint { x: 690.0, y: 410.0 },
    );
    assert!(state(&mut harness).crop_selection.is_some());
    click(&mut harness, "view.crop.cancel");

    click(&mut harness, "view.top.edit");
    click(&mut harness, "view.ctx.crop");
    drag_points(
        &mut harness,
        UiAutomationPoint { x: 590.0, y: 330.0 },
        UiAutomationPoint { x: 690.0, y: 410.0 },
    );
    click(&mut harness, "view.crop.apply");
    assert_eq!(state(&mut harness).active_edit_tool, None);

    click(&mut harness, "view.top.edit");
    click(&mut harness, "view.ctx.resize");
    assert_present(
        &capture(&mut harness),
        &[
            "view.resize.width_minus",
            "view.resize.width_plus",
            "view.resize.height_minus",
            "view.resize.height_plus",
            "view.resize.aspect",
            "view.resize.cancel",
            "view.resize.apply",
        ],
    );
    click(&mut harness, "view.resize.width_plus");
    click(&mut harness, "view.resize.height_minus");
    click(&mut harness, "view.resize.aspect");
    click(&mut harness, "view.resize.cancel");
    click(&mut harness, "view.top.edit");
    click(&mut harness, "view.ctx.resize");
    click(&mut harness, "view.resize.apply");
    assert_eq!(state(&mut harness).active_edit_tool, None);

    click(&mut harness, "view.top.edit");
    click(&mut harness, "view.ctx.convert");
    assert_present(
        &capture(&mut harness),
        &[
            "view.convert.format.png",
            "view.convert.format.jpg",
            "view.convert.format.webp",
            "view.convert.format.bmp",
            "view.convert.format.tiff",
            "view.convert.browse_output",
            "view.convert.cancel",
            "view.convert.apply",
        ],
    );
    for fmt in ["png", "jpg", "webp", "bmp", "tiff"] {
        click(&mut harness, &format!("view.convert.format.{fmt}"));
        // Plan 06: Convert format state verification
        assert_eq!(state(&mut harness).convert_format, fmt);
    }
    // Plan 04: Convert browse output dialog injection
    {
        let pod = harness.root_mut();
        let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
        app.inject_save_as_path("/test/converted/".to_string());
    }
    click(&mut harness, "view.convert.browse_output");
    assert_eq!(
        state(&mut harness).convert_output_path.as_deref(),
        Some("/test/converted/")
    );
    click(&mut harness, "view.convert.cancel");
    click(&mut harness, "view.top.edit");
    click(&mut harness, "view.ctx.convert");
    click(&mut harness, "view.convert.apply");
    assert_eq!(state(&mut harness).active_edit_tool, None);

    key(&mut harness, UiAutomationKey::Character("a".to_string()));
    assert_present(
        &capture(&mut harness),
        &[
            "view.ai.close",
            "view.ai.feature.enhance",
            "view.ai.feature.upscale 2x",
            "view.ai.feature.bg remove",
            "view.ai.feature.smart crop",
            "view.ai.feature.tag",
            "view.ai.feature.describe",
        ],
    );
    click(&mut harness, "view.ai.feature.describe");
    assert_present(&capture(&mut harness), &["view.ai.run"]);
    click(&mut harness, "view.ai.run");
    assert!(state(&mut harness).ai_result_text.is_some());
    click(&mut harness, "view.ai.close");

    key(&mut harness, UiAutomationKey::Character("d".to_string()));
    assert_present(
        &capture(&mut harness),
        &[
            "view.compare.toggle",
            "view.compare.mode",
            "view.compare.drag",
        ],
    );
    let split_before = state(&mut harness).compare_split;
    drag_selector_fraction(&mut harness, "view.compare.drag", 0.5, 0.9);
    assert!(state(&mut harness).compare_split > split_before);
    click(&mut harness, "view.compare.mode");
    assert_eq!(state(&mut harness).compare_mode, CompareMode::SideBySide);
    click(&mut harness, "view.compare.toggle");
}

#[test]
fn view_plan_automatic_runtime_behaviors_use_real_events_ui_e2e() {
    let mut harness = harness_with_image();
    assert!(!state(&mut harness).show_chrome);
    show_chrome(&mut harness);
    assert!(state(&mut harness).show_chrome);

    drag_points(
        &mut harness,
        UiAutomationPoint { x: 640.0, y: 360.0 },
        UiAutomationPoint { x: 642.0, y: 362.0 },
    );
    assert!(state(&mut harness).pixel_info.is_some());
    assert_present(
        &capture(&mut harness),
        &[
            "view.automatic.pixel_hover_info",
            "view.automatic.chrome_visibility",
            "view.automatic.thumbnail_generation",
            "view.automatic.thumbnail_virtual_window",
            "view.automatic.image_layout",
        ],
    );

    let mut slideshow_harness = harness_with_image();
    key(
        &mut slideshow_harness,
        UiAutomationKey::Character("s".to_string()),
    );
    slideshow_harness.dispatch_window(WindowEvent::AnimFrame(0));
    slideshow_harness.dispatch_window(WindowEvent::AnimFrame(3100));
    assert_eq!(
        state(&mut slideshow_harness)
            .document
            .as_ref()
            .map(|doc| doc.path.as_str()),
        Some("/test/img3.png")
    );
    assert_present(
        &capture(&mut slideshow_harness),
        &[
            "view.automatic.slideshow_auto_advance",
            "view.automatic.slideshow_transition",
            "view.automatic.adjacent_prefetch",
        ],
    );
}
