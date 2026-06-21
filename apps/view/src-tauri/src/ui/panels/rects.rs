use super::*;

// ---------------------------------------------------------------------------
// Rect computation helpers (automation fallback)
// ---------------------------------------------------------------------------

/// Computes metadata drawer button rects without rendering.
pub fn metadata_drawer_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_metadata {
        return Vec::new();
    }
    let drawer_w = 320.0_f64.min(size.width);
    let x = size.width - drawer_w;
    let pad = 18.0;
    let close_x = size.width - pad - 40.0;
    let mut rects = vec![(
        ClickAction::ToggleMetadata,
        Rect::new(close_x, 20.0, close_x + 40.0, 48.0),
    )];

    // Rating stars
    if state.document.is_some() {
        // y progression must match paint_metadata_drawer exactly:
        //   20.0 (start) + 40.0 (header) + 96.0 (histogram) + 4*24.0 (fields)
        //   + 8.0 + 24.0 (EXIF header) + 16.0 + 24.0 (Rating header)
        let mut y = 20.0 + 40.0 + 96.0 + 4.0 * 24.0 + 8.0 + 24.0 + 16.0 + 24.0;
        for r in 1u8..=5 {
            let star_x = x + pad + (r - 1) as f64 * (20.0 + 6.0);
            rects.push((
                ClickAction::SetRating(r),
                Rect::new(star_x, y, star_x + 20.0, y + 20.0),
            ));
        }
        y += 36.0;

        // Tags section header ("Tags" label) + 24.0 gap
        y += 24.0;

        // Quick-add tag buttons (must match paint_metadata_drawer quick_tags)
        let quick_tags = [
            "Favorite",
            "Landscape",
            "Portrait",
            "Nature",
            "Urban",
            "Art",
        ];
        let mut tag_x = x + pad;
        let mut tag_y = y;
        for tag_name in &quick_tags {
            let tw = 80.0; // conservative width (paint uses measure_text_width)
            if tag_x + tw > size.width - pad - 8.0 {
                tag_x = x + pad;
                tag_y += 26.0;
            }
            rects.push((
                ClickAction::ToggleTag(tag_name.to_string()),
                Rect::new(tag_x, tag_y, tag_x + tw, tag_y + 20.0),
            ));
            tag_x += tw + 6.0;
        }
    }

    rects
}

/// Computes filter panel button rects without rendering.
pub fn filter_panel_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_filter {
        return Vec::new();
    }
    let panel_w = 280.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;
    let pad = 16.0;
    let sliders_len = 5usize;
    let _panel_h = 50.0 + sliders_len as f64 * 40.0 + 50.0;

    let close_x = panel_x + panel_w - pad - 24.0;
    let mut rects = vec![(
        ClickAction::ToggleFilter,
        Rect::new(close_x, panel_y + pad, close_x + 24.0, panel_y + pad + 24.0),
    )];

    let btn_y = panel_y + pad + 30.0 + sliders_len as f64 * 40.0 + 4.0;
    let reset_x = panel_x + panel_w - pad - 140.0;
    let apply_x = panel_x + panel_w - pad - 60.0;

    rects.push((
        ClickAction::FilterReset,
        Rect::new(reset_x, btn_y, reset_x + 60.0, btn_y + 28.0),
    ));
    rects.push((
        ClickAction::FilterApply,
        Rect::new(apply_x, btn_y, apply_x + 60.0, btn_y + 28.0),
    ));

    rects
}

/// Computes AI panel button rects without rendering.
pub fn ai_panel_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_ai {
        return Vec::new();
    }
    let panel_w = 300.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;
    let pad = 16.0;
    let feature_h = 52.0;

    let close_x = panel_x + panel_w - pad - 40.0;
    let mut rects = vec![(
        ClickAction::ToggleAi,
        Rect::new(close_x, panel_y + pad, close_x + 40.0, panel_y + pad + 28.0),
    )];

    let mut y = panel_y + pad + 50.0;
    for feature in AiFeature::all() {
        let btn_rect = Rect::new(
            panel_x + pad,
            y,
            panel_x + panel_w - pad * 2.0,
            y + feature_h,
        );
        rects.push((ClickAction::SelectAiFeature(*feature), btn_rect));
        y += feature_h + 8.0;
    }

    // Run button
    if state.ai_selected_feature.is_some() {
        let run_w = 80.0;
        let run_x = panel_x + panel_w - pad - run_w;
        rects.push((
            ClickAction::RunAi,
            Rect::new(run_x, y, run_x + run_w, y + 32.0),
        ));
    }

    rects
}

