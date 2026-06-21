//! Timeline data model: Clip, Track, Timeline, and edit operations.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a clip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClipId(pub u64);

impl fmt::Display for ClipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "clip-{}", self.0)
    }
}

/// Unique identifier for a track.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub u64);

impl fmt::Display for TrackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "track-{}", self.0)
    }
}

/// Track type: video, audio, or subtitle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrackType {
    Video,
    Audio,
    Subtitle,
}

impl TrackType {
    pub const fn badge(self) -> &'static str {
        match self {
            Self::Video => "V",
            Self::Audio => "A",
            Self::Subtitle => "CC",
        }
    }
}

/// Time range in frames (inclusive start, exclusive end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: u32,
    pub end: u32,
}

impl TimeRange {
    pub fn new(start: u32, end: u32) -> Self {
        debug_assert!(start <= end, "TimeRange start must be <= end");
        Self { start, end }
    }

    pub fn duration(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    pub fn contains(&self, frame: u32) -> bool {
        frame >= self.start && frame < self.end
    }

    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start < other.end && other.start < self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// A media clip on the timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: ClipId,
    pub name: String,
    /// Source media file path.
    pub source_path: String,
    /// In-point on the source media (in frames from media start).
    pub media_in: u32,
    /// Out-point on the source media (in frames from media start).
    pub media_out: u32,
    /// Position on the timeline (frame where this clip starts).
    pub timeline_in: u32,
    /// Duration of this clip on the timeline in frames.
    pub duration: u32,
    /// Playback speed multiplier (1.0 = normal, 2.0 = double speed).
    pub speed: f64,
    /// Whether the clip is reversed.
    pub reversed: bool,
    /// Whether this clip is enabled (visible/audible).
    pub enabled: bool,
    /// Effects applied to this clip (indices into project effects list).
    pub effect_ids: Vec<u64>,
    /// Transition in (index into project transitions).
    pub transition_in: Option<u64>,
    /// Transition out (index into project transitions).
    pub transition_out: Option<u64>,
}

impl Clip {
    /// Timeline out-point (exclusive).
    pub fn timeline_out(&self) -> u32 {
        self.timeline_in.saturating_add(self.duration)
    }

    /// Timeline range.
    pub fn timeline_range(&self) -> TimeRange {
        TimeRange::new(self.timeline_in, self.timeline_out())
    }

    /// Source media duration used.
    pub fn media_duration(&self) -> u32 {
        self.media_out.saturating_sub(self.media_in)
    }

    /// Map a timeline frame to a source media frame.
    pub fn timeline_to_media_frame(&self, timeline_frame: u32) -> u32 {
        let offset = timeline_frame.saturating_sub(self.timeline_in);
        let media_offset = if self.speed > 0.0 {
            (offset as f64 * self.speed) as u32
        } else {
            offset
        };
        if self.reversed {
            self.media_out
                .saturating_sub(1)
                .saturating_sub(media_offset)
        } else {
            self.media_in.saturating_add(media_offset)
        }
    }

    /// Whether this clip contains the given timeline frame.
    pub fn contains_frame(&self, frame: u32) -> bool {
        self.timeline_range().contains(frame)
    }
}

/// A track on the timeline (video, audio, or subtitle).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub kind: TrackType,
    pub clips: Vec<Clip>,
    pub muted: bool,
    pub locked: bool,
    pub hidden: bool,
    pub volume: f64,
    pub pan: f64,
}

