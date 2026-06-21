//! Plan-backed Pixel Design E2E tests using the Rust-native UI harness.

use tench_pixel_design_lib::ui::{
    state::{AiTool, FileAction, JobStatus, ModalType, PanelTab, Persona, Tool},
    PixelDesignApp,
};
use tench_ui_automation_core::{
    UiAutomationAction, UiAutomationCapture, UiAutomationKey, UiAutomationModifiers,
    UiAutomationPoint, UiAutomationSelector,
};
use tench_ui_test::{
    assert_capture_changed, harness::HarnessConfig, CaptureAssertions, TestHarness,
};

fn harness() -> TestHarness {
    TestHarness::with_config(
        PixelDesignApp::new(),
        HarnessConfig::with_viewport(1280.0, 720.0),
    )
}

fn selector(debug_id: &str) -> UiAutomationSelector {
    UiAutomationSelector::debug_id(debug_id)
}

fn state(harness: &mut TestHarness) -> tench_pixel_design_lib::ui::state::PixelDesignState {
    let pod = harness.root_mut();
    let app: &mut PixelDesignApp = pod.widget.downcast_mut().expect("root is PixelDesignApp");
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

fn type_text(harness: &mut TestHarness, debug_id: &str, text: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::TypeText {
            selector: selector(debug_id),
            text: text.to_string(),
        })
        .unwrap_or_else(|error| panic!("type text into {debug_id}: {error:?}"))
}

fn key(
    harness: &mut TestHarness,
    key: UiAutomationKey,
    modifiers: UiAutomationModifiers,
) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::KeyPress { key, modifiers })
        .expect("key press")
}

fn point_in(debug_id: &str, harness: &mut TestHarness, fx: f64, fy: f64) -> UiAutomationPoint {
    let rect = harness
        .automation_bounds(&selector(debug_id))
        .unwrap_or_else(|error| panic!("bounds for {debug_id}: {error:?}"));
    UiAutomationPoint {
        x: rect.x + rect.width * fx,
        y: rect.y + rect.height * fy,
    }
}

fn press_fraction(
    harness: &mut TestHarness,
    debug_id: &str,
    fx: f64,
    fy: f64,
) -> UiAutomationCapture {
    let point = point_in(debug_id, harness, fx, fy);
    harness
        .automation_action(UiAutomationAction::Drag {
            start: point,
            end: point,
            steps: 1,
        })
        .unwrap_or_else(|error| panic!("press {debug_id}: {error:?}"))
}

fn drag_fraction(
    harness: &mut TestHarness,
    debug_id: &str,
    start: (f64, f64),
    end: (f64, f64),
) -> UiAutomationCapture {
    let start = point_in(debug_id, harness, start.0, start.1);
    let end = point_in(debug_id, harness, end.0, end.1);
    harness
        .automation_action(UiAutomationAction::Drag {
            start,
            end,
            steps: 5,
        })
        .unwrap_or_else(|error| panic!("drag {debug_id}: {error:?}"))
}

fn assert_present(capture: &UiAutomationCapture, debug_ids: &[&str]) {
    for debug_id in debug_ids {
        capture.assert_selector_present(&selector(debug_id));
    }
}

fn ai_tool_id(tool: AiTool) -> &'static str {
    match tool {
        AiTool::Inpaint => "inpaint",
        AiTool::Outpaint => "outpaint",
        AiTool::BgRemove => "bg_remove",
        AiTool::Upscale => "upscale",
        AiTool::Denoise => "denoise",
        AiTool::GenFill => "gen_fill",
        AiTool::StyleTransfer => "style_transfer",
    }
}

