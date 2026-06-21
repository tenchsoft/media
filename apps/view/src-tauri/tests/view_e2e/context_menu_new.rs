use super::*;

// ---------------------------------------------------------------------------
// Context menu additions (Phase 3: rotate-l/r, set-wallpaper, open-with, properties)
//
// Tests verify that the context menu items exist and dispatch correct actions.
// The context menu debug_id pattern is "view.ctx.{label_lowercase}".
// ---------------------------------------------------------------------------

#[test]
fn view_context_menu_rotate_left_item_exists_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.rotate_left".to_string(),
            }
        )
        .is_some(),
        "context menu should contain rotate-left item"
    );
}

#[test]
fn view_context_menu_rotate_right_item_exists_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.rotate_right".to_string(),
            }
        )
        .is_some(),
        "context menu should contain rotate-right item"
    );
}

#[test]
fn view_context_menu_set_wallpaper_item_exists_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.set_as_wallpaper".to_string(),
            }
        )
        .is_some(),
        "context menu should contain set-as-wallpaper item"
    );
}

#[test]
fn view_context_menu_open_with_item_exists_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.open_with...".to_string(),
            }
        )
        .is_some(),
        "context menu should contain open-with item"
    );
}

#[test]
fn view_context_menu_properties_item_exists_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));
    let tree = harness.automation_tree();

    assert!(
        find_node(
            &tree,
            &UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.properties".to_string(),
            }
        )
        .is_some(),
        "context menu should contain properties item"
    );
}

#[test]
fn view_context_menu_rotate_left_dispatches_action_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.rotate_left".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click rotate-left in context menu");

    let state = read_state(&mut harness);
    assert!(
        !state.show_context_menu,
        "context menu should close after clicking rotate-left"
    );
}

#[test]
fn view_context_menu_set_wallpaper_dispatches_action_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_context_menu = true;
    app.state_mut().context_menu_x = 100.0;
    app.state_mut().context_menu_y = 100.0;
    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.ctx.set_as_wallpaper".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click set-as-wallpaper in context menu");

    let state = read_state(&mut harness);
    assert!(
        !state.show_context_menu,
        "context menu should close after clicking set-as-wallpaper"
    );
}