impl Track {
    pub fn new(id: TrackId, name: String, kind: TrackType) -> Self {
        Self {
            id,
            name,
            kind,
            clips: Vec::new(),
            muted: false,
            locked: false,
            hidden: false,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Find the clip at the given timeline frame using binary search.
    pub fn clip_at_frame(&self, frame: u32) -> Option<&Clip> {
        // Binary search for the clip whose timeline range contains `frame`.
        let idx = self.clips.partition_point(|c| c.timeline_in <= frame);
        if idx == 0 {
            return None;
        }
        let clip = &self.clips[idx - 1];
        if clip.contains_frame(frame) {
            Some(clip)
        } else {
            None
        }
    }

    /// Find the clip at the given timeline frame (mutable).
    pub fn clip_at_frame_mut(&mut self, frame: u32) -> Option<&mut Clip> {
        let idx = self.clips.partition_point(|c| c.timeline_in <= frame);
        if idx == 0 {
            return None;
        }
        let clip = &mut self.clips[idx - 1];
        if clip.contains_frame(frame) {
            Some(clip)
        } else {
            None
        }
    }

    /// Find clip index by ID.
    pub fn clip_index(&self, id: ClipId) -> Option<usize> {
        self.clips.iter().position(|c| c.id == id)
    }

    /// Total duration of this track (end of last clip).
    pub fn duration(&self) -> u32 {
        self.clips
            .iter()
            .map(|c| c.timeline_out())
            .max()
            .unwrap_or(0)
    }

    /// Insert a clip at the appropriate position (sorted by timeline_in).
    pub fn insert_clip_sorted(&mut self, clip: Clip) {
        let idx = self
            .clips
            .partition_point(|c| c.timeline_in < clip.timeline_in);
        self.clips.insert(idx, clip);
    }

    /// Find the gap at the given frame position.
    /// Returns (gap_start, gap_end) if there is a gap.
    pub fn gap_at(&self, frame: u32) -> Option<(u32, u32)> {
        let idx = self.clips.partition_point(|c| c.timeline_in <= frame);
        if idx == 0 {
            // Before the first clip
            if let Some(first) = self.clips.first() {
                if frame < first.timeline_in {
                    return Some((frame, first.timeline_in));
                }
            }
            return None;
        }
        let prev_end = self.clips[idx - 1].timeline_out();
        if idx < self.clips.len() {
            let next_start = self.clips[idx].timeline_in;
            if frame >= prev_end && frame < next_start {
                return Some((prev_end, next_start));
            }
        }
        None
    }
}

/// Edit mode for insert operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditMode {
    /// Insert shifts subsequent clips to make room.
    Insert,
    /// Overwrite replaces any existing clips in the target range.
    Overwrite,
}

/// The multi-track timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub tracks: Vec<Track>,
    pub framerate: f64,
    pub width: u32,
    pub height: u32,
    /// Duration in frames (0 = auto from track content).
    pub duration_frames: u32,
}

impl Timeline {
    pub fn new(framerate: f64, width: u32, height: u32) -> Self {
        Self {
            tracks: Vec::new(),
            framerate,
            width,
            height,
            duration_frames: 0,
        }
    }

    /// Effective duration: explicit or computed from tracks.
    pub fn duration(&self) -> u32 {
        if self.duration_frames > 0 {
            self.duration_frames
        } else {
            self.tracks.iter().map(|t| t.duration()).max().unwrap_or(0)
        }
    }

    /// Duration in seconds.
    pub fn duration_seconds(&self) -> f64 {
        self.duration() as f64 / self.framerate
    }

    /// Find track by ID.
    pub fn track(&self, id: TrackId) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    /// Find track by ID (mutable).
    pub fn track_mut(&mut self, id: TrackId) -> Option<&mut Track> {
        self.tracks.iter_mut().find(|t| t.id == id)
    }

    /// Find track index by ID.
    pub fn track_index(&self, id: TrackId) -> Option<usize> {
        self.tracks.iter().position(|t| t.id == id)
    }

    /// Add a new track.
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Remove a track by ID.
    pub fn remove_track(&mut self, id: TrackId) -> bool {
        let idx = self.track_index(id);
        if let Some(i) = idx {
            self.tracks.remove(i);
            true
        } else {
            false
        }
    }

    /// Insert a clip onto a track using the given edit mode.
    pub fn insert_clip(
        &mut self,
        track_id: TrackId,
        clip: Clip,
        mode: EditMode,
    ) -> Result<(), String> {
        let track = self
            .track_mut(track_id)
            .ok_or_else(|| format!("Track {} not found", track_id))?;

        match mode {
            EditMode::Insert => {
                // Shift subsequent clips by clip duration.
                let shift = clip.duration;
                for existing in &mut track.clips {
                    if existing.timeline_in >= clip.timeline_in {
                        existing.timeline_in = existing.timeline_in.saturating_add(shift);
                    }
                }
                track.insert_clip_sorted(clip);
            }
            EditMode::Overwrite => {
                // Remove any overlapping portions.
                let range = clip.timeline_range();
                let mut new_clips = Vec::new();
                for existing in track.clips.drain(..) {
                    let er = existing.timeline_range();
                    if !er.overlaps(&range) {
                        new_clips.push(existing);
                    } else {
                        // Split around the overwrite range.
                        if er.start < range.start {
                            let mut left = existing.clone();
                            left.duration = range.start.saturating_sub(er.start);
                            left.media_out = left.media_in.saturating_add(left.duration);
                            left.transition_out = None;
                            new_clips.push(left);
                        }
                        if er.end > range.end {
                            let mut right = existing.clone();
                            let cut = range.end.saturating_sub(er.start);
                            right.timeline_in = range.end;
                            right.duration = er.end.saturating_sub(range.end);
                            right.media_in = right.media_in.saturating_add(cut);
                            right.transition_in = None;
                            new_clips.push(right);
                        }
                    }
                }
                new_clips.push(clip);
                new_clips.sort_by_key(|c| c.timeline_in);
                track.clips = new_clips;
            }
        }
        Ok(())
    }

