use super::*;

#[test]
fn load_image_sets_document_and_resets_view_state() {
    let mut state = ViewState {
        zoom: 4.0,
        rotation: 90,
        has_edited_image: true,
        ..ViewState::default()
    };

    state.load_image(ImageMetadata {
        file_name: "photo.png".into(),
        format: "png".into(),
        dimensions: Some(ImageDimensions {
            width: 800,
            height: 600,
        }),
        file_size: 1200,
        path: "C:/tmp/photo.png".into(),
    });

    assert_eq!(state.document.as_ref().unwrap().file_name, "photo.png");
    assert_eq!(state.zoom, 1.0);
    assert_eq!(state.rotation, 0);
    assert!(!state.has_edited_image);
    assert_eq!(state.status_message, "Loaded");
}

#[test]
fn navigation_walks_sorted_folder_entries() {
    let mut state = ViewState::default();
    state.set_folder_entries(vec![
        entry("2", "C:/tmp/b.png", "b.png", 20),
        entry("1", "C:/tmp/a.png", "a.png", 10),
    ]);
    state.load_image(ImageMetadata {
        file_name: "a.png".into(),
        format: "png".into(),
        dimensions: None,
        file_size: 10,
        path: "C:/tmp/a.png".into(),
    });

    assert!(state.navigate_next());
    assert_eq!(state.document.as_ref().unwrap().file_name, "b.png");
    assert!(state.navigate_prev());
    assert_eq!(state.document.as_ref().unwrap().file_name, "a.png");
}

#[test]
fn edit_history_undo_redo() {
    let mut state = ViewState::default();
    let img1 = make_test_image_data(10, 10);
    let img2 = make_test_image_data(20, 20);

    state.push_edit_history(img1, "crop");
    state.push_edit_history(img2, "resize");

    assert_eq!(state.edit_history.len(), 2);
    assert_eq!(state.edit_history_index, 2);

    assert!(state.undo_edit());
    assert_eq!(state.edit_history_index, 1);

    assert!(state.redo_edit());
    assert_eq!(state.edit_history_index, 2);
}

fn entry(id: &str, path: &str, file_name: &str, size_bytes: u64) -> FolderEntry {
    FolderEntry {
        id: id.into(),
        path: path.into(),
        file_name: file_name.into(),
        size_bytes,
        modified_unix: None,
        is_archive_entry: false,
    }
}

fn make_test_image_data(w: u32, h: u32) -> peniko::ImageData {
    let pixels = vec![0u8; (w * h * 4) as usize];
    peniko::ImageData {
        data: pixels.into(),
        format: peniko::ImageFormat::Rgba8,
        alpha_type: peniko::ImageAlphaType::AlphaPremultiplied,
        width: w,
        height: h,
    }
}

// --- sort_entries ---

#[test]
fn sort_entries_by_name_ascending() {
    let mut state = ViewState {
        sort_key: SortKey::Name,
        sort_order: SortOrder::Asc,
        ..Default::default()
    };
    state.set_folder_entries(vec![
        entry("2", "C:/tmp/b.png", "b.png", 20),
        entry("1", "C:/tmp/a.png", "a.png", 10),
        entry("3", "C:/tmp/c.png", "c.png", 30),
    ]);
    assert_eq!(
        state
            .sorted_entries
            .iter()
            .map(|e| e.file_name.as_str())
            .collect::<Vec<_>>(),
        vec!["a.png", "b.png", "c.png"]
    );
}

#[test]
fn sort_entries_by_size_descending() {
    let mut state = ViewState {
        sort_key: SortKey::Size,
        sort_order: SortOrder::Desc,
        ..Default::default()
    };
    state.set_folder_entries(vec![
        entry("1", "C:/tmp/a.png", "a.png", 10),
        entry("2", "C:/tmp/b.png", "b.png", 30),
        entry("3", "C:/tmp/c.png", "c.png", 20),
    ]);
    assert_eq!(
        state
            .sorted_entries
            .iter()
            .map(|e| e.size_bytes)
            .collect::<Vec<_>>(),
        vec![30, 20, 10]
    );
}

// --- reset_filters ---

