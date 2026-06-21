//! Subtitle parsing for SRT, VTT, and ASS/SSA formats.
//!
//! Provides a unified interface for parsing subtitle files into a common
//! `SubtitleCue` representation with timing information.

use std::fs;
use std::path::Path;

/// A single subtitle cue with timing information.
#[derive(Debug, Clone)]
pub struct SubtitleCue {
    pub id: String,
    pub start: f64,
    pub end: f64,
    pub text: String,
}

/// Parse an SRT subtitle file.
pub fn parse_srt(path: &str) -> Result<Vec<SubtitleCue>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let normalized = content.replace("\r\n", "\n");
    let mut cues = Vec::new();
    let blocks: Vec<&str> = normalized.split("\n\n").collect();

    for (idx, block) in blocks.iter().enumerate() {
        let lines: Vec<&str> = block.trim().lines().collect();
        if lines.len() < 3 {
            continue;
        }
        let ts_line = lines[1];
        let parts: Vec<&str> = ts_line.split(" --> ").collect();
        if parts.len() != 2 {
            continue;
        }
        let start = parse_srt_timestamp(parts[0].trim());
        let end = parse_srt_timestamp(parts[1].trim());
        let text = lines[2..].join("\n");

        cues.push(SubtitleCue {
            id: format!("srt-{}", idx),
            start,
            end,
            text,
        });
    }
    Ok(cues)
}

/// Parse a VTT subtitle file.
pub fn parse_vtt(path: &str) -> Result<Vec<SubtitleCue>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let normalized = content.replace("\r\n", "\n");
    let mut cues = Vec::new();
    let blocks: Vec<&str> = normalized.split("\n\n").collect();

    for (idx, block) in blocks.iter().enumerate() {
        let lines: Vec<&str> = block.trim().lines().collect();
        if lines.is_empty() {
            continue;
        }
        let ts_idx = match lines.iter().position(|l| l.contains("-->")) {
            Some(i) => i,
            None => continue,
        };
        let ts_line = lines[ts_idx];
        let parts: Vec<&str> = ts_line.split("-->").collect();
        if parts.len() != 2 {
            continue;
        }
        let start = parse_vtt_timestamp(parts[0].trim());
        let end = parse_vtt_timestamp(parts[1].trim());
        let text = lines[ts_idx + 1..].join("\n");

        cues.push(SubtitleCue {
            id: format!("vtt-{}", idx),
            start,
            end,
            text,
        });
    }
    Ok(cues)
}

/// Parse an ASS/SSA subtitle file.
pub fn parse_ass(path: &str) -> Result<Vec<SubtitleCue>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut cues = Vec::new();
    let mut idx = 0;

    for line in content.lines() {
        let line = line.trim();
        if !line.starts_with("Dialogue:") {
            continue;
        }
        let dialogue = line.strip_prefix("Dialogue:").unwrap_or("");
        let parts: Vec<&str> = dialogue.splitn(10, ',').collect();
        if parts.len() < 10 {
            continue;
        }
        let start = parse_ass_timestamp(parts[1].trim());
        let end = parse_ass_timestamp(parts[2].trim());
        let text = parts[9].replace("\\N", "\n").replace("\\n", "\n");

        cues.push(SubtitleCue {
            id: format!("ass-{}", idx),
            start,
            end,
            text,
        });
        idx += 1;
    }
    Ok(cues)
}

/// Auto-detect subtitle format and parse.
pub fn parse_subtitle_file(path: &str) -> Result<Vec<SubtitleCue>, String> {
    let ext = Path::new(path)
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "srt" => parse_srt(path),
        "vtt" => parse_vtt(path),
        "ass" | "ssa" => parse_ass(path),
        _ => Err(format!("Unsupported subtitle format: {}", ext)),
    }
}

// ── Timestamp parsers ──

fn parse_srt_timestamp(ts: &str) -> f64 {
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() != 3 {
        return 0.0;
    }
    let h: f64 = parts[0].parse().unwrap_or(0.0);
    let m: f64 = parts[1].parse().unwrap_or(0.0);
    let sec_parts: Vec<&str> = parts[2].split(',').collect();
    let s: f64 = sec_parts[0].parse().unwrap_or(0.0);
    let ms: f64 = sec_parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.0);
    h * 3600.0 + m * 60.0 + s + ms / 1000.0
}