    /// Remove a clip from a track by ID.
    pub fn remove_clip(&mut self, track_id: TrackId, clip_id: ClipId) -> Result<(), String> {
        let track = self
            .track_mut(track_id)
            .ok_or_else(|| format!("Track {} not found", track_id))?;
        let before = track.clips.len();
        track.clips.retain(|c| c.id != clip_id);
        if track.clips.len() == before {
            return Err(format!("Clip {} not found", clip_id));
        }
        Ok(())
    }

    /// Split a clip at the given frame.
    /// Returns the two resulting clip IDs (left, right).
    pub fn split_clip(
        &mut self,
        track_id: TrackId,
        clip_id: ClipId,
        frame: u32,
        new_id: ClipId,
    ) -> Result<(ClipId, ClipId), String> {
        let track = self
            .track_mut(track_id)
            .ok_or_else(|| format!("Track {} not found", track_id))?;

        let idx = track
            .clip_index(clip_id)
            .ok_or_else(|| format!("Clip {} not found", clip_id))?;

        let clip = &track.clips[idx];
        if frame <= clip.timeline_in || frame >= clip.timeline_out() {
            return Err("Split point must be inside the clip".into());
        }

        let original = track.clips[idx].clone();
        let split_point = frame.saturating_sub(original.timeline_in);
        let media_split = original.media_in.saturating_add(split_point);

        // Modify the left half.
        track.clips[idx].duration = split_point;
        track.clips[idx].media_out = media_split;
        track.clips[idx].transition_out = None;

        // Create the right half.
        let right = Clip {
            id: new_id,
            name: format!("{} (2)", original.name),
            source_path: original.source_path.clone(),
            media_in: media_split,
            media_out: original.media_out,
            timeline_in: frame,
            duration: original.duration.saturating_sub(split_point),
            speed: original.speed,
            reversed: original.reversed,
            enabled: original.enabled,
            effect_ids: original.effect_ids.clone(),
            transition_in: None,
            transition_out: original.transition_out,
        };

        let right_id = right.id;
        track.clips.insert(idx + 1, right);
        Ok((clip_id, right_id))
    }

    /// Trim the in-point of a clip.
    pub fn trim_clip_in(
        &mut self,
        track_id: TrackId,
        clip_id: ClipId,
        new_timeline_in: u32,
    ) -> Result<(), String> {
        let track = self
            .track_mut(track_id)
            .ok_or_else(|| format!("Track {} not found", track_id))?;

        let clip = track
            .clips
            .iter_mut()
            .find(|c| c.id == clip_id)
            .ok_or_else(|| format!("Clip {} not found", clip_id))?;

        if new_timeline_in >= clip.timeline_out() {
            return Err("New in-point must be before out-point".into());
        }

        let delta = new_timeline_in.saturating_sub(clip.timeline_in) as f64;
        let media_delta = (delta * clip.speed) as u32;
        clip.media_in = clip.media_in.saturating_add(media_delta);
        clip.duration = clip.duration.saturating_sub(delta as u32);
        clip.timeline_in = new_timeline_in;
        Ok(())
    }

    /// Trim the out-point of a clip.
    pub fn trim_clip_out(
        &mut self,
        track_id: TrackId,
        clip_id: ClipId,
        new_timeline_out: u32,
    ) -> Result<(), String> {
        let track = self
            .track_mut(track_id)
            .ok_or_else(|| format!("Track {} not found", track_id))?;

        let clip = track
            .clips
            .iter_mut()
            .find(|c| c.id == clip_id)
            .ok_or_else(|| format!("Clip {} not found", clip_id))?;

        if new_timeline_out <= clip.timeline_in {
            return Err("New out-point must be after in-point".into());
        }

        let new_duration = new_timeline_out.saturating_sub(clip.timeline_in);
        let media_delta = ((new_duration as f64 - clip.duration as f64) * clip.speed) as i32;
        if media_delta > 0 {
            clip.media_out = clip.media_out.saturating_add(media_delta as u32);
        } else {
            clip.media_out = clip.media_out.saturating_sub(media_delta.unsigned_abs());
        }
        clip.duration = new_duration;
        Ok(())
    }