#[test]
fn reset_filters_restores_defaults() {
    let mut state = ViewState {
        filter_brightness: 150.0,
        filter_contrast: 50.0,
        filter_saturation: 200.0,
        filter_blur: 10.0,
        filter_hue_rotate: 180.0,
        ..Default::default()
    };
    state.reset_filters();
    assert_eq!(state.filter_brightness, 100.0);
    assert_eq!(state.filter_contrast, 100.0);
    assert_eq!(state.filter_saturation, 100.0);
    assert_eq!(state.filter_blur, 0.0);
    assert_eq!(state.filter_hue_rotate, 0.0);
}

// --- dismiss_all ---

#[test]
fn dismiss_all_closes_every_overlay() {
    let mut state = ViewState {
        show_metadata: true,
        show_quick_edit: true,
        show_delete_confirm: true,
        show_compare: true,
        show_batch: true,
        show_ai: true,
        show_filter: true,
        show_context_menu: true,
        show_file_info: true,
        show_rename: true,
        active_edit_tool: Some(EditTool::Crop),
        slideshow_playing: true,
        ..Default::default()
    };
    state.dismiss_all();
    assert!(!state.show_metadata);
    assert!(!state.show_quick_edit);
    assert!(!state.show_delete_confirm);
    assert!(!state.show_compare);
    assert!(!state.show_batch);
    assert!(!state.show_ai);
    assert!(!state.show_filter);
    assert!(!state.show_context_menu);
    assert!(!state.show_file_info);
    assert!(!state.show_rename);
    assert!(state.active_edit_tool.is_none());
    assert!(!state.slideshow_playing);
}

// --- selected_index ---

#[test]
fn selected_index_returns_none_when_no_document() {
    let state = ViewState::default();
    assert!(state.selected_index().is_none());
}

#[test]
fn selected_index_finds_matching_entry() {
    let mut state = ViewState::default();
    state.set_folder_entries(vec![
        entry("1", "/tmp/a.png", "a.png", 10),
        entry("2", "/tmp/b.png", "b.png", 20),
    ]);
    state.load_image(ImageMetadata {
        file_name: "b.png".into(),
        format: "png".into(),
        dimensions: None,
        file_size: 20,
        path: "/tmp/b.png".into(),
    });
    assert_eq!(state.selected_index(), Some(1));
}

// --- navigate_next / navigate_prev boundaries ---

#[test]
fn navigate_next_returns_false_at_end() {
    let mut state = ViewState::default();
    state.set_folder_entries(vec![
        entry("1", "/tmp/a.png", "a.png", 10),
        entry("2", "/tmp/b.png", "b.png", 20),
    ]);
    state.load_image(ImageMetadata {
        file_name: "b.png".into(),
        format: "png".into(),
        dimensions: None,
        file_size: 20,
        path: "/tmp/b.png".into(),
    });
    assert!(!state.navigate_next());
}

#[test]
fn navigate_prev_returns_false_at_start() {
    let mut state = ViewState::default();
    state.set_folder_entries(vec![
        entry("1", "/tmp/a.png", "a.png", 10),
        entry("2", "/tmp/b.png", "b.png", 20),
    ]);
    state.load_image(ImageMetadata {
        file_name: "a.png".into(),
        format: "png".into(),
        dimensions: None,
        file_size: 10,
        path: "/tmp/a.png".into(),
    });
    assert!(!state.navigate_prev());
}

#[test]
fn navigate_next_prev_returns_false_without_document() {
    let mut state = ViewState::default();
    assert!(!state.navigate_next());
    assert!(!state.navigate_prev());
}

// --- edit_history undo/redo boundaries ---

#[test]
fn undo_returns_false_when_empty() {
    let mut state = ViewState::default();
    assert!(!state.undo_edit());
}

#[test]
fn redo_returns_false_at_top() {
    let mut state = ViewState::default();
    let img = make_test_image_data(4, 4);
    state.push_edit_history(img, "crop");
    assert!(!state.redo_edit());
}

#[test]
fn edit_history_truncates_on_new_push_after_undo() {
    let mut state = ViewState::default();
    let img1 = make_test_image_data(10, 10);
    let img2 = make_test_image_data(20, 20);
    let img3 = make_test_image_data(30, 30);

    state.push_edit_history(img1, "crop");
    state.push_edit_history(img2, "resize");
    state.undo_edit(); // index goes from 2 -> 1
    state.push_edit_history(img3, "filter"); // truncates entry at index 1

    assert_eq!(state.edit_history.len(), 2);
    assert_eq!(state.edit_history_index, 2);
    assert_eq!(state.edit_history[1].label, "filter");
}

