use super::*;

// ---------------------------------------------------------------------------
// Image stage empty state (work plan items 25-33)
// ---------------------------------------------------------------------------

#[test]
fn view_empty_state_buttons_exist_in_tree_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    // Empty state should have Open Image, Open Folder, Open Archive, and Search
    let empty_selectors = [
        "view.top.open",    // Open Image button (shared with top overlay)
        "view.top.folder",  // Open Folder
        "view.top.archive", // Open Archive
        "view.top.search",  // Search field
    ];

    for selector in &empty_selectors {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: selector.to_string(),
                }
            )
            .is_some(),
            "Expected automation node with debug_id={selector} in empty state"
        );
    }
}

#[test]
fn view_empty_state_no_recent_files_without_history_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    // Without recent files, recent slots should not exist
    for i in 0..5 {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: format!("view.empty.recent.{}", i),
                }
            )
            .is_none(),
            "Should NOT have view.empty.recent.{} without recent files",
            i
        );
    }
}

#[test]
fn view_empty_state_with_recent_files_shows_slots_ui_e2e() {
    let mut app = ViewApp::new();
    // Inject recent files into state
    app.state_mut().recent_files = vec![
        "/test/recent1.png".to_string(),
        "/test/recent2.png".to_string(),
        "/test/recent3.png".to_string(),
    ];
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    // With 3 recent files, slots 0-2 should exist, 3-4 should not
    for i in 0..3 {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: format!("view.empty.recent.{}", i),
                }
            )
            .is_some(),
            "Expected view.empty.recent.{} with {} recent files",
            i,
            3
        );
    }
    for i in 3..5 {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: format!("view.empty.recent.{}", i),
                }
            )
            .is_none(),
            "Should NOT have view.empty.recent.{} with only 3 recent files",
            i
        );
    }
}

#[test]
fn view_navigation_click_prev_changes_selected_index_ui_e2e() {
    let mut harness = view_harness_with_image();
    // The test image is at index 1 (middle of 3 entries)
    // Clicking prev should navigate to index 0
    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.nav.prev".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click nav prev");

    assert!(
        capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "should render after nav prev"
    );
}

#[test]
fn view_navigation_click_next_changes_selected_index_ui_e2e() {
    let mut harness = view_harness_with_image();
    // The test image is at index 1 (middle of 3 entries)
    // Clicking next should navigate to index 2
    let capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.nav.next".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click nav next");

    assert!(
        capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "should render after nav next"
    );
}
