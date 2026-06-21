use super::*;
use tench_view_lib::ui::state::AnnotationTool;

// ---------------------------------------------------------------------------
// Annotation actions (Phase 7: erase, line-width, undo, redo, save, exit,
//   draw-arrow, draw-rect, draw-freehand, add-text)
//
// All tests use automation_action Click to exercise the real dispatch path
// in action_metadata.rs, not direct state mutation.
// ---------------------------------------------------------------------------

/// Helper: create an app with a test image, quick edit panel open, and annotation mode active.
/// Starts with Rectangle tool selected so that clicking Arrow/Freeform/Text tools
/// can verify tool switching.
fn annotation_harness() -> TestHarness {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Rectangle);
    TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0))
}

// ---------------------------------------------------------------------------
// Erase mode toggle
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_erase_mode_toggles_via_click_ui_e2e() {
    let mut harness = annotation_harness();

    // Click the erase mode button
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.eraser".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click erase mode");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool,
        Some(AnnotationTool::Eraser),
        "eraser should be active after clicking erase mode"
    );

    // Click again to deactivate
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.eraser".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click erase mode again");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool, None,
        "eraser should be deactivated after second click"
    );
}

// ---------------------------------------------------------------------------
// Line width
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_line_width_changes_via_click_ui_e2e() {
    let mut harness = annotation_harness();

    // Default line width
    let state = read_state(&mut harness);
    assert_eq!(state.annotation_line_width, 2.0);

    // Click line width button (the debug_id includes the width * 10)
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.annotation.line_width.40".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click line width 4.0");

    let state = read_state(&mut harness);
    assert_eq!(state.annotation_line_width, 4.0);
}

// ---------------------------------------------------------------------------
// Undo
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_undo_via_click_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Arrow);

    // Manually add an annotation to have something to undo
    let color = app.state_mut().annotation_color;
    let line_width = app.state_mut().annotation_line_width;
    app.state_mut().annotation_undo_stack.push(vec![]);
    app.state_mut()
        .annotations
        .push(tench_view_lib::ui::state::Annotation {
            tool: AnnotationTool::Arrow,
            x: 10.0,
            y: 10.0,
            w: 50.0,
            h: 50.0,
            text: String::new(),
            color,
            line_width,
        });

    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click undo
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.annotation.undo".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click undo");

    let state = read_state(&mut harness);
    assert!(
        state.annotations.is_empty(),
        "undo should restore empty annotation list"
    );
    assert!(
        !state.annotation_redo_stack.is_empty(),
        "redo stack should have the undone state"
    );
}

// ---------------------------------------------------------------------------
// Redo
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_redo_via_click_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Rectangle);

    // Set up state where undo has already happened
    let color = app.state_mut().annotation_color;
    let line_width = app.state_mut().annotation_line_width;
    let annotation = tench_view_lib::ui::state::Annotation {
        tool: AnnotationTool::Arrow,
        x: 10.0,
        y: 10.0,
        w: 50.0,
        h: 50.0,
        text: String::new(),
        color,
        line_width,
    };
    app.state_mut().annotations = vec![];
    app.state_mut().annotation_redo_stack.push(vec![annotation]);

    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click redo
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.annotation.redo".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click redo");

    let state = read_state(&mut harness);
    assert_eq!(
        state.annotations.len(),
        1,
        "redo should restore the annotation"
    );
    assert_eq!(state.annotations[0].tool, AnnotationTool::Arrow);
}

// ---------------------------------------------------------------------------
// Save
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_save_via_click_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Rectangle);

    // Set up undo/redo stacks to verify they are cleared on save
    app.state_mut().annotation_undo_stack.push(vec![]);
    app.state_mut().annotation_redo_stack.push(vec![]);

    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click save
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.annotation.save".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click save");

    let state = read_state(&mut harness);
    assert!(
        state.annotation_undo_stack.is_empty(),
        "save should clear undo stack"
    );
    assert!(
        state.annotation_redo_stack.is_empty(),
        "save should clear redo stack"
    );
    assert_eq!(state.status_message, "Annotations saved");
}

// ---------------------------------------------------------------------------
// Exit (with annotations -> shows confirm)
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_exit_with_annotations_shows_confirm_via_click_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Arrow);

    // Add an annotation so exit shows confirm dialog
    let color = app.state_mut().annotation_color;
    let line_width = app.state_mut().annotation_line_width;
    app.state_mut()
        .annotations
        .push(tench_view_lib::ui::state::Annotation {
            tool: AnnotationTool::Arrow,
            x: 10.0,
            y: 10.0,
            w: 50.0,
            h: 50.0,
            text: String::new(),
            color,
            line_width,
        });

    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click exit
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.annotation.exit".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click exit");

    let state = read_state(&mut harness);
    assert!(
        state.show_annotation_exit_confirm,
        "exit confirm should appear when annotations exist"
    );
}

