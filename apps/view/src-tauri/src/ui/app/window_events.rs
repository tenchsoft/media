// ---------------------------------------------------------------------------
// Window events, animation frames, and file drops
// ---------------------------------------------------------------------------

use tench_ui::anim::AnimInterval;
use tench_ui::core::events::WindowEvent;
use tench_ui::prelude::*;

use super::ViewApp;

impl ViewApp {
    pub(super) fn handle_window_event(&mut self, ctx: &mut EventCtx, event: &WindowEvent) {
        match event {
            WindowEvent::AnimFrame(ts) => {
                self.handle_anim_frame(*ts, ctx);
                ctx.request_paint();
            }
            WindowEvent::FileDrop { paths } => {
                self.handle_file_drop(paths);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    /// Handles animation frame events for slideshow and transitions.
    fn handle_anim_frame(&mut self, ts: u64, ctx: &mut EventCtx) {
        // Slideshow timer: auto-advance
        if self.state.slideshow_playing {
            if self.state.slideshow_timer.is_none() {
                self.state.slideshow_timer =
                    Some(AnimInterval::new(self.state.slideshow_interval_ms as f64));
            }
            if let Some(ref mut timer) = self.state.slideshow_timer {
                timer.interval_ms = self.state.slideshow_interval_ms as f64;
                let ticks = timer.update(ts);
                if ticks > 0 {
                    // Store previous image for fade transition
                    self.state.slideshow_prev_image = self.state.current_image_data.clone();
                    self.state.slideshow_fade_alpha = 0.0;
                    self.state.slideshow_fade_timer = Some(AnimInterval::new(16.0)); // ~60fps fade
                    self.navigate_and_load(true);
                    self.prefetch_adjacent();
                }
            }
            ctx.request_anim_frame();
        }

        // Fade transition animation
        if self.state.slideshow_fade_alpha < 1.0 {
            if let Some(ref mut fade_timer) = self.state.slideshow_fade_timer {
                let ticks = fade_timer.update(ts);
                // Each tick advances ~16ms, fade over 300ms = ~19 ticks
                let alpha_step = 1.0 / 19.0;
                self.state.slideshow_fade_alpha =
                    (self.state.slideshow_fade_alpha + alpha_step * ticks as f64).min(1.0);
            }
            ctx.request_anim_frame();
            ctx.request_paint();
        } else {
            // Fade complete, clear previous
            if self.state.slideshow_prev_image.is_some() {
                self.state.slideshow_prev_image = None;
                self.state.slideshow_fade_timer = None;
            }
        }
    }

    /// Handles file drag-and-drop events.
    fn handle_file_drop(&mut self, paths: &[String]) {
        for path in paths {
            let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
            if ["zip", "cbz", "7z", "rar", "tar", "gz"].contains(&ext.as_str()) {
                self.open_archive(path);
            } else if std::path::Path::new(path).is_dir() {
                self.open_folder(path);
            } else {
                // Assume it's an image file
                self.add_recent_file(path);
                self.load_image_from_path(path);
                // If multiple files are dropped, load the first one and add the rest to folder entries
                break;
            }
        }
    }
}