    /// Move a clip to a new position (optionally on a different track).
    pub fn move_clip(
        &mut self,
        clip_id: ClipId,
        source_track_id: TrackId,
        dest_track_id: TrackId,
        new_timeline_in: u32,
    ) -> Result<(), String> {
        // Extract the clip from source track.
        let source = self
            .track_mut(source_track_id)
            .ok_or_else(|| format!("Source track {} not found", source_track_id))?;
        let idx = source
            .clip_index(clip_id)
            .ok_or_else(|| format!("Clip {} not found", clip_id))?;
        let mut clip = source.clips.remove(idx);
        clip.timeline_in = new_timeline_in;

        // Insert into destination track.
        let dest = self
            .track_mut(dest_track_id)
            .ok_or_else(|| format!("Dest track {} not found", dest_track_id))?;
        dest.insert_clip_sorted(clip);
        Ok(())
    }

    /// Ripple delete: remove a clip and close the gap.
    pub fn ripple_delete(&mut self, track_id: TrackId, clip_id: ClipId) -> Result<(), String> {
        let track = self
            .track_mut(track_id)
            .ok_or_else(|| format!("Track {} not found", track_id))?;

        let idx = track
            .clip_index(clip_id)
            .ok_or_else(|| format!("Clip {} not found", clip_id))?;

        let removed_duration = track.clips[idx].duration;
        track.clips.remove(idx);

        // Shift subsequent clips left.
        let shift_threshold = track.clips.get(idx).map_or(u32::MAX, |c| c.timeline_in);
        for clip in &mut track.clips {
            if clip.timeline_in >= shift_threshold {
                clip.timeline_in = clip.timeline_in.saturating_sub(removed_duration);
            }
        }
        Ok(())
    }

    /// Remove all gaps on a track.
    pub fn remove_gaps(&mut self, track_id: TrackId) -> Result<(), String> {
        let track = self
            .track_mut(track_id)
            .ok_or_else(|| format!("Track {} not found", track_id))?;

        track.clips.sort_by_key(|c| c.timeline_in);

        let mut cursor = 0u32;
        for clip in &mut track.clips {
            if clip.timeline_in > cursor {
                clip.timeline_in = cursor;
            }
            cursor = clip.timeline_out();
        }
        Ok(())
    }

    /// Find all clips at the given frame across all tracks.
    pub fn clips_at_frame(&self, frame: u32) -> Vec<(&Track, &Clip)> {
        self.tracks
            .iter()
            .filter_map(|track| track.clip_at_frame(frame).map(|clip| (track, clip)))
            .collect()
    }

