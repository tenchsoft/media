use super::*;

impl ComposerState {
    pub fn set_notice(&mut self, msg: impl Into<String>) {
        self.composer_notice = msg.into();
        self.notice_expires_at =
            Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
    }

    pub fn check_notice_expiry(&mut self) {
        if let Some(expires) = self.notice_expires_at {
            if std::time::Instant::now() >= expires {
                self.composer_notice.clear();
                self.notice_expires_at = None;
            }
        }
    }

    /// Check if auto-save should trigger. Returns true if saved.
    pub fn check_auto_save(&mut self) -> bool {
        if self.save_path.is_none() {
            return false;
        }
        let now = std::time::Instant::now();
        let last = self.last_auto_save.unwrap_or(now - self.auto_save_interval);
        if now.duration_since(last) >= self.auto_save_interval {
            self.save_project();
            true
        } else {
            false
        }
    }
}