/// Computes quick edit overlay button rects without rendering.
pub fn quick_edit_overlay_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_quick_edit {
        return Vec::new();
    }
    let panel_w = 260.0_f64.min(size.width - 44.0);
    let panel_x = size.width - panel_w - 22.0;
    let panel_y = 80.0;
    let pad = 14.0;

    let close_x = panel_x + panel_w - pad - 40.0;
    let mut rects = vec![(
        ClickAction::ToggleQuickEdit,
        Rect::new(close_x, panel_y + pad, close_x + 40.0, panel_y + pad + 28.0),
    )];

    let action_h = 36.0;
    let mut y = panel_y + pad + 40.0;

    let action_clicks: [ClickAction; 6] = [
        ClickAction::ContextMenuAction("Crop".to_string()),
        ClickAction::ContextMenuAction("Resize".to_string()),
        ClickAction::Rotate,
        ClickAction::ContextMenuAction("Convert".to_string()),
        ClickAction::QuickEditMarkup,
        ClickAction::CopyImage,
    ];

    for action in &action_clicks {
        let btn_rect = Rect::new(
            panel_x + pad,
            y,
            panel_x + panel_w - pad * 2.0,
            y + action_h,
        );
        rects.push((action.clone(), btn_rect));
        y += action_h + 6.0;
    }

    // Delete button
    y += 14.0;
    let delete_rect = Rect::new(
        panel_x + pad,
        y,
        panel_x + panel_w - pad * 2.0,
        y + action_h,
    );
    rects.push((
        ClickAction::ContextMenuAction("Delete".to_string()),
        delete_rect,
    ));

    // Annotation tools
    if state.active_annotation_tool.is_some() {
        y += action_h + 20.0;
        let tools: [AnnotationTool; 7] = [
            AnnotationTool::Arrow,
            AnnotationTool::Rectangle,
            AnnotationTool::Circle,
            AnnotationTool::Text,
            AnnotationTool::Freeform,
            AnnotationTool::BlurMask,
            AnnotationTool::Eraser,
        ];
        let tool_btn_w = (panel_w - pad * 2.0 - 6.0 * 4.0) / 7.0;
        for (i, tool) in tools.iter().enumerate() {
            let tx = panel_x + pad + i as f64 * (tool_btn_w + 4.0);
            rects.push((
                ClickAction::SelectAnnotationTool(*tool),
                Rect::new(tx, y, tx + tool_btn_w, y + 24.0),
            ));
        }
        y += 32.0;

        // Color picker toggle
        let swatch_rect = Rect::new(
            panel_x + pad + 42.0,
            y + 2.0,
            panel_x + pad + 58.0,
            y + 18.0,
        );
        rects.push((ClickAction::ToggleAnnotationColorPicker, swatch_rect));
        y += 28.0;

        // Line width buttons
        let line_widths: [f32; 3] = [1.0, 2.0, 4.0];
        let lw_btn_w = 36.0;
        let lw_start_x = panel_x + pad + 42.0;
        for (i, w) in line_widths.iter().enumerate() {
            let bx = lw_start_x + i as f64 * (lw_btn_w + 4.0);
            rects.push((
                ClickAction::AnnotationSetLineWidth(*w),
                Rect::new(bx, y + 2.0, bx + lw_btn_w, y + 18.0),
            ));
        }
        y += 28.0;

        // Undo / Redo buttons
        let undo_redo_w = (panel_w - pad * 2.0 - 4.0) / 2.0;
        rects.push((
            ClickAction::AnnotationUndo,
            Rect::new(panel_x + pad, y, panel_x + pad + undo_redo_w, y + 24.0),
        ));
        let redo_x = panel_x + pad + undo_redo_w + 4.0;
        rects.push((
            ClickAction::AnnotationRedo,
            Rect::new(redo_x, y, redo_x + undo_redo_w, y + 24.0),
        ));
        y += 32.0;

        // Save / Exit buttons
        let save_exit_w = (panel_w - pad * 2.0 - 4.0) / 2.0;
        rects.push((
            ClickAction::AnnotationSave,
            Rect::new(panel_x + pad, y, panel_x + pad + save_exit_w, y + 24.0),
        ));
        let exit_x = panel_x + pad + save_exit_w + 4.0;
        rects.push((
            ClickAction::AnnotationExit,
            Rect::new(exit_x, y, exit_x + save_exit_w, y + 24.0),
        ));
        y += 32.0;

        // Clear annotations button
        rects.push((
            ClickAction::ClearAnnotations,
            Rect::new(panel_x + pad, y, panel_x + panel_w - pad, y + 24.0),
        ));
    }

    rects
}

/// Computes file info overlay button rects without rendering.
pub fn file_info_overlay_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_file_info || state.document.is_none() {
        return Vec::new();
    }
    let panel_w = 360.0_f64.min(size.width - 40.0);
    let panel_x = (size.width - panel_w) / 2.0;
    let panel_y = size.height - 100.0 - 220.0;
    let pad = 16.0;
    let close_x = panel_x + panel_w - pad - 24.0;

    vec![(
        ClickAction::ToggleFileInfo,
        Rect::new(close_x, panel_y + pad, close_x + 24.0, panel_y + pad + 24.0),
    )]
}

/// Computes compare panel button rects without rendering.
pub fn compare_panel_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_compare {
        return Vec::new();
    }
    let close_x = size.width - 60.0;
    let mode_x = close_x - 70.0;
    let mut rects = vec![
        (
            ClickAction::ToggleCompare,
            Rect::new(close_x, 8.0, close_x + 50.0, 36.0),
        ),
        (
            ClickAction::CycleCompareMode,
            Rect::new(mode_x, 8.0, mode_x + 60.0, 36.0),
        ),
    ];

    // Drag area for split mode
    if state.compare_mode == CompareMode::Split {
        let split_x = size.width * state.compare_split / 100.0;
        let viewport_y = 44.0;
        rects.push((
            ClickAction::CompareDragStart,
            Rect::new(split_x - 20.0, viewport_y, split_x + 20.0, size.height),
        ));
    }

    rects
}

/// Computes help overlay button rects without rendering.
pub fn help_overlay_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_help {
        return Vec::new();
    }
    let dialog_w = 420.0_f64.min(size.width - 40.0);
    let dialog_h = 480.0_f64.min(size.height - 60.0);
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let dialog_x = cx - dialog_w / 2.0;
    let dialog_y = cy - dialog_h / 2.0;
    let pad = 20.0;
    let close_x = dialog_x + dialog_w - pad - 40.0;

    vec![(
        ClickAction::ShowHelp,
        Rect::new(
            close_x,
            dialog_y + pad,
            close_x + 40.0,
            dialog_y + pad + 28.0,
        ),
    )]
}
