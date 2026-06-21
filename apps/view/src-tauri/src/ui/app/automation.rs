// ---------------------------------------------------------------------------
// Automation tree assembly
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::ClickAction;
use crate::ui::{controls, image_stage, overlays, panels, tools};

use super::automation_extra::extra_view_automation_nodes;
use super::automation_ids::action_debug_id;
use super::automation_nodes::{make_button_node, rect_to_automation};
use super::ViewApp;

impl ViewApp {
    pub(super) fn automation_children_nodes(
        &self,
        state: &tench_ui::core::widget::WidgetState,
    ) -> Vec<tench_ui::UiAutomationNode> {
        let size = state.size;

        // Use paint-time click_regions as the single source of truth for
        // automation node bounds.  This guarantees that the automation tree
        // exposes exactly the same interactive regions that were painted and
        // registered during the last paint frame, eliminating coordinate drift
        // between paint-time hit targets and the automation rects.
        //
        // For overlays that may not be painted (e.g. show_chrome = false),
        // we still fall back to the computed button rects so that selectors
        // remain discoverable in the automation tree.
        let click_regions: Vec<(ClickAction, Rect)> = self
            .state
            .click_regions
            .iter()
            .map(|r| (r.action.clone(), r.rect))
            .collect();

        // Fallback computed rects for overlays that are not always painted.
        let top_rects = overlays::top_overlay_button_rects(&self.state, size.width);
        let bottom_rects = overlays::bottom_overlay_button_rects(&self.state, size);
        let nav_rects = overlays::nav_edge_button_rects(&self.state, size);
        let overlay_empty_rects = overlays::overlay_empty_button_rects(&self.state, size);
        let empty_state_rects = image_stage::empty_state_button_rects(&self.state, size);
        let ctx_menu_rects = controls::context_menu_item_rects(&self.state, size);

        // Slideshow, batch, dialogs, panels, tools, overlays
        let slideshow_rects = controls::slideshow_controls_button_rects(&self.state, size);
        let batch_rects = controls::batch_panel_button_rects(&self.state, size);
        let delete_rects = controls::delete_confirm_button_rects(&self.state, size);
        let edit_banner_rects = controls::edit_banner_button_rects(&self.state, size);
        let rename_rects = controls::rename_dialog_button_rects(&self.state, size);
        let batch_trigger_rects = controls::batch_trigger_button_rect(&self.state, size);

        let metadata_rects = panels::metadata_drawer_button_rects(&self.state, size);
        let filter_rects = panels::filter_panel_button_rects(&self.state, size);
        let ai_rects = panels::ai_panel_button_rects(&self.state, size);
        let quick_edit_rects = panels::quick_edit_overlay_button_rects(&self.state, size);
        let file_info_rects = panels::file_info_overlay_button_rects(&self.state, size);
        let compare_rects = panels::compare_panel_button_rects(&self.state, size);
        let help_rects = panels::help_overlay_button_rects(&self.state, size);

        let crop_rects = tools::crop_tool_button_rects(&self.state, size);
        let resize_rects = tools::resize_tool_button_rects(&self.state, size);
        let convert_rects = tools::convert_tool_button_rects(&self.state, size);

        let url_rects = overlays::url_dialog_button_rects(&self.state, size);
        let print_rects = overlays::print_dialog_button_rects(&self.state, size);

        // Build a lookup from ClickAction to its paint-time rect.
        // Click-regions are the authoritative source; computed rects are
        // fallback for elements not yet painted (e.g. hidden overlays).
        use std::collections::HashMap;
        let mut paint_rects: HashMap<String, (ClickAction, Rect)> = HashMap::new();
        for (action, rect) in &click_regions {
            if let Some(id) = action_debug_id(action) {
                paint_rects.entry(id).or_insert((action.clone(), *rect));
            }
        }

        let computed_rects: Vec<(ClickAction, Rect)> = top_rects
            .into_iter()
            .chain(bottom_rects)
            .chain(nav_rects)
            .chain(overlay_empty_rects)
            .chain(empty_state_rects)
            .chain(slideshow_rects)
            .chain(batch_rects)
            .chain(delete_rects)
            .chain(edit_banner_rects)
            .chain(rename_rects)
            .chain(batch_trigger_rects)
            .chain(metadata_rects)
            .chain(filter_rects)
            .chain(ai_rects)
            .chain(quick_edit_rects)
            .chain(file_info_rects)
            .chain(compare_rects)
            .chain(help_rects)
            .chain(crop_rects)
            .chain(resize_rects)
            .chain(convert_rects)
            .chain(url_rects)
            .chain(print_rects)
            .collect();

        // Merge: paint-time rects take priority; computed rects fill gaps.
        let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut all_rects: Vec<(ClickAction, Rect)> = Vec::new();

        // First, emit all paint-time rects.
        for (id, (action, rect)) in &paint_rects {
            all_rects.push((action.clone(), *rect));
            seen_ids.insert(id.clone());
        }

        // Then, emit computed rects that weren't covered by paint-time rects.
        for (action, rect) in computed_rects {
            if let Some(id) = action_debug_id(&action) {
                if !seen_ids.contains(&id) {
                    all_rects.push((action, rect));
                    seen_ids.insert(id);
                }
            }
        }

        let mut nodes: Vec<tench_ui::UiAutomationNode> = all_rects
            .into_iter()
            .filter_map(|(action, rect)| {
                let debug_id: Option<String> = action_debug_id(&action);
                debug_id.map(|id| make_button_node(id, &action, rect))
            })
            .collect();

        // Context menu items
        for (label, rect) in ctx_menu_rects {
            nodes.push(tench_ui::UiAutomationNode {
                id: 0,
                debug_id: Some(format!(
                    "view.ctx.{}",
                    label.to_lowercase().replace(' ', "_")
                )),
                role: "menuitem".to_string(),
                label: Some(label),
                value: None,
                bounds: rect_to_automation(rect),
                enabled: true,
                focused: false,
                hovered: false,
                children: Vec::new(),
            });
        }

        nodes.extend(extra_view_automation_nodes(&self.state, size));

        nodes
    }
}
