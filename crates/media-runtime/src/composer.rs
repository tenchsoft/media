use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tench_shared_types::PlatformAction;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub path: String,
    pub name: String,
    pub media_type: String,
    pub duration: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectData {
    pub name: String,
    pub timeline_duration: f64,
    pub tracks: Vec<TrackData>,
    pub clips: Vec<ClipData>,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackData {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub locked: bool,
    pub visible: bool,
    pub muted: bool,
    pub solo: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClipData {
    pub id: String,
    pub track_id: String,
    pub name: String,
    pub start: f64,
    pub duration: f64,
    pub tone: String,
    pub source_path: Option<String>,
}

pub fn new_project(name: String) -> ProjectData {
    let now = current_unix_timestamp();
    ProjectData {
        name,
        timeline_duration: 340.0,
        tracks: vec![
            track("v2", "video", "V2", false),
            track("v1", "video", "V1", false),
            track("a1", "audio", "A1", false),
            track("a2", "audio", "A2", false),
            track("sub", "subtitle", "SUB", true),
        ],
        clips: vec![],
        created_at: now.clone(),
        modified_at: now,
    }
}

pub fn save_project(
    projects_dir: impl AsRef<Path>,
    project: &ProjectData,
) -> Result<String, String> {
    let dir = ensure_project_dir(projects_dir)?;
    let filename = format!("{}.json", project.name);
    let path = dir.join(&filename);
    let json = serde_json::to_string_pretty(project).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

pub fn load_project(projects_dir: impl AsRef<Path>, name: &str) -> Result<ProjectData, String> {
    let dir = ensure_project_dir(projects_dir)?;
    let path = dir.join(format!("{name}.json"));
    let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

pub fn list_projects(projects_dir: impl AsRef<Path>) -> Result<Vec<String>, String> {
    let dir = ensure_project_dir(projects_dir)?;
    let mut names = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(".json") {
                names.push(name.trim_end_matches(".json").to_string());
            }
        }
    }
    names.sort();
    Ok(names)
}

pub fn import_media_files(paths: &[String]) -> Result<Vec<MediaInfo>, String> {
    Ok(paths
        .iter()
        .filter_map(|path| media_info_for_path(path).ok())
        .collect())
}

pub fn scan_media_folder(folder_path: impl AsRef<Path>) -> Result<Vec<MediaInfo>, String> {
    let entries = tench_media_core::scan_folder(
        folder_path,
        &tench_media_core::MediaScanOptions::all_media(),
    )
    .map_err(|e| e.message)?;
    Ok(entries
        .into_iter()
        .map(|entry| MediaInfo {
            path: entry.path,
            name: entry.file_name,
            media_type: media_kind_label(entry.kind).to_string(),
            duration: None,
            width: None,
            height: None,
            file_size: entry.size_bytes,
        })
        .collect())
}

pub fn generate_media_thumbnail(path: &str, max_size: u32) -> Result<String, String> {
    let media_type = media_type_from_path(path);
    let color = match media_type.as_str() {
        "video" => "#4f46e5",
        "audio" => "#16a34a",
        "image" => "#d97706",
        _ => "#6b7280",
    };
    let name = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("?");
    let short_name = name.chars().take(12).collect::<String>();

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}"><rect width="100%" height="100%" fill="{}"/><text x="50%" y="50%" fill="white" text-anchor="middle" dy=".3em" font-size="12">{}</text></svg>"#,
        max_size,
        (max_size as f64 * 0.5625) as u32,
        color,
        short_name
    );

    Ok(format!("data:image/svg+xml;base64,{}", base64_encode(&svg)))
}

pub fn show_in_file_manager(path: &str) -> Result<(), String> {
    let action = PlatformAction::RevealFile(path.to_string());
    let result = action.execute();
    if result.success {
        Ok(())
    } else {
        Err(result.message)
    }
}

pub fn save_project_json(path: impl AsRef<Path>, json: &str) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn media_file_size(path: impl AsRef<Path>) -> u64 {
    fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0)
}

fn ensure_project_dir(projects_dir: impl AsRef<Path>) -> Result<PathBuf, String> {
    let dir = projects_dir.as_ref().to_path_buf();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn media_info_for_path(path_str: &str) -> Result<MediaInfo, String> {
    let path = Path::new(path_str);
    if !path.exists() {
        return Err(format!("Media file does not exist: {path_str}"));
    }
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    let file_size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);

    Ok(MediaInfo {
        path: path_str.to_string(),
        name,
        media_type: media_type_from_path(path_str),
        duration: None,
        width: None,
        height: None,
        file_size,
    })
}

fn media_type_from_path(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    media_kind_label(tench_media_core::kind_for_extension(ext)).to_string()
}

fn media_kind_label(kind: tench_media_core::MediaKind) -> &'static str {
    match kind {
        tench_media_core::MediaKind::Video => "video",
        tench_media_core::MediaKind::Audio => "audio",
        tench_media_core::MediaKind::Image => "image",
        tench_media_core::MediaKind::Other => "unknown",
    }
}

fn track(id: &str, kind: &str, label: &str, locked: bool) -> TrackData {
    TrackData {
        id: id.into(),
        kind: kind.into(),
        label: label.into(),
        locked,
        visible: true,
        muted: false,
        solo: false,
    }
}

fn current_unix_timestamp() -> String {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}

fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_project_has_default_tracks() {
        let project = new_project("Cut".into());

        assert_eq!(project.name, "Cut");
        assert_eq!(project.tracks.len(), 5);
        assert_eq!(project.tracks[0].id, "v2");
        assert!(project.tracks[4].locked);
    }

    #[test]
    fn project_roundtrips_through_disk() {
        let dir = std::env::temp_dir().join("tench_composer_runtime_project");
        let _ = fs::remove_dir_all(&dir);
        let project = new_project("Roundtrip".into());

        let saved_path = save_project(&dir, &project).unwrap();
        let restored = load_project(&dir, "Roundtrip").unwrap();
        let names = list_projects(&dir).unwrap();

        assert!(saved_path.ends_with("Roundtrip.json"));
        assert_eq!(restored.name, "Roundtrip");
        assert_eq!(names, vec!["Roundtrip".to_string()]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn import_media_files_classifies_existing_files() {
        let dir = std::env::temp_dir().join("tench_composer_runtime_media");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("clip.mp4");
        fs::write(&path, b"fake").unwrap();

        let imported = import_media_files(&[path.to_string_lossy().to_string()]).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].media_type, "video");
        assert_eq!(imported[0].file_size, 4);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn placeholder_thumbnail_is_svg_data_url() {
        let thumbnail = generate_media_thumbnail("/tmp/clip.mp4", 160).unwrap();

        assert!(thumbnail.starts_with("data:image/svg+xml;base64,"));
    }
}
