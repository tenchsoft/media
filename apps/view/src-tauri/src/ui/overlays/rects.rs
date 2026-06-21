use super::*;

// ---------------------------------------------------------------------------
// Rect computation helpers (automation fallback)
// ---------------------------------------------------------------------------

/// Computes URL dialog button rects without rendering.
pub fn url_dialog_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_url_dialog {
        return Vec::new();
    }
    let dialog_w = 420.0_f64.min(size.width - 40.0);
    let dialog_h = 160.0;
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let dialog_x = cx - dialog_w / 2.0;
    let dialog_y = cy - dialog_h / 2.0;
    let pad = 20.0;

    let mut y = dialog_y + pad + 30.0;

    // Load button
    let load_x = dialog_x + dialog_w - pad - 70.0;
    let mut rects = vec![(
        ClickAction::LoadFromUrl,
        Rect::new(load_x, y, load_x + 70.0, y + 28.0),
    )];

    y += 40.0;

    // Cancel
    let cancel_w = 60.0;
    let cancel_x = cx - cancel_w / 2.0;
    rects.push((
        ClickAction::UrlCancel,
        Rect::new(cancel_x, y, cancel_x + cancel_w, y + 24.0),
    ));

    rects
}

/// Computes print dialog button rects without rendering.
pub fn print_dialog_button_rects(state: &ViewState, size: Size) -> Vec<(ClickAction, Rect)> {
    if !state.show_print_dialog {
        return Vec::new();
    }
    let dialog_w = 380.0_f64.min(size.width - 40.0);
    let dialog_h = 320.0;
    let cx = size.width / 2.0;
    let dialog_x = cx - dialog_w / 2.0;
    let dialog_y = size.height / 2.0 - dialog_h / 2.0;
    let dialog_rect = Rect::new(dialog_x, dialog_y, dialog_x + dialog_w, dialog_y + dialog_h);
    let pad = 20.0;
    let mut rects = Vec::new();

    let mut y = dialog_y + pad + 36.0;
    let paper_sizes = ["A4", "A3", "Letter", "Legal", "4x6\"", "5x7\""];
    let mut px = dialog_rect.x0 + pad;
    for paper in &paper_sizes {
        let btn_w = paper.len() as f64 * 6.0 + 22.0;
        rects.push((
            ClickAction::PrintSelectPaper((*paper).to_string()),
            Rect::new(px, y, px + btn_w, y + 22.0),
        ));
        px += btn_w + 6.0;
    }
    y += 34.0 + 22.0;

    let orientations = ["Portrait", "Landscape"];
    let mut ox = dialog_rect.x0 + pad;
    for orient in &orientations {
        let btn_w = orient.len() as f64 * 6.0 + 22.0;
        rects.push((
            ClickAction::PrintSelectOrientation((*orient).to_string()),
            Rect::new(ox, y, ox + btn_w, y + 22.0),
        ));
        ox += btn_w + 6.0;
    }
    y += 34.0 + 22.0;

    let scales = ["Fit to page", "Fill page", "Actual size", "50%", "25%"];
    let mut sx = dialog_rect.x0 + pad;
    for scale in &scales {
        let btn_w = scale.len() as f64 * 6.0 + 22.0;
        if sx + btn_w > dialog_rect.x1 - pad {
            sx = dialog_rect.x0 + pad;
            y += 26.0;
        }
        rects.push((
            ClickAction::PrintSelectScaling((*scale).to_string()),
            Rect::new(sx, y, sx + btn_w, y + 22.0),
        ));
        sx += btn_w + 6.0;
    }
    y += 40.0;

    // Print button
    let print_w = 80.0;
    let print_x = cx - print_w / 2.0 - 50.0;
    rects.push((
        ClickAction::PrintImage,
        Rect::new(print_x, y, print_x + print_w, y + 28.0),
    ));

    // Cancel
    let cancel_w = 70.0;
    let cancel_x = cx + 20.0;
    rects.push((
        ClickAction::PrintCancel,
        Rect::new(cancel_x, y, cancel_x + cancel_w, y + 28.0),
    ));

    rects
}
