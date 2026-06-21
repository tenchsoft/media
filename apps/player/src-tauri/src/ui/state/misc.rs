use super::*;

impl PlayerState {
    // ── A-B loop ──

    pub fn toggle_ab_loop(&mut self) {
        match self.ab_stage {
            0 => {
                self.ab_loop = Some((self.current_time, self.current_time));
                self.ab_stage = 1;
                self.show_toast("A point marked");
            }
            1 => {
                if let Some((a, _)) = self.ab_loop {
                    self.ab_loop = Some((a, self.current_time.max(a + 1.0)));
                }
                self.ab_stage = 2;
                self.show_toast("B point marked");
            }
            _ => {
                self.ab_loop = None;
                self.ab_stage = 0;
                self.show_toast("A-B loop cleared");
            }
        }
    }

    // ── Toast ──

    pub fn show_toast(&mut self, message: impl Into<String>) {
        self.toast = Some(message.into());
    }

    // ── Theme ──

    pub fn toggle_theme(&mut self) {
        self.light_theme = !self.light_theme;
    }
}
