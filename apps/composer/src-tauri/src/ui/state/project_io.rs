use super::*;
use tench_composer_core::ComposerProject;

impl ComposerState {
    /// Save project to disk.
    pub fn save_project(&mut self) {
        if let Some(path) = &self.save_path {
            match self.project.to_json() {
                Ok(json) => {
                    if let Err(e) = tench_media_runtime::composer::save_project_json(path, &json) {
                        self.set_notice(format!("Save failed: {e}"));
                    } else {
                        self.set_notice("Project saved");
                        self.last_auto_save = Some(std::time::Instant::now());
                    }
                }
                Err(e) => self.set_notice(format!("Save failed: {e}")),
            }
        }
        // If no save_path, caller should trigger a save dialog.
    }

    /// Save project to a new path.
    pub fn save_project_as(&mut self, path: String) {
        self.save_path = Some(path.clone());
        self.save_project();
    }

    /// Load project from JSON string.
    pub fn load_project(&mut self, json: &str) {
        if let Ok(project) = ComposerProject::from_json(json) {
            self.project = project;
            self.selected_clip_id = None;
            self.selected_clip_ids.clear();
            self.current_frame = 0;
            self.is_playing = false;
            self.set_notice("Project loaded");
        } else {
            self.set_notice("Failed to load project");
        }
    }
}
