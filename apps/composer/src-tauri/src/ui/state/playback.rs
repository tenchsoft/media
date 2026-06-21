use super::*;

impl ComposerState {
    pub fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
        if self.is_playing {
            self.shuttle_direction = 1;
            self.shuttle_speed = 1.0;
        } else {
            self.shuttle_direction = 0;
            self.shuttle_speed = 1.0;
            self.j_press_count = 0;
            self.l_press_count = 0;
        }
    }

    pub fn set_playing(&mut self, playing: bool) {
        self.is_playing = playing;
        if !playing {
            self.shuttle_direction = 0;
            self.shuttle_speed = 1.0;
            self.j_press_count = 0;
            self.l_press_count = 0;
        }
    }

    /// JKL shuttle: J = reverse (press count = speed multiplier).
    pub fn shuttle_reverse(&mut self) {
        self.j_press_count = self.j_press_count.saturating_add(1);
        self.l_press_count = 0;
        self.shuttle_direction = -1;
        self.shuttle_speed = match self.j_press_count {
            1 => 1.0,
            2 => 2.0,
            3 => 4.0,
            _ => 8.0,
        };
        self.is_playing = true;
    }

    /// JKL shuttle: L = forward (press count = speed multiplier).
    pub fn shuttle_forward(&mut self) {
        self.l_press_count = self.l_press_count.saturating_add(1);
        self.j_press_count = 0;
        self.shuttle_direction = 1;
        self.shuttle_speed = match self.l_press_count {
            1 => 1.0,
            2 => 2.0,
            3 => 4.0,
            _ => 8.0,
        };
        self.is_playing = true;
    }

    /// JKL shuttle: K = stop.
    pub fn shuttle_stop(&mut self) {
        self.shuttle_direction = 0;
        self.shuttle_speed = 1.0;
        self.is_playing = false;
        self.j_press_count = 0;
        self.l_press_count = 0;
    }

    pub fn seek_to_frame(&mut self, frame: u32) {
        self.current_frame = frame.min(self.total_frames().saturating_sub(1));
    }

    pub fn step_frame(&mut self, delta: i32) {
        if delta.is_negative() {
            self.current_frame = self.current_frame.saturating_sub(delta.unsigned_abs());
        } else {
            self.seek_to_frame(self.current_frame.saturating_add(delta as u32));
        }
    }

    /// Advance playback by the shuttle speed. Returns true if still playing.
    pub fn advance_playback(&mut self) -> bool {
        if !self.is_playing {
            return false;
        }
        let delta = (self.shuttle_speed * self.shuttle_direction as f64).round() as i32;
        self.step_frame(delta);

        // Loop playback within in/out points.
        if self.loop_playback {
            if let (Some(inp), Some(outp)) = (self.in_point, self.out_point) {
                if self.current_frame >= outp {
                    self.seek_to_frame(inp);
                }
            }
        }

        // Wrap at end.
        if self.current_frame >= self.total_frames().saturating_sub(1) && !self.loop_playback {
            // Clamp but don't stop.
        }
        true
    }

    pub fn select_clip_at_frame(&mut self, track_idx: usize) -> bool {
        let Some(track) = self.tracks().get(track_idx) else {
            return false;
        };
        let Some(clip) = track.clip_at_frame(self.current_frame) else {
            self.selected_clip_id = None;
            return false;
        };
        self.selected_clip_id = Some(clip.id);
        true
    }
}
