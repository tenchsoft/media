// ---------------------------------------------------------------------------
// Automation node helpers
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::ClickAction;

/// Helper: create a button automation node.
pub(super) fn make_button_node(
    debug_id: String,
    action: &ClickAction,
    rect: Rect,
) -> tench_ui::UiAutomationNode {
    make_button_node_with_role(debug_id, action, rect, false)
}

pub(super) fn make_semantic_node(
    debug_id: impl Into<String>,
    role: impl Into<String>,
    label: impl Into<String>,
    value: Option<String>,
    rect: Rect,
    enabled: bool,
) -> tench_ui::UiAutomationNode {
    tench_ui::UiAutomationNode {
        id: 0,
        debug_id: Some(debug_id.into()),
        role: role.into(),
        label: Some(label.into()),
        value,
        bounds: rect_to_automation(rect),
        enabled,
        focused: false,
        hovered: false,
        children: Vec::new(),
    }
}

fn make_button_node_with_role(
    debug_id: String,
    action: &ClickAction,
    rect: Rect,
    is_menu: bool,
) -> tench_ui::UiAutomationNode {
    tench_ui::UiAutomationNode {
        id: 0,
        debug_id: Some(debug_id),
        role: if is_menu { "menuitem" } else { "button" }.to_string(),
        label: Some(format!("{:?}", action)),
        value: None,
        bounds: rect_to_automation(rect),
        enabled: true,
        focused: false,
        hovered: false,
        children: Vec::new(),
    }
}

pub(super) fn rect_to_automation(rect: Rect) -> tench_ui::UiAutomationRect {
    tench_ui::UiAutomationRect {
        x: rect.x0,
        y: rect.y0,
        width: rect.width(),
        height: rect.height(),
    }
}
