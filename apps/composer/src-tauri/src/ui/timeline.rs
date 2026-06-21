//! Timeline and clip lane geometry for Composer.

use tench_ui::prelude::{Point, Rect};

pub const HEADER_H: f64 = 28.0;
pub const TOOLBAR_H: f64 = 28.0;
/// Width of the trim handle hit area on each side of a clip.
pub const TRIM_HANDLE_W: f64 = 7.0;

pub fn timeline_y(window_height: f64, timeline_h: f64) -> f64 {
    window_height - timeline_h
}

pub fn content_width(window_width: f64, left_w: f64) -> f64 {
    (window_width - left_w).max(1.0)
}

pub fn track_height(timeline_h: f64, track_count: usize) -> f64 {
    let lanes_h = (timeline_h - HEADER_H - TOOLBAR_H).max(1.0);
    if track_count == 0 {
        lanes_h
    } else {
        lanes_h / track_count as f64
    }
}

pub fn frame_to_x(left_w: f64, width: f64, frame: u32, total_frames: u32) -> f64 {
    let ratio = if total_frames == 0 {
        0.0
    } else {
        frame as f64 / total_frames as f64
    };
    left_w + width * ratio.clamp(0.0, 1.0)
}

pub fn x_to_frame(x: f64, left_w: f64, width: f64, total_frames: u32) -> u32 {
    let ratio = ((x - left_w) / width.max(1.0)).clamp(0.0, 1.0);
    (total_frames as f64 * ratio).round() as u32
}

pub fn clip_rect(
    left_w: f64,
    content_w: f64,
    total_frames: u32,
    track_y: f64,
    track_h: f64,
    start_frame: u32,
    end_frame: u32,
) -> Rect {
    let start_x = frame_to_x(left_w, content_w, start_frame, total_frames);
    let end_x = frame_to_x(left_w, content_w, end_frame, total_frames);
    Rect::new(
        start_x + 1.0,
        track_y + 2.0,
        end_x - 2.0,
        track_y + track_h - 4.0,
    )
}

pub fn hit_test_track(
    point: Point,
    tl_y: f64,
    timeline_h: f64,
    track_count: usize,
) -> Option<usize> {
    let lanes_y = tl_y + HEADER_H + TOOLBAR_H;
    if point.y < lanes_y || point.y > tl_y + timeline_h {
        return None;
    }
    let h = track_height(timeline_h, track_count);
    let idx = ((point.y - lanes_y) / h) as usize;
    (idx < track_count).then_some(idx)
}

/// Determine if a point is within the trim handle zone of a clip rect.
/// Returns `Some(true)` for left (in) handle, `Some(false)` for right (out) handle.
pub fn hit_test_trim_handle(point: Point, clip_rect: Rect) -> Option<bool> {
    if !clip_rect.contains(point) {
        return None;
    }
    if point.x - clip_rect.x0 < TRIM_HANDLE_W {
        return Some(true); // left = trim in
    }
    if clip_rect.x1 - point.x < TRIM_HANDLE_W {
        return Some(false); // right = trim out
    }
    None
}

pub fn kind_badge(kind: &str) -> &'static str {
    match kind {
        "video" => "V",
        "audio" => "A",
        "subtitle" => "CC",
        _ => "-",
    }
}

/// Compute which frame a timeline x-coordinate maps to, accounting for zoom.
pub fn x_to_frame_zoomed(x: f64, left_w: f64, content_w: f64, total_frames: u32, zoom: f64) -> u32 {
    let zoomed_content = content_w * (zoom / 100.0);
    let ratio = ((x - left_w) / zoomed_content.max(1.0)).clamp(0.0, 1.0);
    (total_frames as f64 * ratio).round() as u32
}

/// Compute the x position for a frame with zoom applied.
pub fn frame_to_x_zoomed(
    left_w: f64,
    content_w: f64,
    frame: u32,
    total_frames: u32,
    zoom: f64,
) -> f64 {
    let ratio = if total_frames == 0 {
        0.0
    } else {
        frame as f64 / total_frames as f64
    };
    let zoomed_content = content_w * (zoom / 100.0);
    left_w + zoomed_content * ratio.clamp(0.0, 1.0)
}
