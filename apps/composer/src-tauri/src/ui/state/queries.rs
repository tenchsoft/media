use super::*;
use tench_composer_core::*;

impl ComposerState {
    pub fn timeline(&self) -> &Timeline {
        &self.project.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.project.timeline
    }

    pub fn total_frames(&self) -> u32 {
        self.project.timeline.duration().max(1)
    }

    pub fn fps(&self) -> f64 {
        self.project.timeline.framerate
    }

    pub fn tracks(&self) -> &[Track] {
        &self.project.timeline.tracks
    }

    pub fn media_bin(&self) -> &[MediaAsset] {
        &self.project.media_bin
    }

    pub fn templates(&self) -> &[ProjectTemplate] {
        &self.project.templates
    }

    pub fn render_jobs(&self) -> &[RenderJob] {
        &self.project.render_queue
    }

    /// Find the selected clip.
    pub fn selected_clip(&self) -> Option<&Clip> {
        let id = self.selected_clip_id?;
        self.tracks()
            .iter()
            .find_map(|t| t.clips.iter().find(|c| c.id == id))
    }

    /// Find any clip by ID.
    pub fn find_clip(&self, clip_id: ClipId) -> Option<&Clip> {
        self.tracks()
            .iter()
            .find_map(|t| t.clips.iter().find(|c| c.id == clip_id))
    }

    /// Find any clip by ID mutably.
    pub fn find_clip_mut(&mut self, clip_id: ClipId) -> Option<&mut Clip> {
        self.project
            .timeline
            .tracks
            .iter_mut()
            .find_map(|t| t.clips.iter_mut().find(|c| c.id == clip_id))
    }

    /// Find the track containing the selected clip.
    pub fn selected_track(&self) -> Option<&Track> {
        let id = self.selected_clip_id?;
        self.tracks()
            .iter()
            .find(|t| t.clips.iter().any(|c| c.id == id))
    }

    /// Find the track containing a given clip.
    pub fn track_for_clip(&self, clip_id: ClipId) -> Option<&Track> {
        self.tracks()
            .iter()
            .find(|t| t.clips.iter().any(|c| c.id == clip_id))
    }

    /// Get the first track of a given type.
    pub fn first_track_of_type(&self, kind: TrackType) -> Option<&Track> {
        self.tracks().iter().find(|t| t.kind == kind)
    }

    /// Compute snap position: nearest clip boundary within threshold frames.
    pub fn snap_position(&self, frame: u32, threshold: u32) -> u32 {
        if !self.snap {
            return frame;
        }
        let mut best = frame;
        let mut best_dist = threshold;
        for track in self.tracks() {
            for clip in &track.clips {
                for &boundary in &[clip.timeline_in, clip.timeline_out()] {
                    let dist = (frame as i32 - boundary as i32).unsigned_abs();
                    if dist < best_dist {
                        best_dist = dist;
                        best = boundary;
                    }
                }
            }
        }
        best
    }
}