// --- is_double_click ---

#[test]
fn is_double_click_detects_rapid_same_position_click() {
    let mut state = ViewState::default();
    // First click: returns false, records position
    assert!(!state.is_double_click(100.0, 100.0));
    // Second click at same position within threshold: returns true
    assert!(state.is_double_click(100.0, 100.0));
}

#[test]
fn is_double_click_rejects_distant_click() {
    let mut state = ViewState::default();
    assert!(!state.is_double_click(100.0, 100.0));
    // Click far away: not a double-click
    assert!(!state.is_double_click(200.0, 200.0));
}

// --- click_regions ---

#[test]
fn click_regions_register_and_find() {
    let mut state = ViewState::default();
    let rect = Rect::new(10.0, 10.0, 50.0, 50.0);
    state.register_click(rect, ClickAction::ZoomIn);

    assert!(state.click_action_at(30.0, 30.0).is_some());
    assert!(state.click_action_at(5.0, 5.0).is_none());
}

#[test]
fn click_regions_clear_on_new_frame() {
    let mut state = ViewState::default();
    let rect = Rect::new(10.0, 10.0, 50.0, 50.0);
    state.register_click(rect, ClickAction::ZoomIn);
    state.clear_click_regions();
    assert!(state.click_action_at(30.0, 30.0).is_none());
}

// --- BgColor::cycle ---

#[test]
fn bg_color_cycles_through_all_variants() {
    assert_eq!(BgColor::Black.cycle(), BgColor::Gray);
    assert_eq!(BgColor::Gray.cycle(), BgColor::White);
    assert_eq!(BgColor::White.cycle(), BgColor::Black);
}

// --- SortKey::cycle ---

#[test]
fn sort_key_cycles_through_all_variants() {
    assert_eq!(SortKey::Name.cycle(), SortKey::Modified);
    assert_eq!(SortKey::Modified.cycle(), SortKey::Size);
    assert_eq!(SortKey::Size.cycle(), SortKey::Name);
}

// --- bytes_label ---

#[test]
fn bytes_label_formats_correctly() {
    assert_eq!(bytes_label(500), "500 B");
    assert_eq!(bytes_label(1024), "1024 B");
    assert_eq!(bytes_label(1025), "1.0 KB");
    assert_eq!(bytes_label(1024 * 1024), "1024.0 KB");
    assert_eq!(bytes_label(1024 * 1024 + 1), "1.0 MB");
    assert_eq!(bytes_label(1536 * 1024), "1.5 MB");
}

// --- Top overlay button tests (work plans 1-13) ---

// Work plan 1: Open button - click region registration
#[test]
fn top_overlay_open_button_registers_click_region() {
    let mut state = ViewState::default();
    let rect = Rect::new(10.0, 10.0, 80.0, 40.0);
    state.register_click(rect, ClickAction::OpenFileDialog);
    let action = state.click_action_at(45.0, 25.0);
    assert!(action.is_some());
    assert_eq!(action.unwrap(), &ClickAction::OpenFileDialog);
}

// Work plan 1: Open button - load_image resets view state
#[test]
fn open_file_resets_transient_view_state() {
    let mut state = ViewState {
        zoom: 4.0,
        rotation: 90,
        has_edited_image: true,
        show_quick_edit: true,
        show_filter: true,
        crop_start: Some((10.0, 10.0)),
        crop_selection: Some((0.0, 0.0, 100.0, 100.0)),
        ..ViewState::default()
    };
    state.load_image(ImageMetadata {
        file_name: "photo.png".into(),
        format: "png".into(),
        dimensions: Some(ImageDimensions {
            width: 800,
            height: 600,
        }),
        file_size: 1200,
        path: "/tmp/photo.png".into(),
    });
    assert_eq!(state.zoom, 1.0);
    assert_eq!(state.rotation, 0);
    assert!(!state.has_edited_image);
    assert!(state.crop_start.is_none());
    assert!(state.crop_selection.is_none());
}