#[test]
fn pixel_design_plan_edit_workspace_buttons_layers_colors_and_history_use_real_ui_events_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);
    initial.assert_png_size(1280, 720);
    initial.assert_nonblank();
    assert_present(
        &initial,
        &[
            "pd.top.persona.edit",
            "pd.top.persona.ai",
            "pd.top.persona.adjust",
            "pd.top.persona.export",
            "pd.top.undo",
            "pd.top.redo",
            "pd.top.open",
            "pd.top.save",
            "pd.tool.move",
            "pd.tool.select",
            "pd.tool.brush",
            "pd.tool.eraser",
            "pd.tool.crop",
            "pd.tool.text",
            "pd.tool.shape",
            "pd.tool.fill",
            "pd.tool.gradient",
            "pd.tool.hand",
            "pd.tool.eyedropper",
            "pd.color.fg",
            "pd.color.bg",
            "pd.tab.layers",
            "pd.tab.properties",
            "pd.tab.history",
            "pd.layer.opacity",
            "pd.layer.row.0",
            "pd.layer.visibility.0",
            "pd.layer.lock.0",
            "pd.layer.add",
            "pd.layer.delete",
            "pd.layer.up",
            "pd.layer.down",
            "pd.layer.dup",
            "pd.layer.flatten",
            "pd.status.zoom_out",
            "pd.status.zoom_slider",
            "pd.status.zoom_in",
            "pd.canvas",
            "pd.auto.composited_canvas",
            "pd.auto.layer_thumbnail",
            "pd.auto.checkerboard",
            "pd.auto.status_bar",
            "pd.auto.zoom_percent",
            "pd.auto.active_control_highlight",
            "pd.auto.tool_context_chips",
            "pd.auto.canvas_layout",
        ],
    );

    for (id, persona) in [
        ("pd.top.persona.ai", Persona::AI),
        ("pd.top.persona.adjust", Persona::Adjust),
        ("pd.top.persona.export", Persona::Export),
        ("pd.top.persona.edit", Persona::Edit),
    ] {
        click(&mut harness, id);
        assert_eq!(state(&mut harness).persona, persona);
    }

    for (id, tool) in [
        ("pd.tool.move", Tool::Move),
        ("pd.tool.select", Tool::Select),
        ("pd.tool.brush", Tool::Brush),
        ("pd.tool.eraser", Tool::Eraser),
        ("pd.tool.crop", Tool::Crop),
        ("pd.tool.text", Tool::Text),
        ("pd.tool.shape", Tool::Shape),
        ("pd.tool.fill", Tool::Fill),
        ("pd.tool.gradient", Tool::Gradient),
        ("pd.tool.hand", Tool::Hand),
        ("pd.tool.eyedropper", Tool::Eyedropper),
    ] {
        click(&mut harness, id);
        // Fix 104-114: tool strip buttons set active tool, switch to Edit persona, report status
        assert_eq!(state(&mut harness).active_tool, tool);
        assert_eq!(state(&mut harness).persona, Persona::Edit);
        assert!(state(&mut harness).status_msg.contains(tool.label()));
    }

    click(&mut harness, "pd.color.fg");
    let color_open = capture(&mut harness);
    assert_present(
        &color_open,
        &[
            "pd.color_picker.hue",
            "pd.color_picker.sv",
            "pd.color_picker.apply",
            "pd.color_picker.cancel",
        ],
    );
    // Fix 78: foreground color swatch opens color picker modal
    assert!(state(&mut harness).show_color_picker);
    assert_eq!(state(&mut harness).active_modal, ModalType::ColorPicker);
    assert!(state(&mut harness).color_picker_target_fg);

    press_fraction(&mut harness, "pd.color_picker.hue", 0.35, 0.5);
    // Fix 69: hue slider updates HSV state (hue changes even if S=0 makes preview unchanged)
    let after_hue = state(&mut harness);
    assert!(after_hue.show_color_picker);
    assert_eq!(after_hue.active_modal, ModalType::ColorPicker);
    // Hue value should have changed from the initial 0.0
    assert!(after_hue.color_hue > 0.0);
    let hue_preview = after_hue.color_picker_preview.to_u32();

    press_fraction(&mut harness, "pd.color_picker.sv", 0.75, 0.25);
    // Fix 70: saturation/value control updates SV state and preview
    let after_sv = state(&mut harness);
    assert!(after_sv.show_color_picker);
    assert!(after_sv.color_saturation > 0.0);
    assert!(after_sv.color_value > 0.0);
    assert_ne!(after_sv.color_picker_preview.to_u32(), hue_preview);

    click(&mut harness, "pd.color_picker.apply");
    // Fix 67: color picker apply commits the preview color to fg_color
    assert_eq!(state(&mut harness).active_modal, ModalType::None);
    assert!(!state(&mut harness).show_color_picker);
    assert_eq!(
        state(&mut harness).fg_color.to_u32(),
        after_sv.color_picker_preview.to_u32()
    );
    assert_eq!(state(&mut harness).recent_colors.len(), 1);

    let bg_before = state(&mut harness).bg_color.to_u32();
    let recent_before_cancel = state(&mut harness).recent_colors.len();
    click(&mut harness, "pd.color.bg");
    // Fix 49: background color swatch opens picker targeting bg
    assert!(state(&mut harness).show_color_picker);
    assert!(!state(&mut harness).color_picker_target_fg);
    press_fraction(&mut harness, "pd.color_picker.hue", 0.8, 0.5);
    click(&mut harness, "pd.color_picker.cancel");
    // Fix 68: color picker cancel restores original color and closes modal
    let after_cancel = state(&mut harness);
    assert_eq!(after_cancel.active_modal, ModalType::None);
    assert!(!after_cancel.show_color_picker);
    assert_eq!(after_cancel.bg_color.to_u32(), bg_before);
    assert_eq!(after_cancel.recent_colors.len(), recent_before_cancel);

    for idx in 0..6 {
        click(&mut harness, "pd.color.fg");
        press_fraction(
            &mut harness,
            "pd.color_picker.hue",
            (idx as f64 + 1.0) / 7.0,
            0.5,
        );
        press_fraction(
            &mut harness,
            "pd.color_picker.sv",
            0.9,
            0.25 + idx as f64 * 0.04,
        );
        click(&mut harness, "pd.color_picker.apply");
    }
    let recent_capture = capture(&mut harness);
    assert_present(
        &recent_capture,
        &[
            "pd.color.recent.1",
            "pd.color.recent.2",
            "pd.color.recent.3",
            "pd.color.recent.4",
            "pd.color.recent.5",
            "pd.color.recent.6",
        ],
    );
    for idx in 1..=6 {
        let expected_color = state(&mut harness).recent_colors[idx - 1];
        click(&mut harness, &format!("pd.color.recent.{idx}"));
        // Fix 95-100: recent color swatch sets fg_color to the clicked swatch
        assert_eq!(
            state(&mut harness).fg_color.to_u32(),
            expected_color.to_u32()
        );
        assert!(state(&mut harness).status_msg.contains(&idx.to_string()));
    }

    let layer_opacity = state(&mut harness).document.layers[0].opacity;
    press_fraction(&mut harness, "pd.layer.opacity", 0.25, 0.5);
    // Fix 91: layer opacity slider changes opacity and reports status
    assert!(state(&mut harness).document.layers[0].opacity < layer_opacity);
    assert!(state(&mut harness).status_msg.contains("opacity"));

    let one_layer = state(&mut harness).document.layers.len();
    click(&mut harness, "pd.layer.add");
    let with_added = capture(&mut harness);
    assert_present(
        &with_added,
        &[
            "pd.layer.row.1",
            "pd.layer.visibility.1",
            "pd.layer.lock.1",
            "pd.auto.dirty_dot",
        ],
    );
    assert_eq!(state(&mut harness).document.layers.len(), one_layer + 1);

    click(&mut harness, "pd.layer.row.0");
    // Fix 83: layer row select updates active layer id
    assert_eq!(state(&mut harness).active_layer_index(), 0);
    assert_eq!(
        state(&mut harness).document.active_layer_id,
        state(&mut harness).document.layers[0].id
    );
    click(&mut harness, "pd.layer.visibility.0");
    // Fix 84: layer visibility toggle flips visible and reports status
    assert!(!state(&mut harness).document.layers[0].visible);
    assert!(state(&mut harness).status_msg.contains("hidden"));
    click(&mut harness, "pd.layer.lock.0");
    // Fix 82: layer lock toggle flips locked and reports status
    assert!(state(&mut harness).document.layers[0].locked);
    assert!(state(&mut harness).status_msg.contains("locked"));

    click(&mut harness, "pd.layer.row.0");
    let layer_count_before_reorder = state(&mut harness).document.layers.len();
    click(&mut harness, "pd.layer.up");
    // Fix 90: move layer up changes status
    assert!(state(&mut harness).status_msg.contains("up"));
    assert_eq!(
        state(&mut harness).document.layers.len(),
        layer_count_before_reorder
    );
    click(&mut harness, "pd.layer.down");
    // Fix 89: move layer down changes status
    assert!(state(&mut harness).status_msg.contains("down"));
    assert_eq!(
        state(&mut harness).document.layers.len(),
        layer_count_before_reorder
    );

    let before_dup = state(&mut harness).document.layers.len();
    click(&mut harness, "pd.layer.dup");
    // Fix 87: duplicate layer adds a layer and reports status
    assert_eq!(state(&mut harness).document.layers.len(), before_dup + 1);
    assert_eq!(state(&mut harness).status_msg, "Layer duplicated");
    click(&mut harness, "pd.layer.flatten");
    // Fix 88: flatten layers reduces to one layer and reports status
    assert_eq!(state(&mut harness).document.layers.len(), 1);
    assert_eq!(state(&mut harness).status_msg, "Image flattened");
    click(&mut harness, "pd.layer.add");
    // Fix 85: add layer increases count and reports status
    assert_eq!(state(&mut harness).document.layers.len(), 2);
    assert_eq!(state(&mut harness).status_msg, "Layer added");
    click(&mut harness, "pd.layer.delete");
    // Fix 86: delete layer decreases count and reports status
    assert_eq!(state(&mut harness).document.layers.len(), 1);
    assert_eq!(state(&mut harness).status_msg, "Layer deleted");

    click(&mut harness, "pd.tab.properties");
    // Fix 73: properties tab sets panel_tab state
    assert_eq!(state(&mut harness).panel_tab, PanelTab::Properties);
    let props = capture(&mut harness);
    assert_present(
        &props,
        &["pd.props.size", "pd.props.opacity", "pd.props.hardness"],
    );
    let before_size = state(&mut harness).brush_size;
    press_fraction(&mut harness, "pd.props.size", 0.25, 0.5);
    // Fix 94: brush size slider decreases size when pressed on left half
    assert!(state(&mut harness).brush_size < before_size);
    assert_eq!(
        state(&mut harness).brush_size,
        before_size.saturating_sub(4).max(1)
    );
    let before_opacity = state(&mut harness).brush_opacity;
    press_fraction(&mut harness, "pd.props.opacity", 0.25, 0.5);
    // Fix 93: brush opacity slider decreases opacity when pressed on left half
    assert!(state(&mut harness).brush_opacity < before_opacity);
    assert_eq!(
        state(&mut harness).brush_opacity,
        before_opacity.saturating_sub(5).max(1)
    );
    let before_hardness = state(&mut harness).brush_hardness;
    press_fraction(&mut harness, "pd.props.hardness", 0.25, 0.5);
    // Fix 92: brush hardness slider decreases hardness when pressed on left half
    assert!(state(&mut harness).brush_hardness < before_hardness);
    assert_eq!(
        state(&mut harness).brush_hardness,
        before_hardness.saturating_sub(5)
    );

    click(&mut harness, "pd.tool.brush");
    // Fix 104: brush tool button sets active tool and reports status
    assert_eq!(state(&mut harness).active_tool, Tool::Brush);
    assert_eq!(state(&mut harness).persona, Persona::Edit);
    assert!(state(&mut harness).status_msg.contains("Brush"));
    for (id, preset) in [
        ("pd.brush.bp1", "bp1"),
        ("pd.brush.bp2", "bp2"),
        ("pd.brush.bp3", "bp3"),
        ("pd.brush.bp4", "bp4"),
        ("pd.brush.bp5", "bp5"),
        ("pd.brush.bp6", "bp6"),
    ] {
        click(&mut harness, id);
        // Fix 50-55: brush preset buttons update brush_preset, brush_size, and status
        assert_eq!(state(&mut harness).brush_preset, preset);
        assert!(state(&mut harness).status_msg.contains("Brush preset"));
    }

    click(&mut harness, "pd.tab.history");
    // Fix 71: history tab sets panel_tab state
    assert_eq!(state(&mut harness).panel_tab, PanelTab::History);
    let history = capture(&mut harness);
    assert_present(
        &history,
        &["pd.history.undo", "pd.history.redo", "pd.history.step.0"],
    );
    let before_history = state(&mut harness).history_index;
    click(&mut harness, "pd.history.undo");
    // Fix 80: history panel undo decreases history index
    assert!(state(&mut harness).history_index < before_history);
    click(&mut harness, "pd.history.redo");
    // Fix 79: history panel redo restores history index
    assert_eq!(state(&mut harness).history_index, before_history);
    click(&mut harness, "pd.history.step.0");
    // Fix 81: history step row jumps to the selected snapshot
    assert_eq!(state(&mut harness).history_index, 0);

    click(&mut harness, "pd.top.open");
    // Fix 115: top bar open sets pending file action and status
    assert_eq!(
        state(&mut harness).pending_file_action,
        Some(FileAction::Open)
    );
    assert!(state(&mut harness).status_msg.contains("Open"));
    click(&mut harness, "pd.top.save");
    // Fix 121: top bar save reports status
    assert!(state(&mut harness).status_msg.contains("Save"));

    // Redo to push history_index above 0 so top-bar undo has something to undo.
    click(&mut harness, "pd.history.redo");
    let before_idx = state(&mut harness).history_index;
    assert!(before_idx > 0, "history_index must be > 0 for undo to work");
    click(&mut harness, "pd.top.undo");
    // Fix 122: top bar undo decreases history index and reports status
    assert!(state(&mut harness).history_index < before_idx);
    assert!(state(&mut harness).status_msg.contains("Undo"));
    click(&mut harness, "pd.top.redo");
    // Fix 120: top bar redo restores history index and reports status
    assert_eq!(state(&mut harness).history_index, before_idx);
    assert!(state(&mut harness).status_msg.contains("Redo"));
}

