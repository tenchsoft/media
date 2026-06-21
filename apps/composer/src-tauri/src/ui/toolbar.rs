//! Toolbar rendering for Composer (top bar with mode tabs and action buttons).

use tench_composer_core::*;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, ComposerMode};

pub fn paint_toolbar(
    p: &mut Painter,
    state: &crate::ui::state::ComposerState,
    size: Size,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
) {
    let toolbar_h = 48.0;
    let spacing = theme.spacing;
    let spacing_large = theme.spacing_large;

    let toolbar_rect = Rect::new(0.0, 0.0, size.width, toolbar_h);
    p.fill_rect(toolbar_rect, theme.surface);

    // Project name
    p.draw_text_cached(
        text_cache,
        &state.project.name,
        spacing,
        25.0,
        theme.on_background,
        theme.font_size,
        FontWeight::BOLD,
        false,
        false,
    );

    // Mode tabs
    let mut tab_x = 140.0;
    for mode in ComposerMode::ALL {
        let tab_rect = Rect::new(tab_x, 9.0, tab_x + 64.0, 35.0);
        let active = state.mode == mode;
        if active {
            p.fill_rounded_rect(tab_rect, theme.primary, theme.border_radius);
        }
        p.draw_text_cached(
            text_cache,
            mode.label(),
            tab_x + 32.0,
            25.0,
            if active {
                theme.on_primary
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            true,
            false,
        );
        register_click(tab_rect, ClickAction::SelectMode(mode));
        tab_x += 70.0;
    }

    // Notice
    if !state.composer_notice.is_empty() {
        p.draw_text_cached(
            text_cache,
            &state.composer_notice,
            tab_x + spacing_large,
            25.0,
            theme.secondary,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
            false,
        );
    }

    // Action buttons
    let actions = ["Import", "Split", "Delete", "Export"];
    let mut action_x = (size.width - 300.0).max(tab_x + 120.0);
    for label in actions {
        let rect = Rect::new(action_x, 9.0, action_x + 66.0, 35.0);
        let enabled = !matches!(label, "Split" | "Delete") || state.selected_clip_id.is_some();
        p.fill_rounded_rect(
            rect,
            if enabled {
                theme.background
            } else {
                theme.border
            },
            theme.border_radius,
        );
        p.stroke_rounded_rect(rect, theme.border, 1.0, theme.border_radius);
        p.draw_text_cached(
            text_cache,
            label,
            action_x + 33.0,
            25.0,
            if enabled {
                theme.on_surface
            } else {
                theme.disabled
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            true,
            false,
        );
        let action = match label {
            "Import" => ClickAction::ImportMedia,
            "Split" => ClickAction::SplitAtPlayhead,
            "Delete" => ClickAction::DeleteClip(state.selected_clip_id.unwrap_or(ClipId(0))),
            "Export" => ClickAction::Export,
            _ => continue,
        };
        register_click(rect, action);
        action_x += 70.0;
    }

    p.draw_line(
        Point::new(0.0, toolbar_h),
        Point::new(size.width, toolbar_h),
        theme.border,
        1.0,
    );
}
