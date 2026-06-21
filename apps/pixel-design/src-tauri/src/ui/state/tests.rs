use super::color::{hsv_to_rgb, parse_hex_color, rgb_to_hsv};
use super::*;
use tench_ui::prelude::Color;

#[test]
fn layer_actions_update_active_layer_and_history_safe_status() {
    let mut state = PixelDesignState::new();
    let original_len = state.document.layers.len();

    state.add_layer();
    assert_eq!(state.document.layers.len(), original_len + 1);
    assert_eq!(state.active_layer_index(), original_len);
    assert_eq!(state.status_msg, "Layer added");

    state.delete_layer();
    assert_eq!(state.document.layers.len(), original_len);
    assert_eq!(state.status_msg, "Layer deleted");
}

#[test]
fn canvas_crop_action_changes_document_size() {
    let mut state = PixelDesignState::new();
    state.set_active_tool(Tool::Crop);

    state.begin_canvas_action(10.0, 20.0);
    state.move_canvas_action(210.0, 120.0);
    state.finish_canvas_action();

    assert_eq!(state.document.width, 200);
    assert_eq!(state.document.height, 100);
    assert_eq!(state.status_msg, "Cropped canvas");
}

#[test]
fn text_action_commits_into_history_and_clears_input() {
    let mut state = PixelDesignState::new();
    state.set_active_tool(Tool::Text);
    state.begin_canvas_action(50.0, 60.0);
    state.text_input = "Caption".into();
    let old_history = state.history_index;

    state.commit_text_input();

    assert!(!state.show_text_input);
    assert!(state.text_input.is_empty());
    assert!(state.history_index > old_history);
    assert_eq!(state.status_msg, "Text added");
}

#[test]
fn undo_restores_document_state() {
    let mut state = PixelDesignState::new();
    let original_width = state.document.width;

    state.set_active_tool(Tool::Crop);
    state.begin_canvas_action(10.0, 20.0);
    state.move_canvas_action(210.0, 120.0);
    state.finish_canvas_action();
    assert_ne!(state.document.width, original_width);

    state.undo();
    assert_eq!(state.document.width, original_width);
}

#[test]
fn redo_restores_undone_state() {
    let mut state = PixelDesignState::new();
    let original_width = state.document.width;

    state.set_active_tool(Tool::Crop);
    state.begin_canvas_action(10.0, 20.0);
    state.move_canvas_action(210.0, 120.0);
    state.finish_canvas_action();

    state.undo();
    assert_eq!(state.document.width, original_width);

    state.redo();
    assert_eq!(state.document.width, 200);
}

#[test]
fn fill_tool_fills_pixels() {
    let mut state = PixelDesignState::new();
    state.set_active_tool(Tool::Fill);
    state.fg_color = Color::rgb8(0xFF, 0x00, 0x00);

    state.begin_canvas_action(10.0, 10.0);
    // Fill should have happened
    let (r, _, _, _) = state.document.layers[0].buffer.pixel(10, 10);
    assert_eq!(r, 0xFF);
}

#[test]
fn hsv_roundtrip() {
    let (h, s, v) = rgb_to_hsv(1.0, 0.0, 0.0);
    assert!((h - 0.0).abs() < 1.0 || (h - 360.0).abs() < 1.0);
    assert!((s - 1.0).abs() < 0.01);
    assert!((v - 1.0).abs() < 0.01);

    let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
    assert!((r - 1.0).abs() < 0.01);
    assert!((g).abs() < 0.01);
    assert!((b).abs() < 0.01);
}

#[test]
fn parse_hex_color_works() {
    let color = parse_hex_color("#FF8000").unwrap();
    let packed = color.to_u32();
    let r = ((packed >> 24) & 0xFF) as u8;
    let g = ((packed >> 16) & 0xFF) as u8;
    let b = ((packed >> 8) & 0xFF) as u8;
    assert_eq!(r, 0xFF);
    assert_eq!(g, 0x80);
    assert_eq!(b, 0x00);
}
