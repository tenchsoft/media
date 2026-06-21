use super::*;

// Top Overlay

/// Button layout spec used for both painting and automation.
struct BtnSpec {
    label: &'static str,
    action: Option<ClickAction>,
}

/// Returns the top overlay button specs for the current state.
fn top_btn_specs(state: &ViewState) -> Vec<BtnSpec> {
    let has_image = state.document.is_some();
    if has_image {
        vec![
            BtnSpec {
                label: "Open",
                action: Some(ClickAction::OpenFileDialog),
            },
            BtnSpec {
                label: "Folder",
                action: Some(ClickAction::OpenFolderDialog),
            },
            BtnSpec {
                label: "Archive",
                action: Some(ClickAction::OpenArchiveDialog),
            },
            BtnSpec {
                label: "Info",
                action: Some(ClickAction::ToggleFileInfo),
            },
            BtnSpec {
                label: "Edit",
                action: Some(ClickAction::ToggleQuickEdit),
            },
            BtnSpec {
                label: "|",
                action: None,
            },
            BtnSpec {
                label: "sort_key",
                action: Some(ClickAction::SortByKey),
            },
            BtnSpec {
                label: "sort_order",
                action: Some(ClickAction::ToggleSortOrder),
            },
            BtnSpec {
                label: "|",
                action: None,
            },
            BtnSpec {
                label: "Files",
                action: Some(ClickAction::ToggleMetadata),
            },
            BtnSpec {
                label: "Copy Path",
                action: Some(ClickAction::CopyPath),
            },
            BtnSpec {
                label: "Copy Img",
                action: Some(ClickAction::CopyImage),
            },
            BtnSpec {
                label: "URL",
                action: Some(ClickAction::OpenFromUrl),
            },
            BtnSpec {
                label: "Bookmark",
                action: Some(ClickAction::ToggleBookmark),
            },
            BtnSpec {
                label: "Search",
                action: Some(ClickAction::ToggleSearch),
            },
        ]
    } else {
        vec![
            BtnSpec {
                label: "Open",
                action: Some(ClickAction::OpenFileDialog),
            },
            BtnSpec {
                label: "Folder",
                action: Some(ClickAction::OpenFolderDialog),
            },
            BtnSpec {
                label: "Archive",
                action: Some(ClickAction::OpenArchiveDialog),
            },
            BtnSpec {
                label: "URL",
                action: Some(ClickAction::OpenFromUrl),
            },
            BtnSpec {
                label: "Info",
                action: Some(ClickAction::ToggleFileInfo),
            },
            BtnSpec {
                label: "Edit",
                action: Some(ClickAction::ToggleQuickEdit),
            },
            BtnSpec {
                label: "|",
                action: None,
            },
            BtnSpec {
                label: "sort_key",
                action: Some(ClickAction::SortByKey),
            },
            BtnSpec {
                label: "sort_order",
                action: Some(ClickAction::ToggleSortOrder),
            },
            BtnSpec {
                label: "Bookmark",
                action: Some(ClickAction::ToggleBookmark),
            },
            BtnSpec {
                label: "Search",
                action: Some(ClickAction::ToggleSearch),
            },
        ]
    }
}

/// Returns the bottom overlay button specs for the current state.
fn bottom_btn_specs(_state: &ViewState) -> Vec<BtnSpec> {
    vec![
        BtnSpec {
            label: "Fit",
            action: Some(ClickAction::ZoomFit),
        },
        BtnSpec {
            label: "100%",
            action: Some(ClickAction::ZoomActual),
        },
        BtnSpec {
            label: "-",
            action: Some(ClickAction::ZoomOut),
        },
        BtnSpec {
            label: "zoom",
            action: Some(ClickAction::ZoomFit),
        },
        BtnSpec {
            label: "+",
            action: Some(ClickAction::ZoomIn),
        },
        BtnSpec {
            label: "Film",
            action: Some(ClickAction::ToggleThumbnails),
        },
        BtnSpec {
            label: "Rotate",
            action: Some(ClickAction::Rotate),
        },
        BtnSpec {
            label: "bg",
            action: Some(ClickAction::CycleBgColor),
        },
        BtnSpec {
            label: "Full",
            action: Some(ClickAction::ToggleFullscreen),
        },
        BtnSpec {
            label: "Share",
            action: Some(ClickAction::ShareImage),
        },
        BtnSpec {
            label: "Wallpaper",
            action: Some(ClickAction::SetWallpaperAction),
        },
        BtnSpec {
            label: "Del",
            action: Some(ClickAction::DeleteFromToolbar),
        },
    ]
}

