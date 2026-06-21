use super::*;

/// Computes navigation edge button rects without rendering.
/// Returns a list of (ClickAction, Rect) for prev/next arrows.
pub fn nav_edge_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    let mut rects = Vec::new();
    if state.sorted_entries.is_empty() {
        return rects;
    }

    let idx = state.selected_index();
    let has_prev = idx.is_some_and(|i| i > 0);
    let has_next = idx.is_some_and(|i| i + 1 < state.sorted_entries.len());

    let nav_w = 48.0;
    let nav_h = 64.0;
    let nav_y = (size.height - nav_h) / 2.0;
    let edge_margin = 18.0;

    if has_prev {
        rects.push((
            ClickAction::NavigatePrev,
            Rect::new(edge_margin, nav_y, edge_margin + nav_w, nav_y + nav_h),
        ));
    }
    if has_next {
        let next_x = size.width - edge_margin - nav_w;
        rects.push((
            ClickAction::NavigateNext,
            Rect::new(next_x, nav_y, next_x + nav_w, nav_y + nav_h),
        ));
    }
    rects
}

/// Draws the navigation edge arrows.
/// Matches `.nav-edge` CSS.
pub fn paint_nav_edges(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    if state.sorted_entries.is_empty() {
        return;
    }

    let mut painter = Painter::new(scene);

    let idx = state.selected_index();
    let has_prev = idx.is_some_and(|i| i > 0);
    let has_next = idx.is_some_and(|i| i + 1 < state.sorted_entries.len());

    let nav_w = 48.0;
    let nav_h = 64.0;
    let nav_y = (size.height - nav_h) / 2.0;
    let edge_margin = 18.0;

    // Previous button (left edge)
    if has_prev {
        let prev_rect = Rect::new(edge_margin, nav_y, edge_margin + nav_w, nav_y + nav_h);
        painter.fill_rounded_rect(prev_rect, Color::rgba8(0x0F, 0x0F, 0x0F, 180), 8.0);
        painter.stroke_rounded_rect(prev_rect, BORDER_COLOR, 1.0, 8.0);
        painter.draw_text_cached(
            text_cache,
            "\u{25C0}",
            edge_margin + (nav_w - 12.0) / 2.0,
            nav_y + nav_h / 2.0 + 5.0,
            TEXT_PRIMARY,
            18.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Register click region
        state.register_click(prev_rect, ClickAction::NavigatePrev);
    }

    // Next button (right edge)
    if has_next {
        let next_x = size.width - edge_margin - nav_w;
        let next_rect = Rect::new(next_x, nav_y, next_x + nav_w, nav_y + nav_h);
        painter.fill_rounded_rect(next_rect, Color::rgba8(0x0F, 0x0F, 0x0F, 180), 8.0);
        painter.stroke_rounded_rect(next_rect, BORDER_COLOR, 1.0, 8.0);
        painter.draw_text_cached(
            text_cache,
            "\u{25B6}",
            next_x + (nav_w - 12.0) / 2.0,
            nav_y + nav_h / 2.0 + 5.0,
            TEXT_PRIMARY,
            18.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Register click region
        state.register_click(next_rect, ClickAction::NavigateNext);
    }

    // Breadcrumb edge (left center)
    if let Some(i) = idx {
        let bc_w = 180.0;
        let bc_h = 48.0;
        let bc_x = 12.0;
        let bc_y = (size.height - bc_h) / 2.0 - nav_h - 10.0;

        let bc_rect = Rect::new(bc_x, bc_y, bc_x + bc_w, bc_y + bc_h);
        painter.fill_rounded_rect(bc_rect, Color::rgba8(0x0F, 0x0F, 0x0F, 200), 6.0);
        painter.stroke_rounded_rect(bc_rect, BORDER_COLOR, 1.0, 6.0);

        // Clip breadcrumb content
        painter.push_clip(bc_rect);

        // Folder label
        let folder = state
            .document
            .as_ref()
            .map(|d| d.path.clone())
            .unwrap_or_default();
        let short_folder: String = folder
            .split('/')
            .filter(|s| !s.is_empty())
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join(" / ");
        painter.draw_text_cached(
            text_cache,
            &short_folder,
            bc_x + 8.0,
            bc_y + 16.0,
            TEXT_SECONDARY,
            11.0,
            FontWeight::NORMAL,
            false,
            false,
        );

        // Index label
        let idx_label = format!("{} / {}", i + 1, state.sorted_entries.len());
        painter.draw_text_cached(
            text_cache,
            &idx_label,
            bc_x + 8.0,
            bc_y + 34.0,
            TEXT_PRIMARY,
            13.0,
            FontWeight::MEDIUM,
            false,
            false,
        );

        painter.pop_clip();
    }
}
