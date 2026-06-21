use super::*;

impl PixelDesignState {
    // Phase 1: Real undo with document snapshots
    pub fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            let snapshot = self.history[self.history_index].clone();
            self.document = snapshot.document;
            self.status_msg = format!("Undo: {}", snapshot.label);
        }
    }

    pub fn redo(&mut self) {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            let snapshot = self.history[self.history_index].clone();
            self.document = snapshot.document;
            self.status_msg = format!("Redo: {}", snapshot.label);
        }
    }

    pub(super) fn push_history(&mut self, label: &str) {
        // Truncate any redo history beyond current index
        self.history.truncate(self.history_index + 1);
        let snapshot = DocumentSnapshot {
            document: self.document.clone(),
            label: label.into(),
        };
        self.history.push(snapshot);
        self.history_index = self.history.len() - 1;

        // Limit history to 50 entries
        if self.history.len() > 50 {
            let excess = self.history.len() - 50;
            self.history.drain(0..excess);
            self.history_index = self.history.len() - 1;
        }
        self.document.dirty = true;
    }
}
