use super::state::{Persona, PixelDesignState, Tool};
use super::theme;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

pub const COLOR_FG_Y: f64 = 532.0;
pub const COLOR_BG_Y: f64 = 554.0;
pub const RECENT_COLOR_Y: f64 = 592.0;

pub fn paint_top_bar(state: &PixelDesignState, p: &mut Painter<'_>, theme_ui: &Theme, size: Size) {
    let bar = Rect::new(0.0, 0.0, size.width, 48.0);
    p.fill_rect(bar, theme_ui.surface);
    p.draw_line(
        Point::new(0.0, 47.5),
        Point::new(size.width, 47.5),
        theme::BORDER,
        1.0,
    );

    for (idx, persona) in Persona::ALL.iter().enumerate() {
        let x = 12.0 + idx as f64 * 70.0;
        let rect = Rect::new(x, 8.0, x + 64.0, 40.0);
        let active = state.persona == *persona;
        p.fill_rounded_rect(
            rect,
            if active {
                theme_ui.primary
            } else {
                theme::BG_BUTTON
            },
            theme_ui.border_radius,
        );
        p.draw_text(
            persona.label(),
            x + 12.0,
            27.0,
            if active {
                theme_ui.on_primary
            } else {
                theme_ui.on_surface
            },
            theme_ui.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
    }

    let center_x = (size.width * 0.5 - 140.0).max(304.0);
    p.draw_text(
        &state.document.name,
        center_x,
        20.0,
        theme_ui.on_surface,
        theme_ui.font_size,
        FontWeight::BOLD,
        false,
    );
    // Phase 8: Dirty dot reflects actual dirty state
    if state.document.dirty {
        p.fill_circle(Point::new(center_x + 92.0, 14.0), 3.5, theme::ACCENT_YELLOW);
    }
    p.draw_text(
        &format!("{} x {}", state.document.width, state.document.height),
        center_x,
        38.0,
        theme_ui.secondary,
        theme_ui.font_size_small,
        FontWeight::NORMAL,
        false,
    );

    let ctx_x = (center_x + 190.0).min(size.width - 520.0).max(470.0);
    p.draw_text(
        state.active_tool.label(),
        ctx_x,
        20.0,
        theme_ui.primary,
        theme_ui.font_size_small,
        FontWeight::BOLD,
        false,
    );
    let mut option_x = ctx_x + 76.0;
    for option in state
        .active_tool
        .context_options(state.brush_size, state.brush_opacity, state.brush_hardness)
        .into_iter()
        .take(3)
    {
        p.fill_rounded_rect(
            Rect::new(option_x - 8.0, 9.0, option_x + 112.0, 35.0),
            theme::BG_BUTTON,
            theme_ui.border_radius,
        );
        p.draw_text(
            &option,
            option_x,
            26.0,
            theme_ui.secondary,
            theme_ui.font_size_small,
            FontWeight::NORMAL,
            false,
        );
        option_x += 124.0;
    }

    let action_x = size.width - 148.0;
    for (idx, label) in ["Undo", "Redo", "Open", "Save"].iter().enumerate() {
        let x = action_x + idx as f64 * 34.0;
        let enabled = match idx {
            0 => state.history_index > 0,
            1 => state.history_index + 1 < state.history.len(),
            _ => true,
        };
        p.fill_rounded_rect(
            Rect::new(x, 9.0, x + 28.0, 37.0),
            if enabled {
                theme::BG_BUTTON
            } else {
                theme_ui.surface
            },
            theme_ui.border_radius,
        );
        p.draw_text(
            &label[..2],
            x + 6.0,
            28.0,
            if enabled {
                theme_ui.on_surface
            } else {
                theme_ui.disabled
            },
            theme_ui.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
    }
}

pub fn paint_tool_strip(
    state: &PixelDesignState,
    p: &mut Painter<'_>,
    theme_ui: &Theme,
    top: f64,
    width: f64,
    height: f64,
) {
    let rect = Rect::new(0.0, top, width, height);
    p.fill_rect(rect, theme_ui.surface);
    p.draw_line(
        Point::new(width - 0.5, top),
        Point::new(width - 0.5, height),
        theme::BORDER,
        1.0,
    );

    if state.persona == Persona::Edit {
        for (idx, tool) in Tool::ALL.iter().enumerate() {
            paint_tool_button(
                p,
                theme_ui,
                top + 8.0 + idx as f64 * 42.0,
                tool.glyph(),
                tool.shortcut(),
                state.active_tool == *tool,
            );
        }
    } else if state.persona == Persona::AI {
        use super::state::AiTool;
        for (idx, tool) in AiTool::ALL.iter().enumerate() {
            paint_tool_button(
                p,
                theme_ui,
                top + 8.0 + idx as f64 * 46.0,
                tool.glyph(),
                "",
                state.expanded_ai == *tool,
            );
        }
    }

    // FG/BG color swatches
    let fg = Rect::new(8.0, top + COLOR_FG_Y, 36.0, top + COLOR_FG_Y + 28.0);
    let bg = Rect::new(16.0, top + COLOR_BG_Y, 44.0, top + COLOR_BG_Y + 28.0);
    p.fill_rounded_rect(bg, state.bg_color, 4.0);
    p.stroke_rounded_rect(bg, theme_ui.disabled, 1.0, 4.0);
    p.fill_rounded_rect(fg, state.fg_color, 4.0);
    p.stroke_rounded_rect(fg, Color::rgb8(0xFF, 0xFF, 0xFF), 1.0, 4.0);
    p.draw_text(
        "FG",
        10.0,
        top + COLOR_BG_Y + 24.0,
        theme_ui.secondary,
        theme_ui.font_size_small,
        FontWeight::NORMAL,
        false,
    );

    // Recent colors row (small swatches below FG/BG)
    let recent_y = top + RECENT_COLOR_Y;
    for (idx, color) in state.recent_colors.iter().take(6).enumerate() {
        let col = idx % 3;
        let row = idx / 3;
        let rx = 4.0 + col as f64 * 14.0;
        let ry = recent_y + row as f64 * 14.0;
        p.fill_rounded_rect(Rect::new(rx, ry, rx + 12.0, ry + 12.0), *color, 2.0);
    }
}

fn paint_tool_button(
    p: &mut Painter<'_>,
    theme_ui: &Theme,
    y: f64,
    glyph: &str,
    shortcut: &str,
    active: bool,
) {
    let rect = Rect::new(6.0, y, 42.0, y + 36.0);
    p.fill_rounded_rect(
        rect,
        if active {
            theme_ui.primary
        } else {
            theme::BG_BUTTON
        },
        5.0,
    );
    p.draw_text(
        glyph,
        15.0,
        y + 22.0,
        if active {
            theme_ui.on_primary
        } else {
            theme_ui.on_surface
        },
        theme_ui.font_size_small,
        FontWeight::BOLD,
        false,
    );
    if !shortcut.is_empty() {
        p.draw_text(
            shortcut,
            31.0,
            y + 32.0,
            if active {
                theme_ui.on_primary
            } else {
                theme_ui.disabled
            },
            9.0,
            FontWeight::NORMAL,
            false,
        );
    }
}