fn parse_vtt_timestamp(ts: &str) -> f64 {
    let ts = ts.trim();
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() == 3 {
        let h: f64 = parts[0].parse().unwrap_or(0.0);
        let m: f64 = parts[1].parse().unwrap_or(0.0);
        let sec_parts: Vec<&str> = parts[2].split('.').collect();
        let s: f64 = sec_parts[0].parse().unwrap_or(0.0);
        let ms: f64 = sec_parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.0);
        h * 3600.0 + m * 60.0 + s + ms / 1000.0
    } else if parts.len() == 2 {
        let m: f64 = parts[0].parse().unwrap_or(0.0);
        let sec_parts: Vec<&str> = parts[1].split('.').collect();
        let s: f64 = sec_parts[0].parse().unwrap_or(0.0);
        let ms: f64 = sec_parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.0);
        m * 60.0 + s + ms / 1000.0
    } else {
        0.0
    }
}

fn parse_ass_timestamp(ts: &str) -> f64 {
    let ts = ts.trim();
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() != 3 {
        return 0.0;
    }
    let h: f64 = parts[0].parse().unwrap_or(0.0);
    let m: f64 = parts[1].parse().unwrap_or(0.0);
    let sec_parts: Vec<&str> = parts[2].split('.').collect();
    let s: f64 = sec_parts[0].parse().unwrap_or(0.0);
    let cs: f64 = sec_parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.0);
    h * 3600.0 + m * 60.0 + s + cs / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_srt_timestamp_works() {
        assert_eq!(parse_srt_timestamp("01:23:45,678"), 5025.678);
        assert_eq!(parse_srt_timestamp("00:00:00,000"), 0.0);
        assert_eq!(parse_srt_timestamp("00:01:30,500"), 90.5);
    }

    #[test]
    fn parse_vtt_timestamp_works() {
        assert_eq!(parse_vtt_timestamp("01:23:45.678"), 5025.678);
        assert_eq!(parse_vtt_timestamp("01:30.500"), 90.5);
    }

    #[test]
    fn parse_ass_timestamp_works() {
        assert_eq!(parse_ass_timestamp("1:23:45.67"), 5025.67);
    }

    #[test]
    fn parse_srt_file() {
        let dir = std::env::temp_dir().join("tench_subtitle_test");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("test.srt");
        fs::write(&path, "1\n00:00:01,000 --> 00:00:04,000\nHello World\n\n2\n00:00:05,000 --> 00:00:08,000\nSecond line\n").ok();
        let cues = parse_srt(&path.to_string_lossy()).unwrap();
        assert_eq!(cues.len(), 2);
        assert_eq!(cues[0].start, 1.0);
        assert_eq!(cues[0].end, 4.0);
        assert_eq!(cues[0].text, "Hello World");
        assert_eq!(cues[1].text, "Second line");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_vtt_file() {
        let dir = std::env::temp_dir().join("tench_subtitle_test_vtt");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("test.vtt");
        fs::write(
            &path,
            "WEBVTT\n\n00:00:01.000 --> 00:00:04.000\nHello VTT\n",
        )
        .ok();
        let cues = parse_vtt(&path.to_string_lossy()).unwrap();
        assert_eq!(cues.len(), 1);
        assert_eq!(cues[0].text, "Hello VTT");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_ass_file() {
        let dir = std::env::temp_dir().join("tench_subtitle_test_ass");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("test.ass");
        fs::write(&path, "[Script Info]\nScriptType: v4.00+\n\n[Events]\nFormat: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\nDialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,Hello ASS\n").ok();
        let cues = parse_ass(&path.to_string_lossy()).unwrap();
        assert_eq!(cues.len(), 1);
        assert_eq!(cues[0].text, "Hello ASS");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn unsupported_format() {
        let result = parse_subtitle_file("test.txt");
        assert!(result.is_err());
    }
}
