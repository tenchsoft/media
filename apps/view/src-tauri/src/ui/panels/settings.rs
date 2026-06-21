use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::super::state::{ClickAction, SettingsTab, ViewState};
use super::super::theme::*;

pub fn paint_settings_panel(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let panel_w = 500.0_f64.min(size.width - 40.0);
    let panel_h = 400.0_f64.min(size.height - 40.0);
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let panel_rect = Rect::new(
        cx - panel_w / 2.0,
        cy - panel_h / 2.0,
        cx + panel_w / 2.0,
        cy + panel_h / 2.0,
    );

    let mut painter = Painter::new(scene);

    // Backdrop
    painter.fill_rect(
        Rect::from_origin_size((0.0, 0.0), size),
        Color::rgba8(0, 0, 0, 120),
    );
    state.register_click(
        Rect::from_origin_size((0.0, 0.0), size),
        ClickAction::DismissAll,
    );

    // Panel background
    painter.fill_rounded_rect(panel_rect, PANEL_BG, 10.0);
    painter.stroke_rounded_rect(panel_rect, BORDER_COLOR, 1.0, 10.0);

    // Title bar
    painter.draw_text_cached(
        text_cache,
        "Settings",
        panel_rect.x0 + 20.0,
        panel_rect.y0 + 24.0,
        TEXT_PRIMARY,
        16.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Close button (X) top right
    let close_rect = Rect::new(
        panel_rect.x1 - 40.0,
        panel_rect.y0 + 8.0,
        panel_rect.x1 - 12.0,
        panel_rect.y0 + 36.0,
    );
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.draw_text_cached(
        text_cache,
        "X",
        close_rect.x0 + 10.0,
        close_rect.y0 + 18.0,
        TEXT_PRIMARY,
        14.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::SettingsClose);

    // Tab bar on the left
    let tab_w = 130.0;
    let tab_h = 36.0;
    let tabs = [
        (SettingsTab::General, "General"),
        (SettingsTab::Image, "Image"),
        (SettingsTab::Slideshow, "Slideshow"),
        (SettingsTab::FileAssociation, "Files"),
    ];
    let mut tab_y = panel_rect.y0 + 50.0;
    for (tab, label) in &tabs {
        let tab_rect = Rect::new(
            panel_rect.x0 + 10.0,
            tab_y,
            panel_rect.x0 + 10.0 + tab_w,
            tab_y + tab_h,
        );
        let is_active = state.settings_tab == *tab;
        let bg = if is_active { ACCENT_VIEW } else { BTN_BG };
        let text_color = if is_active {
            Color::WHITE
        } else {
            TEXT_PRIMARY
        };
        painter.fill_rounded_rect(tab_rect, bg, 4.0);
        let tw = text_cache.measure_text_width(label, 12.0, FontWeight::MEDIUM);
        painter.draw_text_cached(
            text_cache,
            label,
            tab_rect.x0 + (tab_w - tw) / 2.0,
            tab_y + tab_h / 2.0 + 4.0,
            text_color,
            12.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        state.register_click(tab_rect, ClickAction::SettingsTab(*tab));
        tab_y += tab_h + 4.0;
    }

    // Content area on the right
    let content_x = panel_rect.x0 + tab_w + 30.0;
    let content_y = panel_rect.y0 + 50.0;
    let content_w = panel_w - tab_w - 50.0;

    match state.settings_tab {
        SettingsTab::General => paint_general_tab(
            state,
            text_cache,
            &mut painter,
            content_x,
            content_y,
            content_w,
        ),
        SettingsTab::Image => paint_image_tab(
            state,
            text_cache,
            &mut painter,
            content_x,
            content_y,
            content_w,
        ),
        SettingsTab::Slideshow => paint_slideshow_tab(
            state,
            text_cache,
            &mut painter,
            content_x,
            content_y,
            content_w,
        ),
        SettingsTab::FileAssociation => paint_file_assoc_tab(
            state,
            text_cache,
            &mut painter,
            content_x,
            content_y,
            content_w,
        ),
    }
}

fn paint_general_tab(
    _state: &mut ViewState,
    text_cache: &mut TextCache,
    painter: &mut Painter,
    x: f64,
    y: f64,
    _w: f64,
) {
    let mut cy = y;
    painter.draw_text_cached(
        text_cache,
        "Theme",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "System",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    cy += 28.0;

    painter.draw_text_cached(
        text_cache,
        "Default view mode",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "Fit to window",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
}

fn paint_image_tab(
    _state: &mut ViewState,
    text_cache: &mut TextCache,
    painter: &mut Painter,
    x: f64,
    y: f64,
    _w: f64,
) {
    let mut cy = y;
    painter.draw_text_cached(
        text_cache,
        "Default zoom mode",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "Fit to window",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    cy += 28.0;

    painter.draw_text_cached(
        text_cache,
        "Interpolation",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "Lanczos3",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    cy += 28.0;

    painter.draw_text_cached(
        text_cache,
        "EXIF auto-rotate",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "On",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
}

fn paint_slideshow_tab(
    _state: &mut ViewState,
    text_cache: &mut TextCache,
    painter: &mut Painter,
    x: f64,
    y: f64,
    _w: f64,
) {
    let mut cy = y;
    painter.draw_text_cached(
        text_cache,
        "Default speed",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "3 seconds",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    cy += 28.0;

    painter.draw_text_cached(
        text_cache,
        "Default transition",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "Fade",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    cy += 28.0;

    painter.draw_text_cached(
        text_cache,
        "Loop",
        x,
        cy,
        TEXT_SECONDARY,
        11.0,
        FontWeight::MEDIUM,
        false,
        false,
    );
    cy += 20.0;
    painter.draw_text_cached(
        text_cache,
        "On",
        x,
        cy,
        TEXT_PRIMARY,
        12.0,
        FontWeight::NORMAL,
        false,
        false,
    );
}

fn paint_file_assoc_tab(
    _state: &mut ViewState,
    text_cache: &mut TextCache,
    painter: &mut Painter,
    x: f64,
    y: f64,
    _w: f64,
) {
    let formats = ["PNG", "JPG", "GIF", "BMP", "WebP", "TIFF", "SVG"];
    let mut cy = y;
    for fmt in &formats {
        painter.draw_text_cached(
            text_cache,
            fmt,
            x,
            cy,
            TEXT_PRIMARY,
            12.0,
            FontWeight::NORMAL,
            false,
            false,
        );
        cy += 24.0;
    }
}
