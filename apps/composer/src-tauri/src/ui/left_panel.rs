//! Left panel rendering for Composer (media bin, templates, effects, transitions, subtitle editor).

use tench_composer_core::*;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::state::{ClickAction, LeftPanelTab};

pub fn paint_left_panel(
    p: &mut Painter,
    state: &crate::ui::state::ComposerState,
    size: Size,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
) -> f64 {
    let left_w = state.left_panel_w;
    let toolbar_h = 48.0;
    let timeline_h = state.timeline_h;
    let spacing = theme.spacing;
    let spacing_large = theme.spacing_large;

    let left_rect = Rect::new(0.0, toolbar_h, left_w, size.height - timeline_h);
    p.fill_rect(left_rect, theme.surface);

    let mut ly = toolbar_h + spacing_large;

    p.draw_text_cached(
        text_cache,
        "PROJECT PANEL",
        spacing,
        ly,
        theme.secondary,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
        false,
    );
    ly += 20.0;

    // Left-panel tabs
    let mut left_tab_x = spacing;
    for tab in LeftPanelTab::ALL {
        let tab_rect = Rect::new(left_tab_x, ly - 8.0, left_tab_x + 48.0, ly + 10.0);
        let active = state.left_tab == tab;
        if active {
            p.fill_rounded_rect(tab_rect, theme.primary, theme.border_radius);
        }
        p.draw_text_cached(
            text_cache,
            tab.label(),
            left_tab_x + 24.0,
            ly + 4.0,
            if active {
                theme.on_primary
            } else {
                theme.secondary
            },
            10.0,
            FontWeight::MEDIUM,
            true,
            false,
        );
        register_click(tab_rect, ClickAction::SelectLeftTab(tab));
        left_tab_x += 50.0;
    }
    ly += 24.0;

    match state.left_tab {
        LeftPanelTab::Media => {
            paint_media_bin(p, state, theme, register_click, text_cache, left_w, &mut ly);
        }
        LeftPanelTab::Templates => {
            paint_templates(p, state, theme, register_click, text_cache, left_w, &mut ly);
        }
        LeftPanelTab::Effects => {
            paint_effects_list(p, state, theme, register_click, text_cache, left_w, &mut ly);
        }
        LeftPanelTab::Transitions => {
            paint_transitions_list(p, state, theme, register_click, text_cache, left_w, &mut ly);
        }
    }

    ly += spacing_large;

    // Subtitle editor
    p.draw_text_cached(
        text_cache,
        "SUBTITLE EDITOR",
        spacing,
        ly,
        theme.secondary,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
        false,
    );
    ly += 20.0;

    let sub_rect = Rect::new(spacing, ly, left_w - spacing, ly + 60.0);
    let sub_bg = if state.subtitle_focused {
        theme.background
    } else {
        theme.surface
    };
    p.fill_rounded_rect(sub_rect, sub_bg, theme.border_radius);
    let sub_border = if state.subtitle_focused {
        theme.primary
    } else {
        theme.border
    };
    p.stroke_rounded_rect(sub_rect, sub_border, 1.0, theme.border_radius);
    let display_text = if state.subtitle_text.is_empty() && !state.subtitle_focused {
        "Click to type subtitle..."
    } else {
        &state.subtitle_text
    };
    let text_color = if state.subtitle_text.is_empty() && !state.subtitle_focused {
        theme.disabled
    } else {
        theme.on_surface
    };
    p.draw_text_cached(
        text_cache,
        display_text,
        spacing + 8.0,
        ly + 20.0,
        text_color,
        theme.font_size,
        FontWeight::NORMAL,
        false,
        false,
    );
    register_click(sub_rect, ClickAction::FocusSubtitleEditor);

    // Left separator
    p.draw_line(
        Point::new(left_w, toolbar_h),
        Point::new(left_w, size.height - timeline_h),
        theme.border,
        1.0,
    );

    ly + 80.0
}