/// Computes button rects for top overlay buttons. Returns (action, rect) pairs.
/// This mirrors the exact layout logic in `paint_top_overlay`.
pub fn top_overlay_button_rects(state: &ViewState, width: f64) -> Vec<(ClickAction, Rect)> {
    let specs = top_btn_specs(state);
    let pad = 20.0;
    let btn_w = 68.0;
    let btn_h = 28.0;
    let btn_gap = 6.0;
    let divider_w = 8.0;

    let total_btn_w: f64 = specs
        .iter()
        .map(|s| if s.label == "|" { divider_w } else { btn_w })
        .sum::<f64>()
        + (specs.len().saturating_sub(1)) as f64 * btn_gap;

    let mut x = width - total_btn_w - pad;
    let y = 18.0;

    let mut result = Vec::new();
    for spec in &specs {
        if spec.label == "|" {
            x += divider_w + btn_gap;
        } else {
            let btn_rect = Rect::new(x, y, x + btn_w, y + btn_h);
            if let Some(action) = spec.action.clone() {
                result.push((action, btn_rect));
            }
            x += btn_w + btn_gap;
        }
    }
    result
}

/// Computes button rects for bottom overlay buttons. Returns (action, rect) pairs.
/// This mirrors the exact layout logic in `paint_bottom_overlay`.
pub fn bottom_overlay_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    let overlay_h = if state.show_thumbnails && !state.sorted_entries.is_empty() {
        120.0
    } else {
        60.0
    };
    let overlay_y = size.height - overlay_h;
    let ctrl_y = overlay_y + 10.0;
    let ctrl_h = 28.0;
    let btn_w = 52.0;
    let btn_gap = 6.0;

    let specs = bottom_btn_specs(state);
    let total_w = specs.len() as f64 * (btn_w + btn_gap);
    let mut x = (size.width - total_w) / 2.0;

    let mut result = Vec::new();
    for spec in &specs {
        let btn_rect = Rect::new(x, ctrl_y, x + btn_w, ctrl_y + ctrl_h);
        if let Some(action) = spec.action.clone() {
            result.push((action, btn_rect));
        }
        x += btn_w + btn_gap;
    }
    result
}

