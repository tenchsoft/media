use super::*;

impl PlayerState {
    // ── Bookmarks / chapters ──

    pub fn add_bookmark(&mut self) {
        let idx = self.chapters.len() + 1;
        self.chapters.push(ChapterMark {
            title: format!("Bookmark {}", idx),
            time: self.current_time,
            ai_generated: false,
        });
        // Immediately enter rename mode for the new chapter
        self.chapter_rename_idx = Some(self.chapters.len() - 1);
        self.chapter_rename_text = format!("Bookmark {}", idx);
        self.show_toast("Bookmark added — type name and press Enter");
    }

    /// Delete a chapter by index.
    pub fn delete_chapter(&mut self, idx: usize) {
        if idx < self.chapters.len() {
            self.chapters.remove(idx);
            self.show_toast("Chapter deleted");
        }
    }

    /// Rename a chapter by index.
    pub fn rename_chapter(&mut self, idx: usize, name: String) {
        if let Some(ch) = self.chapters.get_mut(idx) {
            ch.title = name;
        }
    }

    /// Add a chapter at the current position with the given name.
    pub fn add_chapter_at_current(&mut self, name: String) {
        let chapter = ChapterMark {
            title: name,
            time: self.current_time,
            ai_generated: false,
        };
        self.chapters.push(chapter);
        // Sort chapters by time
        self.chapters.sort_by(|a, b| {
            a.time
                .partial_cmp(&b.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}