    /// Default tracks for a new project.
    pub fn default_tracks(next_id: &mut u64) -> Self {
        let mut timeline = Self::new(24.0, 1920, 1080);
        let v1 = TrackId(*next_id);
        *next_id += 1;
        let v2 = TrackId(*next_id);
        *next_id += 1;
        let a1 = TrackId(*next_id);
        *next_id += 1;
        let a2 = TrackId(*next_id);
        *next_id += 1;
        let sub = TrackId(*next_id);
        *next_id += 1;

        timeline.add_track(Track::new(v2, "V2 - Overlay".into(), TrackType::Video));
        timeline.add_track(Track::new(v1, "V1 - Main".into(), TrackType::Video));
        timeline.add_track(Track::new(a1, "A1 - Audio".into(), TrackType::Audio));
        timeline.add_track(Track::new(a2, "A2 - Music".into(), TrackType::Audio));
        timeline.add_track(Track::new(
            sub,
            "S1 - Subtitles".into(),
            TrackType::Subtitle,
        ));
        timeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_clip(id: u64, name: &str, source: &str, timeline_in: u32, duration: u32) -> Clip {
        Clip {
            id: ClipId(id),
            name: name.into(),
            source_path: source.into(),
            media_in: 0,
            media_out: duration,
            timeline_in,
            duration,
            speed: 1.0,
            reversed: false,
            enabled: true,
            effect_ids: Vec::new(),
            transition_in: None,
            transition_out: None,
        }
    }

    #[test]
    fn time_range_contains_and_overlaps() {
        let r = TimeRange::new(10, 20);
        assert!(r.contains(10));
        assert!(r.contains(19));
        assert!(!r.contains(20));
        assert!(r.overlaps(&TimeRange::new(15, 25)));
        assert!(!r.overlaps(&TimeRange::new(20, 30)));
    }

    #[test]
    fn clip_timeline_mapping() {
        let clip = make_clip(1, "test", "a.mp4", 100, 50);
        assert_eq!(clip.timeline_out(), 150);
        assert!(clip.contains_frame(100));
        assert!(clip.contains_frame(149));
        assert!(!clip.contains_frame(150));
        assert_eq!(clip.timeline_to_media_frame(120), 20);
    }

    #[test]
    fn clip_reversed_mapping() {
        let mut clip = make_clip(1, "test", "a.mp4", 100, 50);
        clip.reversed = true;
        // Frame 100 maps to media_out - 1 (last frame), frame 149 maps to media_in (first frame)
        assert_eq!(clip.timeline_to_media_frame(100), 49);
        assert_eq!(clip.timeline_to_media_frame(149), 0);
    }

    #[test]
    fn track_binary_search_finds_clip() {
        let mut track = Track::new(TrackId(1), "V1".into(), TrackType::Video);
        track.clips.push(make_clip(1, "A", "a.mp4", 0, 100));
        track.clips.push(make_clip(2, "B", "b.mp4", 100, 100));
        track.clips.push(make_clip(3, "C", "c.mp4", 200, 100));

        assert_eq!(track.clip_at_frame(50).map(|c| c.id), Some(ClipId(1)));
        assert_eq!(track.clip_at_frame(150).map(|c| c.id), Some(ClipId(2)));
        assert_eq!(track.clip_at_frame(250).map(|c| c.id), Some(ClipId(3)));
        assert!(track.clip_at_frame(300).is_none());
    }

    #[test]
    fn insert_clip_shifts_subsequent() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 0, 100),
            EditMode::Insert,
        )
        .unwrap();
        tl.insert_clip(
            TrackId(1),
            make_clip(2, "B", "b.mp4", 100, 100),
            EditMode::Insert,
        )
        .unwrap();

        // Insert a clip at frame 50 — should shift B.
        tl.insert_clip(
            TrackId(1),
            make_clip(3, "C", "c.mp4", 50, 30),
            EditMode::Insert,
        )
        .unwrap();

        let track = tl.track(TrackId(1)).unwrap();
        assert_eq!(track.clips[0].id, ClipId(1)); // A: 0-100
        assert_eq!(track.clips[1].id, ClipId(3)); // C: 50-80
        assert_eq!(track.clips[2].id, ClipId(2)); // B: shifted to 130-230
        assert_eq!(track.clips[2].timeline_in, 130);
    }

    #[test]
    fn overwrite_clip_splits_existing() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 0, 200),
            EditMode::Insert,
        )
        .unwrap();

        // Overwrite middle portion.
        tl.insert_clip(
            TrackId(1),
            make_clip(2, "B", "b.mp4", 50, 80),
            EditMode::Overwrite,
        )
        .unwrap();

        let track = tl.track(TrackId(1)).unwrap();
        assert_eq!(track.clips.len(), 3);
        // Left: 0-50
        assert_eq!(track.clips[0].id, ClipId(1));
        assert_eq!(track.clips[0].duration, 50);
        // New: 50-130
        assert_eq!(track.clips[1].id, ClipId(2));
        // Right: 130-200
        assert_eq!(track.clips[2].id, ClipId(1));
        assert_eq!(track.clips[2].timeline_in, 130);
        assert_eq!(track.clips[2].duration, 70);
    }

    #[test]
    fn split_clip_at_frame() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 100, 200),
            EditMode::Insert,
        )
        .unwrap();

        let (left, right) = tl
            .split_clip(TrackId(1), ClipId(1), 200, ClipId(99))
            .unwrap();

        assert_eq!(left, ClipId(1));
        assert_eq!(right, ClipId(99));

        let track = tl.track(TrackId(1)).unwrap();
        assert_eq!(track.clips[0].duration, 100);
        assert_eq!(track.clips[0].timeline_in, 100);
        assert_eq!(track.clips[1].timeline_in, 200);
        assert_eq!(track.clips[1].duration, 100);
    }

    #[test]
    fn ripple_delete_closes_gap() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 0, 100),
            EditMode::Insert,
        )
        .unwrap();
        tl.insert_clip(
            TrackId(1),
            make_clip(2, "B", "b.mp4", 100, 100),
            EditMode::Insert,
        )
        .unwrap();
        tl.insert_clip(
            TrackId(1),
            make_clip(3, "C", "c.mp4", 200, 100),
            EditMode::Insert,
        )
        .unwrap();

        tl.ripple_delete(TrackId(1), ClipId(2)).unwrap();

        let track = tl.track(TrackId(1)).unwrap();
        assert_eq!(track.clips.len(), 2);
        assert_eq!(track.clips[1].timeline_in, 100); // C shifted left by 100
    }

    #[test]
    fn remove_gaps_closes_spaces() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 0, 50),
            EditMode::Insert,
        )
        .unwrap();
        // Gap: 50-100
        tl.insert_clip(
            TrackId(1),
            make_clip(2, "B", "b.mp4", 100, 50),
            EditMode::Insert,
        )
        .unwrap();

        tl.remove_gaps(TrackId(1)).unwrap();

        let track = tl.track(TrackId(1)).unwrap();
        assert_eq!(track.clips[0].timeline_in, 0);
        assert_eq!(track.clips[0].duration, 50);
        assert_eq!(track.clips[1].timeline_in, 50); // Closed gap
        assert_eq!(track.clips[1].duration, 50);
    }

    #[test]
    fn trim_clip_in_adjusts_media() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 100, 200),
            EditMode::Insert,
        )
        .unwrap();

        tl.trim_clip_in(TrackId(1), ClipId(1), 150).unwrap();

        let clip = &tl.track(TrackId(1)).unwrap().clips[0];
        assert_eq!(clip.timeline_in, 150);
        assert_eq!(clip.duration, 150);
        assert_eq!(clip.media_in, 50);
    }

    #[test]
    fn trim_clip_out_adjusts_media() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 100, 200),
            EditMode::Insert,
        )
        .unwrap();

        tl.trim_clip_out(TrackId(1), ClipId(1), 250).unwrap();

        let clip = &tl.track(TrackId(1)).unwrap().clips[0];
        assert_eq!(clip.timeline_in, 100);
        assert_eq!(clip.duration, 150);
        assert_eq!(clip.media_out, 150);
    }

    #[test]
    fn move_clip_across_tracks() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.add_track(Track::new(TrackId(2), "V2".into(), TrackType::Video));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "A", "a.mp4", 0, 100),
            EditMode::Insert,
        )
        .unwrap();

        tl.move_clip(ClipId(1), TrackId(1), TrackId(2), 200)
            .unwrap();

        assert!(tl.track(TrackId(1)).unwrap().clips.is_empty());
        let v2 = tl.track(TrackId(2)).unwrap();
        assert_eq!(v2.clips.len(), 1);
        assert_eq!(v2.clips[0].timeline_in, 200);
    }

    #[test]
    fn default_tracks_creates_five_tracks() {
        let mut id = 1u64;
        let tl = Timeline::default_tracks(&mut id);
        assert_eq!(tl.tracks.len(), 5);
        assert_eq!(tl.framerate, 24.0);
        assert_eq!(tl.width, 1920);
        assert_eq!(tl.height, 1080);
    }

    #[test]
    fn clips_at_frame_finds_across_tracks() {
        let mut tl = Timeline::new(24.0, 1920, 1080);
        tl.add_track(Track::new(TrackId(1), "V1".into(), TrackType::Video));
        tl.add_track(Track::new(TrackId(2), "A1".into(), TrackType::Audio));
        tl.insert_clip(
            TrackId(1),
            make_clip(1, "V", "v.mp4", 0, 200),
            EditMode::Insert,
        )
        .unwrap();
        tl.insert_clip(
            TrackId(2),
            make_clip(2, "A", "a.mp3", 0, 200),
            EditMode::Insert,
        )
        .unwrap();

        let found = tl.clips_at_frame(100);
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn gap_at_finds_gap_between_clips() {
        let mut track = Track::new(TrackId(1), "V1".into(), TrackType::Video);
        track.clips.push(make_clip(1, "A", "a.mp4", 0, 50));
        track.clips.push(make_clip(2, "B", "b.mp4", 100, 50));

        let gap = track.gap_at(75);
        assert_eq!(gap, Some((50, 100)));
        assert!(track.gap_at(25).is_none());
    }
}
