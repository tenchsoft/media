use super::*;

// ---------------------------------------------------------------------------
// Settings panel (Phase 2)
// ---------------------------------------------------------------------------

#[test]
fn view_settings_panel_not_present_by_default_ui_e2e() {
    let mut harness = view_harness();
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.settings.close".to_string(),
            }
        )
        .is_none(),
        "Settings panel should not be present by default"
    );
}

#[test]
fn view_settings_panel_present_when_shown_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().show_settings = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.settings.close".to_string(),
            }
        )
        .is_some(),
        "Expected view.settings.close when settings panel shown"
    );
}

#[test]
fn view_settings_panel_tabs_present_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().show_settings = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    let tab_ids = [
        "view.settings.tab.general",
        "view.settings.tab.image",
        "view.settings.tab.slideshow",
        "view.settings.tab.files",
    ];
    for id in &tab_ids {
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: id.to_string(),
                }
            )
            .is_some(),
            "Expected {} in settings panel",
            id
        );
    }
}

#[test]
fn view_settings_close_dismisses_panel_ui_e2e() {
    let mut app = ViewApp::new();
    app.state_mut().show_settings = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.settings.close".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click settings close");

    let state = read_state(&mut harness);
    assert!(!state.show_settings, "settings panel should be closed");
}

#[test]
fn view_settings_tab_switch_changes_state_ui_e2e() {
    use tench_view_lib::ui::state::SettingsTab;
    let mut app = ViewApp::new();
    app.state_mut().show_settings = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.settings.tab.image".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click image tab");

    let state = read_state(&mut harness);
    assert_eq!(state.settings_tab, SettingsTab::Image);
}

#[test]
fn view_settings_general_tab_default_ui_e2e() {
    use tench_view_lib::ui::state::SettingsTab;
    let mut app = ViewApp::new();
    app.state_mut().show_settings = true;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let state = read_state(&mut harness);
    assert_eq!(state.settings_tab, SettingsTab::General);
}