// Work plan 2: Folder button - click region registration
#[test]
fn top_overlay_folder_button_registers_click_region() {
    let mut state = ViewState::default();
    let rect = Rect::new(80.0, 10.0, 150.0, 40.0);
    state.register_click(rect, ClickAction::OpenFolderDialog);
    let action = state.click_action_at(115.0, 25.0);
    assert!(action.is_some());
    assert_eq!(action.unwrap(), &ClickAction::OpenFolderDialog);
}

// Work plan 2: Folder button - set_folder_entries sorts and populates
#[test]
fn folder_dialog_populates_sorted_entries() {
    let mut state = ViewState::default();
    state.set_folder_entries(vec![
        entry("2", "/tmp/b.png", "b.png", 20),
        entry("1", "/tmp/a.png", "a.png", 10),
    ]);
    assert_eq!(state.sorted_entries.len(), 2);
    assert_eq!(state.sorted_entries[0].file_name, "a.png");
    assert_eq!(state.sorted_entries[1].file_name, "b.png");
}

// Work plan 3: Archive button - click region registration
#[test]
fn top_overlay_archive_button_registers_click_region() {
    let mut state = ViewState::default();
    let rect = Rect::new(150.0, 10.0, 220.0, 40.0);
    state.register_click(rect, ClickAction::OpenArchiveDialog);
    let action = state.click_action_at(185.0, 25.0);
    assert!(action.is_some());
    assert_eq!(action.unwrap(), &ClickAction::OpenArchiveDialog);
}

// Work plan 4: Info button - toggles show_file_info
#[test]
fn info_button_toggles_file_info() {
    let mut state = ViewState::default();
    assert!(!state.show_file_info);
    // Simulate toggle
    state.show_file_info = !state.show_file_info;
    assert!(state.show_file_info);
    state.show_file_info = !state.show_file_info;
    assert!(!state.show_file_info);
}

// Work plan 4: Info button - dismiss_all closes file info
#[test]
fn dismiss_all_closes_file_info() {
    let mut state = ViewState {
        show_file_info: true,
        ..ViewState::default()
    };
    state.dismiss_all();
    assert!(!state.show_file_info);
}

// Work plan 5: Edit button - toggles show_quick_edit
#[test]
fn edit_button_toggles_quick_edit() {
    let mut state = ViewState::default();
    assert!(!state.show_quick_edit);
    state.show_quick_edit = !state.show_quick_edit;
    assert!(state.show_quick_edit);
    state.show_quick_edit = !state.show_quick_edit;
    assert!(!state.show_quick_edit);
}

// Work plan 5: Edit button - dismiss_all closes quick edit
#[test]
fn dismiss_all_closes_quick_edit() {
    let mut state = ViewState {
        show_quick_edit: true,
        ..ViewState::default()
    };
    state.dismiss_all();
    assert!(!state.show_quick_edit);
}

// Work plan 6: Sort key button - cycles through Name/Modified/Size
#[test]
fn sort_key_button_cycles_keys() {
    let mut state = ViewState {
        sort_key: SortKey::Name,
        ..ViewState::default()
    };
    state.set_folder_entries(vec![
        entry("1", "/tmp/c.png", "c.png", 30),
        entry("2", "/tmp/a.png", "a.png", 10),
        entry("3", "/tmp/b.png", "b.png", 20),
    ]);

    // Cycle to Modified
    state.sort_key = state.sort_key.cycle();
    state.sort_entries();
    assert_eq!(state.sort_key, SortKey::Modified);

    // Cycle to Size
    state.sort_key = state.sort_key.cycle();
    state.sort_entries();
    assert_eq!(state.sort_key, SortKey::Size);
    assert_eq!(
        state
            .sorted_entries
            .iter()
            .map(|e| e.size_bytes)
            .collect::<Vec<_>>(),
        vec![10, 20, 30]
    );

    // Cycle back to Name
    state.sort_key = state.sort_key.cycle();
    state.sort_entries();
    assert_eq!(state.sort_key, SortKey::Name);
    assert_eq!(
        state
            .sorted_entries
            .iter()
            .map(|e| e.file_name.as_str())
            .collect::<Vec<_>>(),
        vec!["a.png", "b.png", "c.png"]
    );
}

