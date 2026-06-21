use super::super::controls;
use super::*;

impl PlayerState {
    /// Cycle through subtitle tracks (external + built-in).
    pub fn cycle_subtitle_track(&mut self) {
        if !self.subtitle_tracks.is_empty() {
            let current = self.subtitle_tracks.iter().position(|t| t.active);
            // Deactivate all
            for t in &mut self.subtitle_tracks {
                t.active = false;
            }
            if let Some(idx) = current {
                let next = (idx + 1) % self.subtitle_tracks.len();
                self.subtitle_tracks[next].active = true;
                self.show_toast(format!(
                    "Subtitle track {} / {}",
                    next + 1,
                    self.subtitle_tracks.len()
                ));
            } else {
                self.subtitle_tracks[0].active = true;
                self.show_toast("Subtitle track 1 active");
            }
        }
    }

    /// Export chapters as JSON string.
    pub fn export_chapters_json(&self) -> String {
        let entries: Vec<serde_json::Value> = self
            .chapters
            .iter()
            .map(|c| {
                serde_json::json!({
                    "title": c.title,
                    "time": c.time,
                    "ai_generated": c.ai_generated,
                })
            })
            .collect();
        serde_json::to_string_pretty(&entries).unwrap_or_default()
    }

    /// Import chapters from JSON string.
    pub fn import_chapters_json(&mut self, json: &str) {
        if let Ok(entries) = serde_json::from_str::<Vec<serde_json::Value>>(json) {
            self.chapters = entries
                .into_iter()
                .filter_map(|v| {
                    Some(ChapterMark {
                        title: v.get("title")?.as_str()?.to_string(),
                        time: v.get("time")?.as_f64()?,
                        ai_generated: v
                            .get("ai_generated")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    })
                })
                .collect();
            self.show_toast(format!("Imported {} chapters", self.chapters.len()));
        } else {
            self.show_toast("Failed to import chapters");
        }
    }

    /// Search subtitle cues for the given keyword and populate results.
    pub fn search_subtitles(&mut self, keyword: &str) {
        self.subtitle_search_results.clear();
        self.subtitle_search_current = None;
        if keyword.is_empty() {
            return;
        }
        let kw = keyword.to_lowercase();
        for (i, cue) in self.subtitle_cues.iter().enumerate() {
            if cue.text.to_lowercase().contains(&kw) {
                self.subtitle_search_results.push(i);
            }
        }
        if !self.subtitle_search_results.is_empty() {
            self.subtitle_search_current = Some(0);
            let cue_idx = self.subtitle_search_results[0];
            self.seek_to(self.subtitle_cues[cue_idx].start);
        }
    }

    /// Jump to next subtitle search result.
    pub fn subtitle_search_next(&mut self) {
        if let Some(cur) = self.subtitle_search_current {
            if cur + 1 < self.subtitle_search_results.len() {
                self.subtitle_search_current = Some(cur + 1);
                let cue_idx = self.subtitle_search_results[cur + 1];
                self.seek_to(self.subtitle_cues[cue_idx].start);
            }
        }
    }

    /// Jump to previous subtitle search result.
    pub fn subtitle_search_prev(&mut self) {
        if let Some(cur) = self.subtitle_search_current {
            if cur > 0 {
                self.subtitle_search_current = Some(cur - 1);
                let cue_idx = self.subtitle_search_results[cur - 1];
                self.seek_to(self.subtitle_cues[cue_idx].start);
            }
        }
    }

    /// Jump to the next subtitle cue after current time.
    pub fn jump_next_subtitle(&mut self) -> Option<f64> {
        let t = self.current_time;
        let result = self
            .subtitle_cues
            .iter()
            .enumerate()
            .find(|(_, cue)| cue.start > t + 0.1)
            .map(|(i, cue)| (i, cue.start));
        if let Some((i, start)) = result {
            self.seek_to(start);
            self.show_toast(format!(
                "Subtitle {} at {}",
                i + 1,
                controls::format_single_time(start)
            ));
            Some(start)
        } else {
            None
        }
    }

    /// Jump to the previous subtitle cue before current time.
    pub fn jump_prev_subtitle(&mut self) -> Option<f64> {
        let t = self.current_time;
        let result = self
            .subtitle_cues
            .iter()
            .enumerate()
            .rev()
            .find(|(_, cue)| cue.end < t - 0.1)
            .map(|(i, cue)| (i, cue.start));
        if let Some((i, start)) = result {
            self.seek_to(start);
            self.show_toast(format!(
                "Subtitle {} at {}",
                i + 1,
                controls::format_single_time(start)
            ));
            Some(start)
        } else {
            None
        }
    }
}
