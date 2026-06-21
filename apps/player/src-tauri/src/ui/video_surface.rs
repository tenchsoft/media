use tench_ui::prelude::Rect;

pub fn side_panel_width(ai_panel_open: bool, drawer_open: bool, panel_w: f64) -> f64 {
    if ai_panel_open || drawer_open {
        panel_w
    } else {
        0.0
    }
}

pub fn video_right(window_width: f64, ai_panel_open: bool, drawer_open: bool, panel_w: f64) -> f64 {
    window_width - side_panel_width(ai_panel_open, drawer_open, panel_w)
}

pub fn video_rect(
    window_width: f64,
    window_height: f64,
    overlay_h: f64,
    controls_h: f64,
    ai_panel_open: bool,
    drawer_open: bool,
    panel_w: f64,
) -> Rect {
    Rect::new(
        0.0,
        overlay_h,
        video_right(window_width, ai_panel_open, drawer_open, panel_w),
        window_height - controls_h,
    )
}
