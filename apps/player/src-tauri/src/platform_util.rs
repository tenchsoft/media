//! Platform-specific utility operations.
//!
//! Provides clipboard, file manager, and local persistence utilities
//! that can be used by both the Widget (direct call) and Tauri commands (IPC wrapper).
//!
//! All external process calls go through the centralized PlatformAction enum
//! from tench-shared-types to ensure path validation and no shell injection.

use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};
use tench_shared_types::PlatformAction;

/// Per-file persisted data.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileRecord {
    /// Last playback position in seconds.
    pub position: f64,
    /// Unix timestamp of last access.
    pub last_opened: u64,
}

/// Recently opened file entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentFileEntry {
    pub path: String,
    pub title: String,
    pub duration: f64,
    pub last_opened: u64,
}

/// Persistent state stored as JSON in the app data directory.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PersistentState {
    /// File path → last position + access time.
    pub positions: HashMap<String, FileRecord>,
    /// Recently opened files (newest first).
    pub recent_files: Vec<RecentFileEntry>,
}

impl PersistentState {
    /// Get the path to the persistent state file.
    fn state_path() -> std::path::PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let dir = std::path::PathBuf::from(home)
            .join(".config")
            .join("tench-player");
        let _ = fs::create_dir_all(&dir);
        dir.join("state.json")
    }

    /// Load persistent state from disk.
    pub fn load() -> Self {
        let path = Self::state_path();
        match fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save persistent state to disk.
    pub fn save(&self) {
        let path = Self::state_path();
        if let Ok(data) = serde_json::to_string(self) {
            let _ = fs::write(path, data);
        }
    }

    /// Save the playback position for a file.
    pub fn save_position(file_path: &str, position: f64) {
        let mut state = Self::load();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        state.positions.insert(
            file_path.to_string(),
            FileRecord {
                position,
                last_opened: now,
            },
        );
        // Keep only last 500 entries
        if state.positions.len() > 500 {
            let mut entries: Vec<_> = state.positions.iter().collect();
            entries.sort_by_key(|(_, r)| r.last_opened);
            let to_remove: Vec<String> = entries
                .iter()
                .take(entries.len() - 500)
                .map(|(k, _)| (*k).clone())
                .collect();
            for k in to_remove {
                state.positions.remove(&k);
            }
        }
        state.save();
    }

    /// Load the saved position for a file.
    pub fn load_position(file_path: &str) -> Option<f64> {
        let state = Self::load();
        state.positions.get(file_path).map(|r| r.position)
    }

    /// Add a file to recent files list.
    pub fn add_recent(path: &str, title: &str, duration: f64) {
        let mut state = Self::load();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Remove existing entry for same path
        state.recent_files.retain(|e| e.path != path);

        state.recent_files.insert(
            0,
            RecentFileEntry {
                path: path.to_string(),
                title: title.to_string(),
                duration,
                last_opened: now,
            },
        );

        // Keep only last 50 entries
        state.recent_files.truncate(50);
        state.save();
    }

    /// Get recent files list.
    pub fn recent_files() -> Vec<RecentFileEntry> {
        let state = Self::load();
        state.recent_files
    }
}

/// Opens the system file manager and selects the given file path.
///
/// Uses the centralized PlatformAction abstraction for safe process spawning.
pub fn show_in_file_manager(path: &str) -> Result<(), String> {
    let action = PlatformAction::RevealFile(path.to_string());
    let result = action.execute();
    if result.success {
        Ok(())
    } else {
        Err(result.message)
    }
}

/// Copies text to the system clipboard.
pub fn copy_to_clipboard_text(text: &str) -> Result<(), String> {
    let mut clipboard =
        arboard::Clipboard::new().map_err(|e| format!("Failed to access clipboard: {e}"))?;
    clipboard
        .set_text(text)
        .map_err(|e| format!("Failed to copy text to clipboard: {e}"))?;
    Ok(())
}
