pub const CONTROLS_HEIGHT: f64 = 64.0;

pub fn format_time(current: f64, duration: f64) -> String {
    let fmt = |secs: f64| {
        let s = secs as u32;
        format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
    };
    format!("{} / {}", fmt(current), fmt(duration))
}

pub fn seek_ratio(x: f64, margin: f64, video_right: f64) -> f64 {
    ((x - margin) / (video_right - margin * 2.0)).clamp(0.0, 1.0)
}

pub fn volume_ratio(x: f64, left: f64, width: f64) -> f64 {
    ((x - left) / width.max(1.0)).clamp(0.0, 1.0)
}

pub fn ab_loop_label(ab_loop: Option<(f64, f64)>) -> Option<String> {
    ab_loop.map(|(a, b)| format!("A-B {} - {}", format_single_time(a), format_single_time(b)))
}

pub fn format_single_time(secs: f64) -> String {
    let s = secs as u32;
    let h = s / 3600;
    let m = (s / 60) % 60;
    let sec = s % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, sec)
    } else {
        format!("{:02}:{:02}", m, sec)
    }
}

pub fn speed_options() -> [f64; 10] {
    [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0]
}
