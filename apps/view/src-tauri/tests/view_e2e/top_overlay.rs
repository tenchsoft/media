use super::*;

// ---------------------------------------------------------------------------
// Top overlay buttons (work plan items 1-13)
// ---------------------------------------------------------------------------

#[test]
fn view_renders_nonblank_ui_e2e() {
    let mut harness = view_harness();
    let capture = harness
        .automation_action(UiAutomationAction::Capture {
            request: UiAutomationCaptureRequest {
                include_png: true,
                include_tree: true,
            },
        })
        .expect("capture");

    assert!(
        capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "capture should return PNG bytes"
    );
    assert!(capture.ui_tree.is_some(), "capture should include UI tree");

    let image = image::load_from_memory(&capture.png_bytes)
        .expect("decode PNG")
        .into_rgba8();
    assert_eq!(image.width(), 1280);
    assert_eq!(image.height(), 720);
    assert!(
        is_nonblank(&image, 32),
        "headless render should contain painted pixels"
    );
}

#[test]
fn view_top_overlay_buttons_exist_in_tree_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    // Top overlay buttons present when no image is loaded
    let top_selectors = [
        "view.top.open",
        "view.top.folder",
        "view.top.archive",
        "view.top.info",
        "view.top.edit",
        "view.top.sort_key",
        "view.top.sort_order",
        "view.top.url",
        "view.top.bookmark",
        "view.top.search",
    ];

    for selector in &top_selectors {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: selector.to_string(),
                }
            )
            .is_some(),
            "Expected automation node with debug_id={selector} in tree"
        );
    }

    // These buttons only appear when an image is loaded
    let image_only_selectors = ["view.top.files", "view.top.copy_path", "view.top.copy_img"];
    for selector in &image_only_selectors {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: selector.to_string(),
                }
            )
            .is_none(),
            "Should NOT have debug_id={selector} without image loaded"
        );
    }
}

#[test]
fn view_bottom_overlay_buttons_exist_in_tree_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    let bottom_selectors = [
        "view.bottom.fit",
        "view.bottom.100",
        "view.bottom.zoom_out",
        "view.bottom.zoom_in",
        "view.bottom.filmstrip",
        "view.bottom.rotate",
        "view.bottom.background",
        "view.bottom.fullscreen",
    ];

    for selector in &bottom_selectors {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: selector.to_string(),
                }
            )
            .is_some(),
            "Expected automation node with debug_id={selector} in tree"
        );
    }
}

#[test]
fn view_click_zoom_in_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial_zoom = read_state(&mut harness).zoom;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.zoom_in".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click zoom_in");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_zoom = read_state(&mut harness).zoom;
    assert!(
        new_zoom > initial_zoom,
        "zoom should increase after clicking zoom_in: {} -> {}",
        initial_zoom,
        new_zoom
    );
}

#[test]
fn view_click_zoom_out_changes_state_ui_e2e() {
    let mut harness = view_harness();
    // First zoom in to have room to zoom out
    harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.zoom_in".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("zoom in first");
    let after_in = read_state(&mut harness).zoom;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.zoom_out".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click zoom_out");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let after_out = read_state(&mut harness).zoom;
    assert!(
        after_out < after_in,
        "zoom should decrease after clicking zoom_out: {} -> {}",
        after_in,
        after_out
    );
}

#[test]
fn view_click_rotate_changes_state_ui_e2e() {
    let mut harness = view_harness_with_image();
    let initial_data = read_state(&mut harness).current_image_data.clone();

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.rotate".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click rotate");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_data = read_state(&mut harness).current_image_data;
    assert!(
        initial_data.is_some() && new_data.is_some(),
        "image data should exist before and after rotate"
    );
    // The pixel data should differ after rotation
    assert_ne!(
        initial_data.as_ref().unwrap().data.data(),
        new_data.as_ref().unwrap().data.data(),
        "image pixels should change after rotation"
    );
}

#[test]
fn view_click_fit_changes_state_ui_e2e() {
    let mut harness = view_harness();

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.fit".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click fit");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let state = read_state(&mut harness);
    assert_eq!(state.fit_mode, tench_view_lib::ui::state::FitMode::Fit);
    assert_eq!(state.zoom, 1.0);
}

#[test]
fn view_click_actual_size_changes_state_ui_e2e() {
    let mut harness = view_harness();

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.100".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click 100%");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let state = read_state(&mut harness);
    assert_eq!(state.fit_mode, tench_view_lib::ui::state::FitMode::Actual);
    assert_eq!(state.zoom, 1.0);
}

#[test]
fn view_click_cycle_bg_color_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial_bg = read_state(&mut harness).bg_color;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.background".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click background");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_bg = read_state(&mut harness).bg_color;
    assert_ne!(initial_bg, new_bg, "bg_color should cycle after clicking");
}

#[test]
fn view_click_toggle_filmstrip_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial = read_state(&mut harness).show_thumbnails;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.bottom.filmstrip".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click filmstrip");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_val = read_state(&mut harness).show_thumbnails;
    assert_ne!(initial, new_val, "show_thumbnails should toggle");
}

#[test]
fn view_click_sort_key_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial = read_state(&mut harness).sort_key;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.top.sort_key".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click sort_key");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_val = read_state(&mut harness).sort_key;
    assert_ne!(initial, new_val, "sort_key should cycle after clicking");
}

#[test]
fn view_click_sort_order_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial = read_state(&mut harness).sort_order;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.top.sort_order".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click sort_order");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_val = read_state(&mut harness).sort_order;
    assert_ne!(initial, new_val, "sort_order should toggle after clicking");
}

#[test]
fn view_click_toggle_quick_edit_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial = read_state(&mut harness).show_quick_edit;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.top.edit".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click edit");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_val = read_state(&mut harness).show_quick_edit;
    assert_ne!(initial, new_val, "show_quick_edit should toggle");
}

#[test]
fn view_click_toggle_file_info_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial = read_state(&mut harness).show_file_info;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.top.info".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click info");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_val = read_state(&mut harness).show_file_info;
    assert_ne!(initial, new_val, "show_file_info should toggle");
}

// NOTE: view_click_toggle_metadata requires an image loaded.
// That test will be added once we have a test utility to load images in harness.

#[test]
fn view_click_toggle_search_changes_state_ui_e2e() {
    let mut harness = view_harness();
    let initial = read_state(&mut harness).show_search;

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.top.search".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click search");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_val = read_state(&mut harness).show_search;
    assert_ne!(initial, new_val, "show_search should toggle");
}

#[test]
fn view_click_toggle_bookmark_changes_state_ui_e2e() {
    let mut harness = view_harness_with_image();
    let initial_count = read_state(&mut harness).folder_bookmarks.len();

    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.top.bookmark".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click bookmark");

    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let new_count = read_state(&mut harness).folder_bookmarks.len();
    assert_ne!(
        initial_count, new_count,
        "folder_bookmarks should change after clicking"
    );
}
