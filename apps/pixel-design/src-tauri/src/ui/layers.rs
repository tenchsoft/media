use super::state::{PanelTab, PixelDesignState};
use super::theme;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

pub fn paint_edit_panel(state: &PixelDesignState, p: &mut Painter<'_>, theme: &Theme, panel: Rect) {
    paint_panel_shell(p, panel);
    paint_tabs(state, p, theme, panel);

    match state.panel_tab {
        PanelTab::Layers => paint_layers_tab(state, p, theme, panel),
        PanelTab::Properties => paint_properties_tab(state, p, theme, panel),
        PanelTab::History => paint_history_tab(state, p, theme, panel),
    }

    if state.active_tool.uses_brush() {
        paint_brush_presets(state, p, theme, panel);
    }
}

fn paint_panel_shell(p: &mut Painter<'_>, panel: Rect) {
    p.fill_rect(panel, theme::BG_SURFACE);
    p.draw_line(
        Point::new(panel.x0, panel.y0),
        Point::new(panel.x0, panel.y1),
        theme::BORDER,
        1.0,
    );
}

fn paint_tabs(state: &PixelDesignState, p: &mut Painter<'_>, theme: &Theme, panel: Rect) {
    for (idx, tab) in PanelTab::ALL.iter().enumerate() {
        let x = panel.x0 + 10.0 + idx as f64 * 88.0;
        let rect = Rect::new(x, panel.y0 + 10.0, x + 82.0, panel.y0 + 40.0);
        let active = state.panel_tab == *tab;
        p.fill_rounded_rect(
            rect,
            if active {
                theme::BG_TAB_ACTIVE
            } else {
                theme::BG_TAB_INACTIVE
            },
            4.0,
        );
        p.draw_text(
            tab.label(),
            x + 10.0,
            panel.y0 + 29.0,
            if active {
                theme::TEXT_ON_ACCENT
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
    }
}

fn paint_layers_tab(state: &PixelDesignState, p: &mut Painter<'_>, theme: &Theme, panel: Rect) {
    let x = panel.x0 + 16.0;
    let mut y = panel.y0 + 66.0;
    p.draw_text(
        &format!("Layers {}", state.document.layers.len()),
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 26.0;

    let opacity = state
        .active_layer()
        .map(|layer| (layer.opacity * 100.0) as u32)
        .unwrap_or(100);
    paint_labeled_bar(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 28.0),
        "Opacity",
        opacity as f64 / 100.0,
        &format!("{opacity}%"),
    );
    y += 42.0;

    for (idx, layer) in state.document.layers.iter().enumerate() {
        let row = Rect::new(x - 2.0, y, panel.x1 - 14.0, y + 36.0);
        let active = idx == state.active_layer_index();
        p.fill_rounded_rect(
            row,
            if active {
                theme::BG_ROW_ACTIVE
            } else {
                theme::BG_ROW_INACTIVE
            },
            5.0,
        );
        p.draw_text(
            if layer.visible { "ON" } else { "--" },
            x + 6.0,
            y + 23.0,
            theme.secondary,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        let thumb = Rect::new(x + 40.0, y + 7.0, x + 62.0, y + 29.0);
        p.fill_rounded_rect(thumb, theme::BG_THUMBNAIL, 3.0);
        // Phase 3: render actual thumbnail
        if let Some(thumb_buf) = state.layer_thumbnails.get(&layer.id) {
            let img_data = tench_ui::peniko::ImageData {
                width: thumb_buf.width,
                height: thumb_buf.height,
                format: tench_ui::peniko::ImageFormat::Rgba8,
                alpha_type: tench_ui::peniko::ImageAlphaType::AlphaPremultiplied,
                data: thumb_buf.data.clone().into(),
            };
            p.draw_image(&img_data, thumb);
        }
        p.draw_text(
            &layer.name,
            x + 72.0,
            y + 17.0,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        let opacity_pct = (layer.opacity * 100.0) as u32;
        p.draw_text(
            &format!("{} / {}%", layer.blend_mode.label(), opacity_pct),
            x + 72.0,
            y + 31.0,
            theme.secondary,
            10.0,
            FontWeight::NORMAL,
            false,
        );
        if layer.locked {
            p.draw_text(
                "LOCK",
                panel.x1 - 52.0,
                y + 31.0,
                theme.disabled,
                9.0,
                FontWeight::BOLD,
                false,
            );
        }
        y += 42.0;
    }

    let add = Rect::new(x, panel.y0 + 262.0, x + 116.0, panel.y0 + 294.0);
    let del = Rect::new(x + 130.0, panel.y0 + 262.0, x + 254.0, panel.y0 + 294.0);
    paint_action_button(p, theme, add, "+ Layer");
    paint_action_button(p, theme, del, "Delete");

    // Phase 3: Reorder buttons
    let up = Rect::new(x, panel.y0 + 300.0, x + 116.0, panel.y0 + 328.0);
    let down = Rect::new(x + 130.0, panel.y0 + 300.0, x + 254.0, panel.y0 + 328.0);
    paint_action_button(p, theme, up, "Move Up");
    paint_action_button(p, theme, down, "Move Down");

    // Phase 3: Context menu buttons
    let dup = Rect::new(x, panel.y0 + 336.0, x + 116.0, panel.y0 + 364.0);
    let flat = Rect::new(x + 130.0, panel.y0 + 336.0, x + 254.0, panel.y0 + 364.0);
    paint_action_button(p, theme, dup, "Duplicate");
    paint_action_button(p, theme, flat, "Flatten");
}

fn paint_properties_tab(state: &PixelDesignState, p: &mut Painter<'_>, theme: &Theme, panel: Rect) {
    let x = panel.x0 + 18.0;
    let mut y = panel.y0 + 66.0;
    p.draw_text(
        state.active_tool.label(),
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 26.0;
    paint_labeled_bar(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 30.0),
        "Size",
        state.brush_size as f64 / 200.0,
        &state.brush_size.to_string(),
    );
    y += 48.0;
    paint_labeled_bar(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 30.0),
        "Opacity",
        state.brush_opacity as f64 / 100.0,
        &format!("{}%", state.brush_opacity),
    );
    y += 48.0;
    paint_labeled_bar(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 30.0),
        "Hardness",
        state.brush_hardness as f64 / 100.0,
        &format!("{}%", state.brush_hardness),
    );
    y += 52.0;
    p.draw_text(
        "Foreground",
        x,
        y,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    p.fill_rounded_rect(
        Rect::new(x + 98.0, y - 14.0, x + 138.0, y + 12.0),
        state.fg_color,
        4.0,
    );
}

fn paint_history_tab(state: &PixelDesignState, p: &mut Painter<'_>, theme: &Theme, panel: Rect) {
    let x = panel.x0 + 20.0;
    p.draw_text(
        &format!("History {}", state.history_index + 1),
        x,
        panel.y0 + 78.0,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    paint_action_button(
        p,
        theme,
        Rect::new(x, panel.y0 + 92.0, x + 110.0, panel.y0 + 124.0),
        "Undo",
    );
    paint_action_button(
        p,
        theme,
        Rect::new(x + 126.0, panel.y0 + 92.0, x + 236.0, panel.y0 + 124.0),
        "Redo",
    );
    let mut y = panel.y0 + 152.0;
    for idx in 0..state.history.len().min(8) {
        let active = idx == state.history_index;
        p.draw_text(
            &state.history[idx].label,
            x,
            y,
            if active {
                theme::ACCENT
            } else {
                theme.secondary
            },
            theme.font_size_small,
            if active {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            },
            false,
        );
        y += 20.0;
    }
}

fn paint_brush_presets(state: &PixelDesignState, p: &mut Painter<'_>, theme: &Theme, panel: Rect) {
    let x = panel.x0 + 16.0;
    let y = panel.y0 + 380.0;
    p.draw_text(
        "Brush Presets",
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    for (idx, preset) in PixelDesignState::brush_presets().iter().enumerate() {
        let col = idx % 2;
        let row = idx / 2;
        let rx = x + col as f64 * 128.0;
        let ry = y + 20.0 + row as f64 * 58.0;
        let active = state.brush_preset == preset.id;
        p.fill_rounded_rect(
            Rect::new(rx, ry, rx + 116.0, ry + 50.0),
            if active {
                theme::BG_ROW_ACTIVE
            } else {
                theme::BG_ROW_INACTIVE
            },
            5.0,
        );
        p.fill_circle(
            Point::new(rx + 26.0, ry + 25.0),
            (preset.size as f64 / 3.0).clamp(3.0, 16.0),
            theme.on_surface,
        );
        p.draw_text(
            preset.name,
            rx + 48.0,
            ry + 29.0,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
    }
}

fn paint_labeled_bar(
    p: &mut Painter<'_>,
    theme: &Theme,
    rect: Rect,
    label: &str,
    amount: f64,
    value: &str,
) {
    p.draw_text(
        label,
        rect.x0,
        rect.y0 + 18.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    let bar = Rect::new(
        rect.x0 + 76.0,
        rect.y0 + 8.0,
        rect.x1 - 48.0,
        rect.y0 + 18.0,
    );
    p.fill_rounded_rect(bar, theme::BG_BAR, 5.0);
    p.fill_rounded_rect(
        Rect::new(
            bar.x0,
            bar.y0,
            bar.x0 + bar.width() * amount.clamp(0.0, 1.0),
            bar.y1,
        ),
        theme::ACCENT,
        5.0,
    );
    p.draw_text(
        value,
        rect.x1 - 42.0,
        rect.y0 + 18.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::MEDIUM,
        false,
    );
}

fn paint_action_button(p: &mut Painter<'_>, theme: &Theme, rect: Rect, label: &str) {
    p.fill_rounded_rect(rect, theme::BG_BAR, 4.0);
    p.draw_text(
        label,
        rect.x0 + 12.0,
        rect.y0 + 20.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::MEDIUM,
        false,
    );
}
