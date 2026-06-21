use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use super::super::state::{ClickAction, ViewState};
use super::super::theme::{BORDER_COLOR, ERROR_COLOR, PANEL_BG, TEXT_PRIMARY};

// Context Menu

/// Computes context menu item rects without rendering.
/// Returns a list of (label, Rect) for each menu item.
pub fn context_menu_item_rects(state: &ViewState, size: Size) -> Vec<(String, Rect)> {
    let mut rects = Vec::new();

    if !state.show_context_menu {
        return rects;
    }

    let menu_x = state.context_menu_x.min(size.width - 200.0);
    let menu_y = state.context_menu_y.min(size.height - 300.0);
    let has_image = state.document.is_some();

    let items: Vec<&str> = if has_image {
        vec![
            "Open Image",
            "Open Folder",
            "Filters",
            "Metadata",
            "Rotate Left",
            "Rotate Right",
            "Set as Wallpaper",
            "Open With...",
            "Show in Files",
            "Copy Path",
            "Copy Image",
            "Rename",
            "Print",
            "Properties",
            "Delete",
        ]
    } else {
        vec!["Open Image", "Open Folder"]
    };

    let item_h = 30.0;
    let menu_w = 180.0;

    for (i, label) in items.iter().enumerate() {
        let y = menu_y + i as f64 * item_h;
        rects.push((
            label.to_string(),
            Rect::new(menu_x, y, menu_x + menu_w, y + item_h),
        ));
    }

    rects
}

/// Draws the right-click context menu.
/// Matches `.context-menu` CSS: fixed position, min 180px, list of items.
pub fn paint_context_menu(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let menu_x = state.context_menu_x.min(size.width - 200.0);
    let menu_y = state.context_menu_y.min(size.height - 300.0);

    let has_image = state.document.is_some();

    // Menu items
    let items: Vec<MenuItem> = if has_image {
        vec![
            MenuItem::new("Open Image"),
            MenuItem::new("Open Folder"),
            MenuItem::divider(),
            MenuItem::new("Filters"),
            MenuItem::new("Metadata"),
            MenuItem::new("Rotate Left"),
            MenuItem::new("Rotate Right"),
            MenuItem::new("Set as Wallpaper"),
            MenuItem::new("Open With..."),
            MenuItem::new("Show in Files"),
            MenuItem::new("Copy Path"),
            MenuItem::new("Copy Image"),
            MenuItem::new("Rename"),
            MenuItem::new("Print"),
            MenuItem::new("Properties"),
            MenuItem::danger("Delete"),
        ]
    } else {
        vec![MenuItem::new("Open Image"), MenuItem::new("Open Folder")]
    };
    let item_h = 30.0;
    let divider_h = 10.0;
    let menu_w = 180.0;
    let total_h: f64 = items
        .iter()
        .map(|i| if i.is_divider { divider_h } else { item_h })
        .sum();

    // Backdrop - semi-transparent overlay covering everything (clicking dismisses)
    let backdrop = Rect::from_origin_size((0.0, 0.0), size);
    painter.fill_rect(backdrop, Color::rgba8(0, 0, 0, 60));

    // Register backdrop as dismiss action
    state.register_click(backdrop, ClickAction::DismissAll);

    // Menu background
    let menu_rect = Rect::new(menu_x, menu_y, menu_x + menu_w, menu_y + total_h);
    painter.fill_rounded_rect(menu_rect, PANEL_BG, 6.0);

    // Shadow effect - darker border
    painter.stroke_rounded_rect(menu_rect, Color::rgba8(0, 0, 0, 80), 2.0, 6.0);

    // Clip to menu bounds
    painter.push_clip(menu_rect);

    let mut y = menu_y;

    for item in &items {
        if item.is_divider {
            let div_y = y + divider_h / 2.0;
            painter.draw_line(
                Point::new(menu_x + 8.0, div_y),
                Point::new(menu_x + menu_w - 8.0, div_y),
                BORDER_COLOR,
                1.0,
            );
            y += divider_h;
        } else {
            let item_rect = Rect::new(menu_x, y, menu_x + menu_w, y + item_h);
            let text_color = if item.is_danger {
                ERROR_COLOR
            } else {
                TEXT_PRIMARY
            };
            painter.draw_text_cached(
                text_cache,
                &item.label,
                menu_x + 14.0,
                y + item_h / 2.0 + 4.0,
                text_color,
                13.0,
                FontWeight::NORMAL,
                false,
                false,
            );

            // Register click region for each menu item
            state.register_click(
                item_rect,
                ClickAction::ContextMenuAction(item.label.clone()),
            );

            y += item_h;
        }
    }

    painter.pop_clip();
}

struct MenuItem {
    label: String,
    is_divider: bool,
    is_danger: bool,
}

impl MenuItem {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            is_divider: false,
            is_danger: false,
        }
    }

    fn danger(label: &str) -> Self {
        Self {
            label: label.to_string(),
            is_divider: false,
            is_danger: true,
        }
    }

    fn divider() -> Self {
        Self {
            label: String::new(),
            is_divider: true,
            is_danger: false,
        }
    }
}
