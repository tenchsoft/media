//! Preview monitor geometry and formatting for Composer.

use tench_ui::prelude::{Point, Rect};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub fn monitor_rect(area: Rect) -> Rect {
    let width = area.x1 - area.x0;
    let height = area.y1 - area.y0;
    let (preview_w, preview_h) = if width / height > ASPECT_RATIO {
        (height * ASPECT_RATIO, height)
    } else {
        (width, width / ASPECT_RATIO)
    };
    let x = area.x0 + (width - preview_w) / 2.0;
    let y = area.y0 + (height - preview_h) / 2.0;
    Rect::new(x, y, x + preview_w, y + preview_h)
}

/// Format timecode as HH:MM:SS:FF (industry standard NLE format).
pub fn format_timecode(frame: u32, fps: f64) -> String {
    let total_seconds = frame as f64 / fps;
    let hours = (total_seconds / 3600.0) as u32;
    let minutes = ((total_seconds % 3600.0) / 60.0) as u32;
    let seconds = (total_seconds % 60.0) as u32;
    let frames = (frame as f64 % fps) as u32;
    format!("{:02}:{:02}:{:02}:{:02}", hours, minutes, seconds, frames)
}

pub fn monitor_playhead_x(rect: Rect, current_frame: u32, total_frames: u32) -> f64 {
    let ratio = if total_frames == 0 {
        0.0
    } else {
        current_frame as f64 / total_frames as f64
    };
    rect.x0 + 10.0 + ((rect.x1 - rect.x0) - 20.0) * ratio.clamp(0.0, 1.0)
}

pub fn hit_test_preview(point: Point, left: f64, right: f64, top: f64, bottom: f64) -> bool {
    point.x >= left && point.x < right && point.y >= top && point.y < bottom
}

/// Format a shuttle speed label for the preview overlay.
pub fn shuttle_label(direction: i32, speed: f64) -> String {
    if direction == 0 {
        return "Paused".into();
    }
    let arrow = if direction > 0 { ">>" } else { "<<" };
    if speed == 1.0 {
        arrow.to_string()
    } else {
        format!("{} {:.0}x", arrow, speed)
    }
}