fn paint_media_bin(
    p: &mut Painter,
    state: &crate::ui::state::ComposerState,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
    left_w: f64,
    ly: &mut f64,
) {
    let spacing = theme.spacing;
    for asset in state.media_bin().iter() {
        let (icon, icon_color) = match asset.media_type {
            MediaType::Video => ("[V]", Color::rgb8(0x60, 0xA5, 0xFA)),
            MediaType::Audio => ("[A]", Color::rgb8(0x22, 0xC5, 0x5E)),
            MediaType::Image => ("[I]", Color::rgb8(0xF5, 0x9E, 0x0B)),
        };

        // Thumbnail row
        let row_rect = Rect::new(spacing, *ly - 8.0, left_w - spacing, *ly + 28.0);

        // Thumbnail placeholder
        let thumb_rect = Rect::new(
            spacing + spacing,
            *ly - 4.0,
            spacing + spacing + 28.0,
            *ly + 20.0,
        );
        p.fill_rounded_rect(thumb_rect, icon_color, 2.0);
        p.draw_text_cached(
            text_cache,
            icon,
            spacing + spacing + 14.0,
            *ly + 10.0,
            Color::WHITE,
            10.0,
            FontWeight::BOLD,
            true,
            false,
        );

        // Name + metadata
        p.draw_text_cached(
            text_cache,
            &asset.name,
            spacing + spacing + 34.0,
            *ly,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Metadata line
        let mut meta = String::new();
        if let Some(w) = asset.width {
            if let Some(h) = asset.height {
                meta.push_str(&format!("{}x{}", w, h));
            }
        }
        if let Some(dur) = asset.duration_frames {
            if !meta.is_empty() {
                meta.push_str(" | ");
            }
            let fps = state.fps();
            let secs = dur as f64 / fps;
            meta.push_str(&format!("{:.1}s", secs));
        }
        if let Some(fps) = asset.fps {
            if !meta.is_empty() {
                meta.push_str(" | ");
            }
            meta.push_str(&format!("{:.0}fps", fps));
        }
        if !meta.is_empty() {
            p.draw_text_cached(
                text_cache,
                &meta,
                spacing + spacing + 34.0,
                *ly + 14.0,
                theme.secondary,
                9.0,
                FontWeight::NORMAL,
                false,
                false,
            );
        }

        // Make the media row draggable: register as a potential drag source.
        // For now, clicking selects it for drop-on-timeline.
        register_click(row_rect, ClickAction::SelectClip(None));
        *ly += 32.0;
    }
    if state.media_bin().is_empty() {
        p.draw_text_cached(
            text_cache,
            "No media imported",
            spacing + spacing,
            *ly,
            theme.disabled,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
            false,
        );
        *ly += 20.0;
    }
    p.draw_text_cached(
        text_cache,
        &state.import_status,
        spacing + spacing,
        *ly + 8.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
        false,
    );
}

fn paint_templates(
    p: &mut Painter,
    state: &crate::ui::state::ComposerState,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
    left_w: f64,
    ly: &mut f64,
) {
    let spacing = theme.spacing;
    let templates: Vec<_> = state.templates().to_vec();
    let selected_template_idx = state.selected_template_idx;
    for (idx, template) in templates.iter().enumerate() {
        let rect = Rect::new(spacing, *ly - 8.0, left_w - spacing, *ly + 34.0);
        if selected_template_idx == Some(idx) {
            p.fill_rounded_rect(rect, theme.primary, theme.border_radius);
        } else {
            p.fill_rounded_rect(rect, theme.background, theme.border_radius);
        }
        register_click(rect, ClickAction::SelectTemplate(idx));
        p.draw_text_cached(
            text_cache,
            &template.name,
            spacing + 8.0,
            *ly + 8.0,
            if state.selected_template_idx == Some(idx) {
                theme.on_primary
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::BOLD,
            false,
            false,
        );
        p.draw_text_cached(
            text_cache,
            &format!(
                "{} | {}x{} @ {:.0}fps",
                template.category, template.width, template.height, template.fps
            ),
            spacing + 8.0,
            *ly + 24.0,
            if state.selected_template_idx == Some(idx) {
                theme.on_primary
            } else {
                theme.secondary
            },
            10.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        *ly += 46.0;
    }
}

fn paint_effects_list(
    p: &mut Painter,
    state: &crate::ui::state::ComposerState,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
    left_w: f64,
    ly: &mut f64,
) {
    let spacing = theme.spacing;
    // Search box
    let search_rect = Rect::new(spacing, *ly - 4.0, left_w - spacing, *ly + 16.0);
    p.fill_rounded_rect(search_rect, theme.background, theme.border_radius);
    p.stroke_rounded_rect(search_rect, theme.border, 1.0, theme.border_radius);
    let search_text = if state.effects_search.is_empty() {
        "Search effects..."
    } else {
        &state.effects_search
    };
    p.draw_text_cached(
        text_cache,
        search_text,
        spacing + 8.0,
        *ly + 8.0,
        if state.effects_search.is_empty() {
            theme.disabled
        } else {
            theme.on_surface
        },
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
        false,
    );
    register_click(search_rect, ClickAction::FocusEffectsSearch);
    *ly += 28.0;

    let search_lower = state.effects_search.to_lowercase();
    for effect in VideoEffectType::ALL {
        if !search_lower.is_empty() && !effect.label().to_lowercase().contains(&search_lower) {
            continue;
        }
        let row_rect = Rect::new(spacing, *ly - 12.0, left_w - spacing, *ly + 8.0);
        p.draw_text_cached(
            text_cache,
            effect.label(),
            spacing + spacing,
            *ly,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
        register_click(row_rect, ClickAction::ApplyEffect(effect));
        *ly += 22.0;
    }
    for synthetic in ["Rotate"] {
        if !search_lower.is_empty() && !synthetic.to_lowercase().contains(&search_lower) {
            continue;
        }
        let row_rect = Rect::new(spacing, *ly - 12.0, left_w - spacing, *ly + 8.0);
        p.draw_text_cached(
            text_cache,
            synthetic,
            spacing + spacing,
            *ly,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
        register_click(
            row_rect,
            ClickAction::RunAiFeature(format!("Effect {synthetic}")),
        );
        *ly += 22.0;
    }
}

fn paint_transitions_list(
    p: &mut Painter,
    state: &crate::ui::state::ComposerState,
    theme: &Theme,
    register_click: &mut dyn FnMut(Rect, ClickAction),
    text_cache: &mut TextCache,
    left_w: f64,
    ly: &mut f64,
) {
    let spacing = theme.spacing;
    // Search box
    let search_rect = Rect::new(spacing, *ly - 4.0, left_w - spacing, *ly + 16.0);
    p.fill_rounded_rect(search_rect, theme.background, theme.border_radius);
    p.stroke_rounded_rect(search_rect, theme.border, 1.0, theme.border_radius);
    let search_text = if state.transitions_search.is_empty() {
        "Search transitions..."
    } else {
        &state.transitions_search
    };
    p.draw_text_cached(
        text_cache,
        search_text,
        spacing + 8.0,
        *ly + 8.0,
        if state.transitions_search.is_empty() {
            theme.disabled
        } else {
            theme.on_surface
        },
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
        false,
    );
    register_click(search_rect, ClickAction::FocusTransitionsSearch);
    *ly += 28.0;

    let search_lower = state.transitions_search.to_lowercase();
    for t in TransitionType::ALL {
        if !search_lower.is_empty() && !t.label().to_lowercase().contains(&search_lower) {
            continue;
        }
        let row_rect = Rect::new(spacing, *ly - 12.0, left_w - spacing, *ly + 8.0);
        p.draw_text_cached(
            text_cache,
            t.label(),
            spacing + spacing,
            *ly,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
        register_click(row_rect, ClickAction::ApplyTransition(t));
        *ly += 22.0;
    }
    for synthetic in ["Zoom"] {
        if !search_lower.is_empty() && !synthetic.to_lowercase().contains(&search_lower) {
            continue;
        }
        let row_rect = Rect::new(spacing, *ly - 12.0, left_w - spacing, *ly + 8.0);
        p.draw_text_cached(
            text_cache,
            synthetic,
            spacing + spacing,
            *ly,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
            false,
        );
        register_click(
            row_rect,
            ClickAction::RunAiFeature(format!("Transition {synthetic}")),
        );
        *ly += 22.0;
    }
}
