use super::*;

impl PlayerState {
    // ── Playback controls ──

    pub fn toggle_playback(&mut self) {
        if self.has_media {
            self.is_playing = !self.is_playing;
        }
    }

    pub fn seek_to(&mut self, time: f64) {
        self.current_time = time.clamp(0.0, self.duration.max(0.0));
        self.update_subtitle_for_position();
    }

    pub fn seek_by(&mut self, delta: f64) {
        self.seek_to(self.current_time + delta);
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
        self.is_muted = self.volume == 0.0;
    }

    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    pub fn set_playback_rate(&mut self, rate: f64) {
        self.playback_rate = rate.clamp(0.1, 4.0);
    }

    pub fn speed_up(&mut self) {
        let options = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];
        if let Some(next) = options.iter().find(|&&s| s > self.playback_rate + 0.01) {
            self.playback_rate = *next;
        }
    }

    pub fn speed_down(&mut self) {
        let options = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];
        if let Some(prev) = options
            .iter()
            .rev()
            .find(|&&s| s < self.playback_rate - 0.01)
        {
            self.playback_rate = *prev;
        }
    }
}