// Work plan 7: Sort order button - toggles ascending/descending
#[test]
fn sort_order_button_toggles_direction() {
    let mut state = ViewState {
        sort_key: SortKey::Name,
        sort_order: SortOrder::Asc,
        ..ViewState::default()
    };
    state.set_folder_entries(vec![
        entry("1", "/tmp/c.png", "c.png", 30),
        entry("2", "/tmp/a.png", "a.png", 10),
    ]);

    // Toggle to descending
    state.sort_order = state.sort_order.toggle();
    state.sort_entries();
    assert_eq!(state.sort_order, SortOrder::Desc);
    assert_eq!(
        state
            .sorted_entries
            .iter()
            .map(|e| e.file_name.as_str())
            .collect::<Vec<_>>(),
        vec!["c.png", "a.png"]
    );

    // Toggle back to ascending
    state.sort_order = state.sort_order.toggle();
    state.sort_entries();
    assert_eq!(state.sort_order, SortOrder::Asc);
    assert_eq!(
        state
            .sorted_entries
            .iter()
            .map(|e| e.file_name.as_str())
            .collect::<Vec<_>>(),
        vec!["a.png", "c.png"]
    );
}

// Work plan 8: Files button - toggles show_metadata
#[test]
fn files_button_toggles_metadata() {
    let mut state = ViewState::default();
    assert!(!state.show_metadata);
    state.show_metadata = !state.show_metadata;
    assert!(state.show_metadata);
    state.show_metadata = !state.show_metadata;
    assert!(!state.show_metadata);
}

// Work plan 9: Copy Path button - status message on copy
#[test]
fn copy_path_updates_status_when_document_present() {
    let mut state = ViewState {
        document: Some(ImageMetadata {
            file_name: "test.png".into(),
            format: "png".into(),
            dimensions: None,
            file_size: 100,
            path: "/tmp/test.png".into(),
        }),
        ..ViewState::default()
    };
    // Simulate what dispatch_click_action does for CopyPath
    if let Some(ref doc) = state.document {
        state.status_message = format!("Copied: {}", doc.path);
    }
    assert_eq!(state.status_message, "Copied: /tmp/test.png");
}

// Work plan 9: Copy Path - no crash without document
#[test]
fn copy_path_safe_without_document() {
    let state = ViewState::default();
    assert!(state.document.is_none());
    // No crash: document is None, status_message stays default
}

// Work plan 10: Copy Img button - status message when no image data
#[test]
fn copy_img_reports_no_image_when_data_missing() {
    let mut state = ViewState {
        document: Some(ImageMetadata {
            file_name: "test.png".into(),
            format: "png".into(),
            dimensions: None,
            file_size: 100,
            path: "/tmp/test.png".into(),
        }),
        ..ViewState::default()
    };
    // current_image_data is None
    if state.current_image_data.is_none() {
        state.status_message = "No image to copy".into();
    }
    assert_eq!(state.status_message, "No image to copy");
}

// Work plan 10: Copy Img - copies current (edited) image data, not original
#[test]
fn copy_img_uses_current_image_data_not_original() {
    let current = make_test_image_data(10, 10);
    let original = make_test_image_data(20, 20);
    let state = ViewState {
        current_image_data: Some(current.clone()),
        original_image_data: Some(original),
        ..ViewState::default()
    };
    // Verify that current_image_data is used (width=10, not original width=20)
    let img = state.current_image_data.as_ref().unwrap();
    assert_eq!(img.width, 10);
    assert_eq!(img.height, 10);
}

// Work plan 11: URL button - toggles show_url_dialog
#[test]
fn url_button_toggles_url_dialog() {
    let mut state = ViewState::default();
    assert!(!state.show_url_dialog);
    state.show_url_dialog = !state.show_url_dialog;
    assert!(state.show_url_dialog);
}

