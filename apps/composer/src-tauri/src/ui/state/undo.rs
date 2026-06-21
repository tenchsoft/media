use super::*;
use tench_composer_core::ComposerProject;

impl ComposerState {
    pub fn push_undo(&mut self) {
        if let Ok(json) = self.project.to_json() {
            self.undo_stack.push(json);
            if self.undo_stack.len() > 50 {
                self.undo_stack.remove(0);
            }
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(json) = self.undo_stack.pop() {
            if let Ok(current) = self.project.to_json() {
                self.redo_stack.push(current);
            }
            if let Ok(restored) = ComposerProject::from_json(&json) {
                self.project = restored;
                self.set_notice("Undo");
                return true;
            }
        }
        self.set_notice("Nothing to undo");
        false
    }

    pub fn redo(&mut self) -> bool {
        if let Some(json) = self.redo_stack.pop() {
            if let Ok(current) = self.project.to_json() {
                self.undo_stack.push(current);
            }
            if let Ok(restored) = ComposerProject::from_json(&json) {
                self.project = restored;
                self.set_notice("Redo");
                return true;
            }
        }
        self.set_notice("Nothing to redo");
        false
    }
}