// ---------------------------------------------------------------------------
// Exit (without annotations -> no confirm)
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_exit_without_annotations_no_confirm_via_click_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Arrow);
    // No annotations

    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click exit
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.annotation.exit".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click exit");

    let state = read_state(&mut harness);
    assert!(
        !state.show_annotation_exit_confirm,
        "exit confirm should NOT appear when no annotations"
    );
    assert!(
        state.active_annotation_tool.is_none(),
        "tool should be deactivated"
    );
}

// ---------------------------------------------------------------------------
// Exit confirm (discard all)
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_exit_confirm_clears_all_via_click_ui_e2e() {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    app.state_mut().show_chrome = true;
    app.state_mut().show_quick_edit = true;
    app.state_mut().active_annotation_tool = Some(AnnotationTool::Arrow);
    app.state_mut().show_annotation_exit_confirm = true;

    let color = app.state_mut().annotation_color;
    let line_width = app.state_mut().annotation_line_width;
    app.state_mut()
        .annotations
        .push(tench_view_lib::ui::state::Annotation {
            tool: AnnotationTool::Arrow,
            x: 10.0,
            y: 10.0,
            w: 50.0,
            h: 50.0,
            text: String::new(),
            color,
            line_width,
        });
    app.state_mut().annotation_undo_stack.push(vec![]);
    app.state_mut().annotation_redo_stack.push(vec![]);

    let mut harness = TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0));

    // Click exit confirm
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.annotation.exit_confirm".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click exit confirm");

    let state = read_state(&mut harness);
    assert!(state.annotations.is_empty());
    assert!(state.annotation_undo_stack.is_empty());
    assert!(state.annotation_redo_stack.is_empty());
    assert!(state.active_annotation_tool.is_none());
    assert!(!state.show_annotation_exit_confirm);
}

// ---------------------------------------------------------------------------
// Text annotation: verify text tool selection via click.
// Text confirm (AnnotationTextConfirm) requires EventCtx and cannot be
// dispatched from tests. We verify the tool selection and state setup.
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_text_tool_selection_and_state_ui_e2e() {
    let mut harness = annotation_harness();

    // Select text tool via click
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.text".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click text tool");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool,
        Some(AnnotationTool::Text),
        "text tool should be selected"
    );

    // Verify text annotation state can be set up
    let app: &mut ViewApp = harness.root_widget_mut();
    app.state_mut().annotation_text_input = Some("Hello World".to_string());
    app.state_mut().annotation_drag_start = Some((50.0, 50.0));

    let state = read_state(&mut harness);
    assert_eq!(
        state.annotation_text_input.as_deref(),
        Some("Hello World"),
        "text input should be set"
    );
    assert_eq!(
        state.annotation_drag_start,
        Some((50.0, 50.0)),
        "drag start should be set"
    );
}

// ---------------------------------------------------------------------------
// Drawing tools: verify tool selection via click on quick edit overlay
// ---------------------------------------------------------------------------

#[test]
fn view_annotation_select_arrow_tool_via_click_ui_e2e() {
    let mut harness = annotation_harness();

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.arrow".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click arrow tool");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool,
        Some(AnnotationTool::Arrow),
        "arrow tool should be selected after switching from rectangle"
    );
}

#[test]
fn view_annotation_select_rectangle_tool_via_click_ui_e2e() {
    let mut harness = annotation_harness();

    // annotation_harness starts with Rectangle active.
    // Click rectangle to deselect (toggle off).
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.rect".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click rectangle tool to deselect");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool, None,
        "rectangle tool should be deselected (toggled off)"
    );

    // When active_annotation_tool is None, annotation buttons are hidden.
    // Re-enable by setting a different tool via state, then click rectangle to switch.
    {
        let pod = harness.root_mut();
        let app: &mut ViewApp = pod.widget.downcast_mut().expect("root is ViewApp");
        app.state_mut().active_annotation_tool = Some(AnnotationTool::Arrow);
    }

    // Click rectangle to select it (switching from Arrow)
    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.rect".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click rectangle tool to select");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool,
        Some(AnnotationTool::Rectangle),
        "rectangle tool should be selected after clicking"
    );
}

#[test]
fn view_annotation_select_freeform_tool_via_click_ui_e2e() {
    let mut harness = annotation_harness();

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.draw".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click freeform tool");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool,
        Some(AnnotationTool::Freeform),
        "freeform tool should be selected after clicking freeform"
    );
}

#[test]
fn view_annotation_select_text_tool_via_click_ui_e2e() {
    let mut harness = annotation_harness();

    let _capture = harness
        .automation_action(UiAutomationAction::Click {
            selector: UiAutomationSelector::ByDebugId {
                debug_id: "view.quick_edit.annotation.text".to_string(),
            },
            modifiers: Default::default(),
        })
        .expect("click text tool");

    let state = read_state(&mut harness);
    assert_eq!(
        state.active_annotation_tool,
        Some(AnnotationTool::Text),
        "text tool should be selected after clicking text"
    );
}