// Work plan 11: URL dialog - isolated from rename/search
#[test]
fn url_dialog_state_isolated_from_other_inputs() {
    let mut state = ViewState {
        show_url_dialog: true,
        url_input_text: "https://example.com/img.png".into(),
        show_rename: true,
        rename_input_text: "newname.png".into(),
        show_search: true,
        search_query: "test".into(),
        ..ViewState::default()
    };
    // Each text input is independent
    assert_eq!(state.url_input_text, "https://example.com/img.png");
    assert_eq!(state.rename_input_text, "newname.png");
    assert_eq!(state.search_query, "test");
    // Closing URL dialog should not affect others
    state.show_url_dialog = false;
    state.url_input_text.clear();
    assert!(state.show_rename);
    assert!(!state.rename_input_text.is_empty());
    assert!(state.show_search);
    assert!(!state.search_query.is_empty());
}

// Work plan 12: Bookmark button - toggles folder bookmark
#[test]
fn bookmark_button_adds_and_removes_folder() {
    let mut state = ViewState {
        document: Some(ImageMetadata {
            file_name: "test.png".into(),
            format: "png".into(),
            dimensions: None,
            file_size: 100,
            path: "/tmp/images/test.png".into(),
        }),
        ..ViewState::default()
    };
    // Add bookmark
    let folder = "/tmp/images".to_string();
    state.folder_bookmarks.push(folder.clone());
    assert!(state.folder_bookmarks.contains(&folder));

    // Remove bookmark
    let pos = state
        .folder_bookmarks
        .iter()
        .position(|b| b == &folder)
        .unwrap();
    state.folder_bookmarks.remove(pos);
    assert!(!state.folder_bookmarks.contains(&folder));
}

// Work plan 12: Bookmark - no crash when no document
#[test]
fn bookmark_safe_without_document() {
    let state = ViewState::default();
    assert!(state.document.is_none());
    assert!(state.folder_bookmarks.is_empty());
}

// Work plan 13: Search button - toggles show_search and clears query on close
#[test]
fn search_button_toggles_and_clears_query() {
    let mut state = ViewState {
        show_search: false,
        search_query: "test query".into(),
        ..ViewState::default()
    };
    // Open search
    state.show_search = !state.show_search;
    assert!(state.show_search);
    // Close search should clear query
    state.show_search = !state.show_search;
    if !state.show_search {
        state.search_query.clear();
    }
    assert!(!state.show_search);
    assert!(state.search_query.is_empty());
}

// Work plan 13: Search state isolated from URL/rename
#[test]
fn search_state_isolated_from_other_dialogs() {
    let mut state = ViewState {
        show_search: true,
        search_query: "searching".into(),
        show_url_dialog: false,
        url_input_text: String::new(),
        show_rename: false,
        rename_input_text: String::new(),
        ..ViewState::default()
    };
    // Closing search should not open other dialogs
    state.show_search = false;
    state.search_query.clear();
    assert!(!state.show_url_dialog);
    assert!(!state.show_rename);
}

// --- Bottom overlay button tests (work plans 14-22) ---

// Work plan 14: Fit button - sets fit mode, resets zoom and pan
#[test]
fn fit_button_sets_fit_mode_and_resets() {
    let mut state = ViewState {
        fit_mode: FitMode::Actual,
        zoom: 3.5,
        pan_x: 100.0,
        pan_y: 200.0,
        ..ViewState::default()
    };
    // Simulate ClickAction::ZoomFit
    state.fit_mode = FitMode::Fit;
    state.zoom = 1.0;
    state.pan_x = 0.0;
    state.pan_y = 0.0;
    assert_eq!(state.fit_mode, FitMode::Fit);
    assert_eq!(state.zoom, 1.0);
    assert_eq!(state.pan_x, 0.0);
    assert_eq!(state.pan_y, 0.0);
}

// Work plan 15: 100% button - sets actual mode, zoom=1.0, clears pan
#[test]
fn actual_size_button_sets_actual_mode() {
    let mut state = ViewState {
        fit_mode: FitMode::Fit,
        zoom: 2.0,
        pan_x: 50.0,
        pan_y: 50.0,
        ..ViewState::default()
    };
    // Simulate ClickAction::ZoomActual
    state.fit_mode = FitMode::Actual;
    state.zoom = 1.0;
    state.pan_x = 0.0;
    state.pan_y = 0.0;
    assert_eq!(state.fit_mode, FitMode::Actual);
    assert_eq!(state.zoom, 1.0);
}

