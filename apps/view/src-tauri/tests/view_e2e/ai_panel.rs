use super::*;

// ---------------------------------------------------------------------------
// AI panel (work plan items 130-137)
// ---------------------------------------------------------------------------

#[test]
fn view_ai_panel_buttons_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_ai = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ai.close".to_string()
            }
        )
        .is_some(),
        "Expected view.ai.close"
    );

    let features = [
        "view.ai.feature.enhance",
        "view.ai.feature.upscale 2x",
        "view.ai.feature.bg remove",
        "view.ai.feature.smart crop",
        "view.ai.feature.tag",
        "view.ai.feature.describe",
    ];
    for id in &features {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string()
                }
            )
            .is_some(),
            "Expected {} in AI panel",
            id
        );
    }
}

#[test]
fn view_ai_run_button_present_when_feature_selected_ui_e2e() {
    use tench_view_lib::ui::state::AiFeature;
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_ai = true;
    app.state_mut().ai_selected_feature = Some(AiFeature::Enhance);
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ai.run".to_string()
            }
        )
        .is_some(),
        "Expected view.ai.run when feature selected"
    );
}

#[test]
fn view_ai_run_button_not_present_without_selection_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_ai = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ai.run".to_string()
            }
        )
        .is_none(),
        "Should NOT have view.ai.run without selected feature"
    );
}
