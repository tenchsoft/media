// ---------------------------------------------------------------------------
// Click action dispatch: nav
// ---------------------------------------------------------------------------

use tench_ui::anim::AnimInterval;
use tench_ui::prelude::*;

use crate::ui::state::{self, ClickAction, FitMode};
use tauri::Manager;
use tench_image_runtime::view::util::rotate_image_data;

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_navigation_action(
        &mut self,
        action: &ClickAction,
        ctx: &mut EventCtx,
    ) -> bool {
        match action {
            ClickAction::OpenFileDialog => {
                self.open_file_dialog();
                true
            }
            ClickAction::OpenFolderDialog => {
                self.open_folder_dialog();
                true
            }
            ClickAction::OpenArchiveDialog => {
                self.open_archive_dialog();
                true
            }
            ClickAction::NavigateNext => {
                self.navigate_and_load(true);
                self.prefetch_adjacent();
                ctx.request_paint();
                true
            }
            ClickAction::NavigatePrev => {
                self.navigate_and_load(false);
                self.prefetch_adjacent();
                ctx.request_paint();
                true
            }
            ClickAction::ZoomFit => {
                self.state.fit_mode = FitMode::Fit;
                self.state.zoom = 1.0;
                self.state.pan_x = 0.0;
                self.state.pan_y = 0.0;
                ctx.request_paint();
                true
            }
            ClickAction::ZoomActual => {
                self.state.fit_mode = FitMode::Actual;
                self.state.zoom = 1.0;
                self.state.pan_x = 0.0;
                self.state.pan_y = 0.0;
                ctx.request_paint();
                true
            }
            ClickAction::ZoomIn => {
                self.state.fit_mode = FitMode::Actual;
                self.state.zoom = (self.state.zoom + 0.1).clamp(0.1, 8.0);
                ctx.request_paint();
                true
            }
            ClickAction::ZoomOut => {
                self.state.fit_mode = FitMode::Actual;
                self.state.zoom = (self.state.zoom - 0.1).clamp(0.1, 8.0);
                ctx.request_paint();
                true
            }
            ClickAction::ToggleThumbnails => {
                self.state.show_thumbnails = !self.state.show_thumbnails;
                ctx.request_paint();
                true
            }
            ClickAction::Rotate => {
                if let Some(ref image_data) = self.state.current_image_data.clone() {
                    if let Some(rotated) = rotate_image_data(image_data, 1) {
                        if let Some(ref current) = self.state.current_image_data {
                            self.state.push_edit_history(current.clone(), "rotate");
                        }
                        self.state.current_image_data = Some(rotated);
                        if let Some(ref mut doc) = self.state.document {
                            if let Some(ref dims) = doc.dimensions {
                                doc.dimensions = Some(state::ImageDimensions {
                                    width: dims.height,
                                    height: dims.width,
                                });
                            }
                        }
                    }
                }
                ctx.request_paint();
                true
            }
            ClickAction::CycleBgColor => {
                self.state.bg_color = self.state.bg_color.cycle();
                ctx.request_paint();
                true
            }
            ClickAction::ToggleFullscreen => {
                if let Some(ref handle) = self.app_handle {
                    if let Some(wvw) = handle.get_webview_window("main") {
                        let is_fullscreen = wvw.is_fullscreen().unwrap_or(false);
                        let _ = wvw.set_fullscreen(!is_fullscreen);
                    }
                }
                ctx.request_paint();
                true
            }
            ClickAction::ToggleCheckerboard => {
                self.state.checkerboard_bg = !self.state.checkerboard_bg;
                ctx.request_paint();
                true
            }
            ClickAction::ToggleSlideshow => {
                self.state.slideshow_playing = !self.state.slideshow_playing;
                if self.state.slideshow_playing {
                    // Start slideshow: initialize timer
                    self.state.slideshow_timer =
                        Some(AnimInterval::new(self.state.slideshow_interval_ms as f64));
                } else {
                    // Stop slideshow: clear timer and fade state
                    self.state.slideshow_timer = None;
                    self.state.slideshow_prev_image = None;
                    self.state.slideshow_fade_alpha = 1.0;
                    self.state.slideshow_fade_timer = None;
                }
                ctx.request_paint();
                true
            }
            ClickAction::DismissAll => {
                self.state.dismiss_all();
                ctx.request_paint();
                true
            }
            ClickAction::OpenRecentFile(idx) => {
                if let Some(path) = self.state.recent_files.get(*idx).cloned() {
                    self.load_image_from_path(&path);
                    ctx.request_paint();
                }
                true
            }
            ClickAction::NavigateToIndex(idx) => {
                if let Some(entry) = self.state.sorted_entries.get(*idx).cloned() {
                    let path = entry.path.clone();
                    // Navigate to the target index
                    self.state.document = Some(state::ImageMetadata {
                        file_name: entry.file_name,
                        format: entry
                            .path
                            .rsplit('.')
                            .next()
                            .unwrap_or("png")
                            .to_lowercase(),
                        dimensions: None,
                        file_size: entry.size_bytes,
                        path: entry.path,
                    });
                    self.state.reset_for_new_image();

                    // Check cache
                    if let Some(cached) = self.image_cache.get(&path).cloned() {
                        self.state.set_image_data(cached);
                        self.load_exif_data(&path);
                    } else {
                        self.load_image_from_path(&path);
                    }
                    self.prefetch_adjacent();
                    ctx.request_paint();
                }
                true
            }
            ClickAction::ShareImage => {
                if let Some(ref doc) = self.state.document.clone() {
                    let path = doc.path.clone();
                    std::thread::spawn(move || {
                        let _ = crate::platform_util::share_file(&path);
                    });
                }
                true
            }
            ClickAction::SetWallpaperAction => {
                if let Some(ref doc) = self.state.document.clone() {
                    let path = doc.path.clone();
                    std::thread::spawn(move || {
                        let _ = crate::platform_util::set_wallpaper(&path);
                    });
                }
                true
            }
            ClickAction::DeleteFromToolbar => {
                self.state.show_delete_confirm = true;
                ctx.request_paint();
                true
            }

            _ => false,
        }
    }
}
