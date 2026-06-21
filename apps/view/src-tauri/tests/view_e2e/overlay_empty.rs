use super::*;

// ---------------------------------------------------------------------------
// Overlay empty state (work plan items 34-40)
// ---------------------------------------------------------------------------

#[test]
fn view_overlay_empty_recent_files_shown_in_tree_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().recent_files = vec!["/test/a.png".to_string(), "/test/b.png".to_string()];
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    // Overlay empty recent slots 0 and 1 should exist
    for i in 0..2 {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: format!("view.overlay.recent.{}", i),
                }
            )
            .is_some(),
            "Expected view.overlay.recent.{} with recent files",
            i
        );
    }
    // Slots 2-4 should not exist
    for i in 2..5 {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: format!("view.overlay.recent.{}", i),
                }
            )
            .is_none(),
            "Should NOT have view.overlay.recent.{} with only 2 recent files",
            i
        );
    }
}