/// Draws the top overlay bar.
/// Matches `.top-overlay` CSS: gradient background, flex layout, action buttons.
pub fn paint_top_overlay(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let overlay_h = 80.0;
    let overlay_rect = Rect::new(0.0, 0.0, size.width, overlay_h);

    // Gradient background - matches `var(--overlay-gradient)`
    painter.fill_rect(overlay_rect, OVERLAY_BG);

    // Clip to overlay bounds
    painter.push_clip(overlay_rect);

    // File info (left side)
    let pad = 20.0;
    if let Some(ref doc) = state.document {
        // Filename
        painter.draw_text_cached(
            text_cache,
            &doc.file_name,
            pad,
            22.0,
            TEXT_PRIMARY,
            14.0,
            FontWeight::MEDIUM,
            false,
            false,
        );

        // Format and dimensions
        let mut info = doc.format.to_uppercase();
        if let Some(dims) = doc.dimensions {
            info.push_str(&format!(" \u{00B7} {} x {}", dims.width, dims.height));
        }
        painter.draw_text_cached(
            text_cache,
            &info,
            pad,
            42.0,
            TEXT_SECONDARY,
            12.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    } else {
        painter.draw_text_cached(
            text_cache,
            "Tench View",
            pad,
            22.0,
            TEXT_PRIMARY,
            14.0,
            FontWeight::MEDIUM,
            false,
            false,
        );
        painter.draw_text_cached(
            text_cache,
            &state.status_message,
            pad,
            42.0,
            TEXT_SECONDARY,
            12.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    }

    // Action buttons (right side)
    let has_image = state.document.is_some();

    // Build button list with their actions
    let btn_specs: Vec<(&str, Option<ClickAction>)> = if has_image {
        vec![
            ("Open", Some(ClickAction::OpenFileDialog)),
            ("Folder", Some(ClickAction::OpenFolderDialog)),
            ("Archive", Some(ClickAction::OpenArchiveDialog)),
            ("Info", Some(ClickAction::ToggleFileInfo)),
            ("Edit", Some(ClickAction::ToggleQuickEdit)),
            ("|", None),
            (state.sort_key.label(), Some(ClickAction::SortByKey)),
            (state.sort_order.arrow(), Some(ClickAction::ToggleSortOrder)),
            ("|", None),
            ("Files", Some(ClickAction::ToggleMetadata)),
            ("Copy Path", Some(ClickAction::CopyPath)),
            ("Copy Img", Some(ClickAction::CopyImage)),
            ("URL", Some(ClickAction::OpenFromUrl)),
            ("Bookmark", Some(ClickAction::ToggleBookmark)),
            ("Search", Some(ClickAction::ToggleSearch)),
        ]
    } else {
        vec![
            ("Open", Some(ClickAction::OpenFileDialog)),
            ("Folder", Some(ClickAction::OpenFolderDialog)),
            ("Archive", Some(ClickAction::OpenArchiveDialog)),
            ("URL", Some(ClickAction::OpenFromUrl)),
            ("Info", Some(ClickAction::ToggleFileInfo)),
            ("Edit", Some(ClickAction::ToggleQuickEdit)),
            ("|", None),
            (state.sort_key.label(), Some(ClickAction::SortByKey)),
            (state.sort_order.arrow(), Some(ClickAction::ToggleSortOrder)),
            ("Bookmark", Some(ClickAction::ToggleBookmark)),
            ("Search", Some(ClickAction::ToggleSearch)),
        ]
    };

    let btn_w = 68.0;
    let btn_h = 28.0;
    let btn_gap = 6.0;
    let divider_w = 8.0;

    // Calculate total width
    let total_btn_w: f64 = btn_specs
        .iter()
        .map(|(l, _)| if *l == "|" { divider_w } else { btn_w })
        .sum::<f64>()
        + (btn_specs.len().saturating_sub(1)) as f64 * btn_gap;

    let mut x = size.width - total_btn_w - pad;
    let y = 18.0;

    for (label, action) in &btn_specs {
        if *label == "|" {
            // Divider
            painter.draw_line(
                Point::new(x + divider_w / 2.0, y + 4.0),
                Point::new(x + divider_w / 2.0, y + btn_h - 4.0),
                BORDER_COLOR,
                1.0,
            );
            x += divider_w + btn_gap;
        } else {
            let btn_rect = Rect::new(x, y, x + btn_w, y + btn_h);
            painter.fill_rounded_rect(btn_rect, BTN_BG, 4.0);
            painter.stroke_rounded_rect(btn_rect, BORDER_COLOR, 1.0, 4.0);

            let tw = text_cache.measure_text_width(label, 11.0, FontWeight::MEDIUM);
            painter.draw_text_cached(
                text_cache,
                label,
                x + (btn_w - tw) / 2.0,
                y + btn_h / 2.0 + 4.0,
                TEXT_PRIMARY,
                11.0,
                FontWeight::MEDIUM,
                false,
                false,
            );

            // Register click region
            if let Some(action) = action.clone() {
                state.register_click(btn_rect, action);
            }

            x += btn_w + btn_gap;
        }
    }

    painter.pop_clip();

    // Hamburger menu button on the far left
    let menu_btn_size = 28.0;
    let menu_btn_x = 6.0;
    let menu_btn_y = 18.0;
    let menu_btn_rect = Rect::new(
        menu_btn_x,
        menu_btn_y,
        menu_btn_x + menu_btn_size,
        menu_btn_y + menu_btn_size,
    );
    painter.fill_rounded_rect(menu_btn_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(menu_btn_rect, BORDER_COLOR, 1.0, 4.0);
    // Draw three horizontal lines (hamburger icon)
    let line_x0 = menu_btn_x + 7.0;
    let line_x1 = menu_btn_x + menu_btn_size - 7.0;
    let line_y_center = menu_btn_y + menu_btn_size / 2.0;
    for offset in [-6.0, 0.0, 6.0] {
        painter.draw_line(
            Point::new(line_x0, line_y_center + offset),
            Point::new(line_x1, line_y_center + offset),
            TEXT_PRIMARY,
            1.5,
        );
    }
    state.register_click(menu_btn_rect, ClickAction::ToggleMenu);
}