#[test]
fn pixel_design_plan_canvas_controls_and_automatic_behaviors_use_real_ui_events_e2e() {
    let mut harness = harness();
    let initial = capture(&mut harness);

    drag_fraction(
        &mut harness,
        "pd.canvas.brush_stroke",
        (0.38, 0.42),
        (0.52, 0.46),
    );
    let brush = capture(&mut harness);
    assert_capture_changed(&initial, &brush);
    assert_present(
        &brush,
        &[
            "pd.auto.composited_canvas",
            "pd.auto.layer_thumbnail",
            "pd.auto.dirty_dot",
        ],
    );
    // Fix 56: brush stroke commits, resets is_drawing, pushes history
    assert_eq!(state(&mut harness).status_msg, "Stroke committed");
    assert!(!state(&mut harness).is_drawing);
    assert!(state(&mut harness).history_index > 0);

    click(&mut harness, "pd.tool.eraser");
    // Fix 106/58: eraser tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Eraser);
    drag_fraction(
        &mut harness,
        "pd.canvas.eraser_stroke",
        (0.42, 0.44),
        (0.50, 0.44),
    );
    // Fix 58: eraser stroke commits and resets drawing state
    assert_eq!(state(&mut harness).status_msg, "Stroke committed");
    assert!(!state(&mut harness).is_drawing);

    click(&mut harness, "pd.tool.fill");
    // Fix 108: fill tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Fill);
    press_fraction(&mut harness, "pd.canvas.fill_click", 0.48, 0.48);
    // Fix 60: fill click fills region and pushes history
    assert_eq!(state(&mut harness).status_msg, "Filled region");
    assert!(state(&mut harness).history.len() > 1);

    click(&mut harness, "pd.tool.text");
    // Fix 114: text tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Text);
    press_fraction(&mut harness, "pd.canvas.text_placement", 0.30, 0.35);
    // Fix 66: text placement shows text input overlay
    assert!(state(&mut harness).show_text_input);
    let text_overlay = capture(&mut harness);
    assert_present(&text_overlay, &["pd.auto.text_input_overlay"]);
    key(
        &mut harness,
        UiAutomationKey::Character("A".to_string()),
        UiAutomationModifiers::default(),
    );
    key(
        &mut harness,
        UiAutomationKey::Character("B".to_string()),
        UiAutomationModifiers::default(),
    );
    key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    key(
        &mut harness,
        UiAutomationKey::Enter,
        UiAutomationModifiers::default(),
    );
    assert_eq!(state(&mut harness).status_msg, "Text added");
    // Fix 66: text commit clears input state
    assert!(!state(&mut harness).show_text_input);
    assert!(state(&mut harness).text_input.is_empty());

    click(&mut harness, "pd.tool.select");
    // Fix 112: select tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Select);
    drag_fraction(
        &mut harness,
        "pd.canvas.select_drag",
        (0.25, 0.25),
        (0.45, 0.45),
    );
    let selected = capture(&mut harness);
    assert_present(&selected, &["pd.auto.selection_overlay"]);
    // Fix 64: select drag creates selection and reports status
    assert_eq!(state(&mut harness).status_msg, "Selection updated");
    assert!(state(&mut harness).selection.is_some());

    click(&mut harness, "pd.tool.gradient");
    // Fix 109: gradient tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Gradient);
    drag_fraction(
        &mut harness,
        "pd.canvas.gradient_drag",
        (0.20, 0.30),
        (0.70, 0.45),
    );
    // Fix 61: gradient drag applies gradient and pushes history
    assert_eq!(state(&mut harness).status_msg, "Gradient applied");
    assert!(state(&mut harness).history.len() > 2);

    click(&mut harness, "pd.tool.shape");
    // Fix 113: shape tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Shape);
    drag_fraction(
        &mut harness,
        "pd.canvas.shape_drag",
        (0.30, 0.30),
        (0.55, 0.55),
    );
    // Fix 65: shape drag places shape and pushes history
    assert_eq!(state(&mut harness).status_msg, "Shape placed");
    assert!(state(&mut harness).history.len() > 3);

    click(&mut harness, "pd.tool.move");
    // Fix 111: move tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Move);
    let before_offset = state(&mut harness).document.layers[0].offset_x;
    drag_fraction(
        &mut harness,
        "pd.canvas.move_layer_drag",
        (0.38, 0.40),
        (0.50, 0.44),
    );
    // Fix 63: move layer drag changes offset, commits, and pushes history
    assert_ne!(
        state(&mut harness).document.layers[0].offset_x,
        before_offset
    );
    assert_eq!(state(&mut harness).status_msg, "Layer moved");
    assert!(!state(&mut harness).is_drawing);

    click(&mut harness, "pd.tool.hand");
    // Fix 110: hand tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Hand);
    let before_pan = state(&mut harness).viewport_offset_x;
    drag_fraction(
        &mut harness,
        "pd.canvas.hand_pan",
        (0.35, 0.40),
        (0.48, 0.46),
    );
    // Fix 62: hand pan changes viewport offset and resets panning state
    assert_ne!(state(&mut harness).viewport_offset_x, before_pan);
    assert!(!state(&mut harness).is_panning);

    click(&mut harness, "pd.tool.eyedropper");
    // Fix 107: eyedropper tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Eyedropper);
    press_fraction(&mut harness, "pd.canvas.eyedropper_click", 0.42, 0.42);
    // Fix 59: eyedropper click samples color and updates fg_color
    assert!(state(&mut harness).status_msg.contains("Sampled color"));

    click(&mut harness, "pd.tool.crop");
    // Fix 105: crop tool button sets active tool
    assert_eq!(state(&mut harness).active_tool, Tool::Crop);
    let before = state(&mut harness).document.width;
    let history_before_crop = state(&mut harness).history.len();
    drag_fraction(
        &mut harness,
        "pd.canvas.crop_drag",
        (0.20, 0.20),
        (0.74, 0.74),
    );
    // Fix 57: crop drag changes document dimensions and pushes history
    assert!(state(&mut harness).document.width < before);
    assert_eq!(state(&mut harness).status_msg, "Cropped canvas");
    assert!(state(&mut harness).history.len() > history_before_crop);

    key(
        &mut harness,
        UiAutomationKey::Character("g".to_string()),
        UiAutomationModifiers {
            control: true,
            ..Default::default()
        },
    );
    key(
        &mut harness,
        UiAutomationKey::Character("r".to_string()),
        UiAutomationModifiers {
            control: true,
            ..Default::default()
        },
    );
    let grid_rulers = capture(&mut harness);
    assert_present(&grid_rulers, &["pd.auto.grid", "pd.auto.rulers"]);

    let zoom_before = state(&mut harness).zoom;
    click(&mut harness, "pd.status.zoom_in");
    // Fix 101: zoom in increases zoom by exactly 25
    assert_eq!(state(&mut harness).zoom, zoom_before + 25);
    let after_zoom_in = state(&mut harness).zoom;
    click(&mut harness, "pd.status.zoom_out");
    // Fix 102: zoom out decreases zoom by exactly 25
    assert_eq!(state(&mut harness).zoom, after_zoom_in.saturating_sub(25));
    press_fraction(&mut harness, "pd.status.zoom_slider", 0.75, 0.5);
    // Fix 103: zoom slider sets zoom to a proportional value
    assert!(state(&mut harness).zoom > zoom_before);
}