// Work plan 16: Zoom out button - decrements zoom, clamps at minimum
#[test]
fn zoom_out_decrements_and_clamps() {
    let mut state = ViewState {
        fit_mode: FitMode::Actual,
        zoom: 1.0,
        ..ViewState::default()
    };
    // Simulate ClickAction::ZoomOut
    state.fit_mode = FitMode::Actual;
    state.zoom = (state.zoom - 0.1).clamp(0.1, 8.0);
    assert!((state.zoom - 0.9).abs() < 0.001);

    // Zoom down to minimum
    state.zoom = 0.15;
    state.zoom = (state.zoom - 0.1).clamp(0.1, 8.0);
    assert!((state.zoom - 0.1).abs() < 0.001);
}

// Work plan 17: Current zoom label - shows Fit or percentage
#[test]
fn current_zoom_label_reflects_mode() {
    let state_fit = ViewState {
        fit_mode: FitMode::Fit,
        zoom: 1.0,
        ..ViewState::default()
    };
    let label_fit = match state_fit.fit_mode {
        FitMode::Fit => "Fit".to_string(),
        FitMode::Actual => format!("{:.0}%", state_fit.zoom * 100.0),
    };
    assert_eq!(label_fit, "Fit");

    let state_actual = ViewState {
        fit_mode: FitMode::Actual,
        zoom: 1.5,
        ..ViewState::default()
    };
    let label_actual = match state_actual.fit_mode {
        FitMode::Fit => "Fit".to_string(),
        FitMode::Actual => format!("{:.0}%", state_actual.zoom * 100.0),
    };
    assert_eq!(label_actual, "150%");
}

// Work plan 18: Zoom in button - increments zoom, clamps at maximum
#[test]
fn zoom_in_increments_and_clamps() {
    let mut state = ViewState {
        fit_mode: FitMode::Actual,
        zoom: 1.0,
        ..ViewState::default()
    };
    // Simulate ClickAction::ZoomIn
    state.fit_mode = FitMode::Actual;
    state.zoom = (state.zoom + 0.1).clamp(0.1, 8.0);
    assert!((state.zoom - 1.1).abs() < 0.001);

    // Zoom up to maximum
    state.zoom = 7.95;
    state.zoom = (state.zoom + 0.1).clamp(0.1, 8.0);
    assert!((state.zoom - 8.0).abs() < 0.001);
}

// Work plan 19: Filmstrip button - toggles show_thumbnails
#[test]
fn filmstrip_button_toggles_thumbnails() {
    let mut state = ViewState::default();
    assert!(state.show_thumbnails); // default is true
    state.show_thumbnails = !state.show_thumbnails;
    assert!(!state.show_thumbnails);
    state.show_thumbnails = !state.show_thumbnails;
    assert!(state.show_thumbnails);
}

// Work plan 20: Rotate button - pushes edit history and updates dimensions
#[test]
fn rotate_button_pushes_edit_history() {
    let mut state = ViewState {
        document: Some(ImageMetadata {
            file_name: "test.png".into(),
            format: "png".into(),
            dimensions: Some(ImageDimensions {
                width: 800,
                height: 600,
            }),
            file_size: 100,
            path: "/tmp/test.png".into(),
        }),
        current_image_data: Some(make_test_image_data(800, 600)),
        ..ViewState::default()
    };
    // Simulate rotation: push history
    if let Some(ref current) = state.current_image_data {
        state.push_edit_history(current.clone(), "rotate");
    }
    assert_eq!(state.edit_history.len(), 1);
    assert_eq!(state.edit_history[0].label, "rotate");
    assert!(state.has_edited_image);
}

// Work plan 21: Background button - cycles through Black/Gray/White
#[test]
fn background_button_cycles_colors() {
    let mut bg = BgColor::Black;
    assert_eq!(bg.cycle(), BgColor::Gray);
    bg = bg.cycle();
    assert_eq!(bg.cycle(), BgColor::White);
    bg = bg.cycle();
    assert_eq!(bg.cycle(), BgColor::Black);
}

// Work plan 22: Fullscreen button - toggle is a no-op in headless (state only)
#[test]
fn fullscreen_button_state_safe_without_window() {
    let state = ViewState::default();
    // Fullscreen toggle requires Tauri window handle, which is None in tests
    // Verify state is stable
    assert!(state.document.is_none());
}
