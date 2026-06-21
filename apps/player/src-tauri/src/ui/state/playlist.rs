use super::*;
use std::hash::BuildHasher;

impl PlayerState {
    // ── Playlist navigation ──

    pub fn next_track(&mut self) {
        if self.playlist.is_empty() {
            return;
        }

        if self.repeat_mode == RepeatMode::One {
            // Repeat one: seek to beginning instead of reloading
            self.current_time = 0.0;
            self.update_subtitle_for_position();
            return;
        }

        let idx = self.current_playlist_index.unwrap_or(0);

        if self.shuffle_enabled {
            // Pick a random track different from current
            if self.playlist.len() > 1 {
                use std::collections::hash_map::RandomState;
                let rng = RandomState::new();
                let hash = rng.hash_one(idx);
                let next = (hash as usize) % self.playlist.len();
                let next = if next == idx {
                    (idx + 1) % self.playlist.len()
                } else {
                    next
                };
                self.current_playlist_index = Some(next);
                if let Some(entry) = self.playlist.get(next) {
                    let path = entry.path.clone();
                    self.open_media_from_path(&path);
                }
            }
        } else if idx + 1 < self.playlist.len() {
            self.current_playlist_index = Some(idx + 1);
            if let Some(entry) = self.playlist.get(idx + 1) {
                let path = entry.path.clone();
                self.open_media_from_path(&path);
            }
        } else if self.repeat_mode == RepeatMode::All {
            // Wrap around to first track
            self.current_playlist_index = Some(0);
            if let Some(entry) = self.playlist.first() {
                let path = entry.path.clone();
                self.open_media_from_path(&path);
            }
        }
    }

    pub fn prev_track(&mut self) {
        let idx = self.current_playlist_index.unwrap_or(0);
        if idx > 0 {
            self.current_playlist_index = Some(idx - 1);
            if let Some(entry) = self.playlist.get(idx - 1) {
                let path = entry.path.clone();
                self.open_media_from_path(&path);
            }
        } else if self.repeat_mode == RepeatMode::All && !self.playlist.is_empty() {
            // Wrap around to last track
            let last = self.playlist.len() - 1;
            self.current_playlist_index = Some(last);
            if let Some(entry) = self.playlist.get(last) {
                let path = entry.path.clone();
                self.open_media_from_path(&path);
            }
        }
    }

    /// Cycle through repeat modes: None → All → One → None.
    pub fn cycle_repeat_mode(&mut self) {
        self.repeat_mode = self.repeat_mode.cycle();
        self.show_toast(match self.repeat_mode {
            RepeatMode::None => "Repeat off",
            RepeatMode::All => "Repeat all",
            RepeatMode::One => "Repeat one",
        });
    }

    /// Toggle shuffle on/off.
    pub fn toggle_shuffle(&mut self) {
        self.shuffle_enabled = !self.shuffle_enabled;
        self.show_toast(if self.shuffle_enabled {
            "Shuffle on"
        } else {
            "Shuffle off"
        });
    }

    /// Remove a track from the playlist by index.
    pub fn remove_from_playlist(&mut self, index: usize) {
        if index >= self.playlist.len() {
            return;
        }
        self.playlist.remove(index);
        // Update current index
        match self.current_playlist_index {
            Some(ci) if ci == index => {
                // Removed current track
                self.current_playlist_index = if self.playlist.is_empty() {
                    None
                } else if index < self.playlist.len() {
                    Some(index)
                } else {
                    Some(self.playlist.len() - 1)
                };
            }
            Some(ci) if ci > index => {
                self.current_playlist_index = Some(ci - 1);
            }
            _ => {}
        }
    }

    /// Move a track in the playlist from one index to another.
    pub fn move_in_playlist(&mut self, from: usize, to: usize) {
        if from >= self.playlist.len() || to >= self.playlist.len() || from == to {
            return;
        }
        let entry = self.playlist.remove(from);
        self.playlist.insert(to, entry);
        // Update current index
        if let Some(ci) = self.current_playlist_index {
            self.current_playlist_index = Some(if ci == from {
                to
            } else if from < ci && to >= ci {
                ci - 1
            } else if from > ci && to <= ci {
                ci + 1
            } else {
                ci
            });
        }
    }

    /// Cycle aspect ratio mode and show toast.
    pub fn cycle_aspect(&mut self) {
        self.aspect_mode = self.aspect_mode.next();
        self.show_toast(format!("Aspect: {}", self.aspect_mode.label()));
    }
}