#[test]
fn pixel_design_plan_ai_adjust_and_export_controls_use_real_ui_events_e2e() {
    let mut harness = harness();

    click(&mut harness, "pd.top.persona.ai");
    let ai = capture(&mut harness);
    assert_present(
        &ai,
        &[
            "pd.ai_tool.inpaint",
            "pd.ai_tool.outpaint",
            "pd.ai_tool.bg_remove",
            "pd.ai_tool.upscale",
            "pd.ai_tool.denoise",
            "pd.ai_tool.gen_fill",
            "pd.ai_tool.style_transfer",
            "pd.ai.prompt",
            "pd.ai.run",
            "pd.ai.cancel",
            "pd.ai.panel.inpaint",
            "pd.ai.panel.outpaint",
            "pd.ai.panel.bg_remove",
            "pd.ai.panel.upscale",
            "pd.ai.panel.denoise",
            "pd.ai.panel.gen_fill",
            "pd.ai.panel.style_transfer",
        ],
    );

    for tool in AiTool::ALL {
        click(&mut harness, &format!("pd.ai_tool.{}", ai_tool_id(tool)));
        // Fix 28-34: AI tool strip buttons set expanded_ai and report status
        assert_eq!(state(&mut harness).expanded_ai, tool);
        assert!(state(&mut harness).status_msg.contains(tool.label()));
    }
    for tool in AiTool::ALL {
        click(&mut harness, &format!("pd.ai.panel.{}", ai_tool_id(tool)));
        // Fix 21-27: AI panel tool buttons set expanded_ai
        assert_eq!(state(&mut harness).expanded_ai, tool);
    }

    let before_prompt = state(&mut harness).ai_prompt;
    // Fix 19: AI prompt field accepts text input and updates ai_prompt
    type_text(&mut harness, "pd.ai.prompt", " with mask");
    assert!(state(&mut harness).ai_prompt.len() > before_prompt.len());
    assert!(state(&mut harness).ai_prompt.contains("with mask"));
    key(
        &mut harness,
        UiAutomationKey::Enter,
        UiAutomationModifiers::default(),
    );
    // Fix 19: Enter defocuses the prompt field
    assert!(!state(&mut harness).ai_prompt_focused);

    let expanded_tool = state(&mut harness).expanded_ai;
    click(&mut harness, "pd.ai.run");
    let with_job = capture(&mut harness);
    assert_present(&with_job, &["pd.auto.ai_job_list"]);
    // Fix 20: run AI job inserts job at top with Running status, correct tool, and progress 0
    let s = state(&mut harness);
    assert!(s
        .ai_jobs
        .iter()
        .any(|job| matches!(job.status, JobStatus::Running)));
    assert_eq!(s.ai_jobs[0].tool, expanded_tool);
    assert_eq!(s.ai_jobs[0].progress, 0);
    assert!(s.status_msg.contains(expanded_tool.label()));
    click(&mut harness, "pd.ai.cancel");
    // Fix 18: AI cancel button stops running job and reports cancellation
    let after_cancel = state(&mut harness);
    assert!(after_cancel
        .ai_jobs
        .iter()
        .all(|job| !matches!(job.status, JobStatus::Running)));
    assert!(after_cancel.status_msg.contains("cancel"));

    click(&mut harness, "pd.top.persona.adjust");
    // Fix 116: persona adjust button switches persona
    assert_eq!(state(&mut harness).persona, Persona::Adjust);
    let adjust = capture(&mut harness);
    assert_present(
        &adjust,
        &[
            "pd.adjust.warm",
            "pd.adjust.cool",
            "pd.adjust.b&w",
            "pd.adjust.vintage",
            "pd.adjust.vivid",
            "pd.adjust.muted",
            "pd.adjust.film",
            "pd.adjust.hdr",
            "pd.adjust.slider.0",
            "pd.adjust.slider.1",
            "pd.adjust.slider.2",
            "pd.adjust.slider.3",
            "pd.adjust.slider.4",
            "pd.adjust.slider.5",
            "pd.adjust.slider.6",
            "pd.adjust.slider.7",
        ],
    );
    for id in [
        "pd.adjust.warm",
        "pd.adjust.cool",
        "pd.adjust.b&w",
        "pd.adjust.vintage",
        "pd.adjust.vivid",
        "pd.adjust.muted",
        "pd.adjust.film",
        "pd.adjust.hdr",
    ] {
        click(&mut harness, id);
        // Fix 07-14: adjust preset buttons set active_adjust and report status
        assert!(state(&mut harness).active_adjust.is_some());
        assert!(state(&mut harness).status_msg.contains("preset"));
    }
    for idx in 0..8 {
        let before = state(&mut harness).adjust_values.rows()[idx].1;
        press_fraction(&mut harness, &format!("pd.adjust.slider.{idx}"), 0.75, 0.5);
        // Fix 02-06, 15-17: adjust sliders nudge values by +5 on right half
        assert!(state(&mut harness).adjust_values.rows()[idx].1 > before);
        assert_eq!(
            state(&mut harness).adjust_values.rows()[idx].1,
            (before + 5).clamp(-100, 100)
        );
    }

    click(&mut harness, "pd.top.persona.export");
    // Fix 119: persona export button switches persona
    assert_eq!(state(&mut harness).persona, Persona::Export);
    let export = capture(&mut harness);
    assert_present(
        &export,
        &[
            "pd.export.format",
            "pd.export.quality",
            "pd.export.scale",
            "pd.export.button",
        ],
    );
    let format_before = state(&mut harness).export_format;
    click(&mut harness, "pd.export.format");
    // Fix 74: export format cycles to next format and reports status
    assert_ne!(state(&mut harness).export_format, format_before);
    assert!(state(&mut harness).status_msg.contains("Export format"));
    let quality_before = state(&mut harness).export_quality;
    press_fraction(&mut harness, "pd.export.quality", 0.25, 0.5);
    // Fix 76: export quality decreases by 5 on left half
    assert!(state(&mut harness).export_quality < quality_before);
    assert_eq!(
        state(&mut harness).export_quality,
        quality_before.saturating_sub(5).max(1)
    );
    let scale_before = state(&mut harness).export_scale;
    press_fraction(&mut harness, "pd.export.scale", 0.75, 0.5);
    // Fix 77: export scale increases by 10 on right half
    assert!(state(&mut harness).export_scale > scale_before);
    assert_eq!(
        state(&mut harness).export_scale,
        (scale_before + 10).min(400)
    );
    click(&mut harness, "pd.export.button");
    // Fix 75: export button sets pending file action to Export
    assert_eq!(
        state(&mut harness).pending_file_action,
        Some(FileAction::Export)
    );
    assert!(state(&mut harness).status_msg.contains("Export"));
}
