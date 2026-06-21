use tench_ui::prelude::Rect;

pub fn panel_rect(video_right: f64, window_width: f64, window_height: f64) -> Rect {
    Rect::new(video_right, 0.0, window_width, window_height)
}

pub fn feature_prompts() -> [&'static str; 4] {
    [
        "Summarize current scene",
        "Find similar frames",
        "Generate chapter marks",
        "Explain dialogue",
    ]
}
