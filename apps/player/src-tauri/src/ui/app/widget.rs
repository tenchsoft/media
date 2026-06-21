use super::super::state::ClickAction;
use super::super::theme::{BG_DARK, BG_LIGHT};
use super::super::{
    controls, paint_controls, paint_overlays, paint_panels, paint_video, state, video_surface,
};
use super::{automation, PlayerApp};
use tench_ui::core::events::{LogicalKey, NamedKey};
use tench_ui::kurbo;
use tench_ui::prelude::*;

impl Widget for PlayerApp {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn measure(&mut self, _ctx: &mut MeasureCtx, axis: Axis, available: f64) -> f64 {
        match axis {
            Axis::Horizontal => available,
            Axis::Vertical => available,
        }
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _size: Size) {}

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let size = ctx.size();
        let theme = ctx.theme();
        let mut p = Painter::new(scene);

        // Process pending dialog results
        self.process_dialog_results();

        // Process backend events (video frames, position, etc.)
        self.process_backend_events();

        // Note: request_anim_frame is called from on_window_event(AnimFrame) during playback

        // Clear click regions from previous frame
        self.state.clear_click_regions();

        // Background - video surface area
        let player_bg = if self.state.light_theme {
            BG_LIGHT
        } else {
            BG_DARK
        };
        p.fill_background(size, player_bg);

        let controls_h = 64.0;
        let overlay_h = 40.0;
        let side_panel_w = 320.0;
        let spacing = theme.spacing;
        let spacing_large = theme.spacing_large;

        // Video surface area
        let video_rect = video_surface::video_rect(
            size.width,
            size.height,
            overlay_h,
            controls_h,
            self.state.ai_panel_open,
            self.state.drawer.is_some(),
            side_panel_w,
        );

        // Render video frame or empty state
        let cx = video_rect.x1 / 2.0;
        let cy = (video_rect.y0 + video_rect.y1) / 2.0;

        // Video frame / empty state / subtitles / GIF indicator
        paint_video::paint_video(
            &mut p,
            &mut self.state,
            theme,
            &video_rect,
            cx,
            cy,
            self.video_frame.as_ref(),
            self.video_dims,
            self.gif_recording,
            self.gif_recording_start,
        );

        // Top overlay bar (title, drawer tabs, AI button)
        paint_controls::paint_top_bar(
            &mut p,
            &mut self.state,
            theme,
            &video_rect,
            overlay_h,
            spacing,
            spacing_large,
        );

        // Bottom controls bar (seekbar, buttons, volume, speed menu)
        paint_controls::paint_controls(
            &mut p,
            &mut self.state,
            theme,
            size,
            &video_rect,
            controls_h,
            spacing,
            spacing_large,
            self.seek_hover_pos,
            self.seek_thumbnail.as_ref(),
        );

        // AI panel
        paint_panels::paint_ai_panel(
            &mut p,
            &mut self.state,
            theme,
            &video_rect,
            size,
            spacing,
            spacing_large,
        );

        // Drawer tab panels (Playlist, Chapters, Subtitles, Info)
        paint_panels::paint_drawer(
            &mut p,
            &mut self.state,
            theme,
            &video_rect,
            size,
            spacing,
            spacing_large,
            self.backend.as_ref(),
        );

        // GIF capture modal
        paint_overlays::paint_gif_capture_modal(
            &mut p,
            &mut self.state,
            theme,
            size,
            self.gif_recording,
        );

        // Toast notification – reset timer when toast message changes.
        if self.state.toast != self.last_toast {
            self.toast_time = None;
            self.last_toast = self.state.toast.clone();
        }
        paint_overlays::paint_toast(
            &mut p,
            &mut self.state,
            theme,
            size,
            controls_h,
            &mut self.toast_time,
        );

        // Context menu overlay
        paint_overlays::paint_context_menu(&mut p, &mut self.state, theme, size);

        // Modal overlays (help, URL, subtitle style/search, GIF options, EQ, add chapter)
        paint_overlays::paint_modals(&mut p, &mut self.state, theme, size);

        // PiP indicator
        paint_overlays::paint_pip_indicator(&mut p, &mut self.state, size);
    }

    fn debug_id(&self) -> Option<&str> {
        Some("player.root")
    }

    fn automation_children(&self, state: &WidgetState) -> Vec<UiAutomationNode> {
        automation::player_automation_nodes(&self.state, state.size, self.gif_recording)
    }

    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::Down(e) => {
                // Close context menu on any click
                if self.state.context_menu.is_some() {
                    // Check if click is inside context menu
                    if let Some(ref menu) = self.state.context_menu {
                        let menu_w = 220.0;
                        let item_h = 28.0;
                        let info_section_h = 60.0;
                        let menu_h = info_section_h + menu.items.len() as f64 * item_h + 8.0;
                        let mx = menu.x.min(ctx.state.size.width - menu_w);
                        let my = menu.y.min(ctx.state.size.height - menu_h);
                        let menu_rect = Rect::new(mx, my, mx + menu_w, my + menu_h);
                        if menu_rect.contains(kurbo::Point::new(e.pos.x, e.pos.y)) {
                            // Determine which item was clicked
                            if e.pos.y >= my + info_section_h {
                                let idx = ((e.pos.y - my - info_section_h) / item_h) as usize;
                                if let Some(item) = menu.items.get(idx) {
                                    let id = item.id.clone();
                                    self.state.context_menu = None;
                                    self.handle_context_menu_action(&id, ctx);
                                }
                            }
                            return;
                        }
                    }
                    self.state.context_menu = None;
                    ctx.request_paint();
                    return;
                }

                // Right-click: show context menu
                if e.button == tench_ui::core::events::PointerButton::Secondary {
                    if self.state.has_media {
                        use state::ContextMenuItem as CMI;
                        self.state.context_menu = Some(state::ContextMenuState {
                            x: e.pos.x,
                            y: e.pos.y,
                            items: vec![
                                CMI::new(
                                    "play_pause",
                                    if self.state.is_playing {
                                        "Pause"
                                    } else {
                                        "Play"
                                    },
                                ),
                                CMI::new("stop", "Stop"),
                                CMI::new("screenshot", "Screenshot"),
                                CMI::new("fullscreen", "Fullscreen"),
                                CMI::new("open_file", "Open File..."),
                                CMI::new("show_in_files", "Show in Files"),
                                CMI::new(
                                    "cycle_aspect",
                                    format!("Aspect: {}", self.state.aspect_mode.label()),
                                ),
                                CMI::new(
                                    "cycle_repeat",
                                    format!("Repeat: {}", self.state.repeat_mode.label()),
                                ),
                                CMI::new(
                                    "toggle_shuffle",
                                    format!(
                                        "Shuffle: {}",
                                        if self.state.shuffle_enabled {
                                            "On"
                                        } else {
                                            "Off"
                                        }
                                    ),
                                ),
                            ],
                        });
                        ctx.request_paint();
                    }
                    return;
                }

                let size = ctx.state.size;
                let controls_h = 64.0;
                let overlay_h = 40.0;
                let seekbar_margin = 16.0;
                let side_panel_w = 320.0;
                let video_right = video_surface::video_right(
                    size.width,
                    self.state.ai_panel_open,
                    self.state.drawer.is_some(),
                    side_panel_w,
                );

                // Check click regions first
                if let Some(action) = self.state.click_action_at(e.pos.x, e.pos.y).cloned() {
                    match &action {
                        ClickAction::SeekTo(_) => {
                            let ratio = controls::seek_ratio(e.pos.x, seekbar_margin, video_right);
                            let pos = self.state.duration * ratio;
                            self.dispatch_click_action(&ClickAction::SeekTo(pos), ctx);
                            self.dragging_seek = true;
                            return;
                        }
                        ClickAction::VolumeSet(_) => {
                            let vol_x = seekbar_margin + 48.0 + 36.0 + 48.0 + 140.0 + 36.0;
                            let vol = controls::volume_ratio(e.pos.x, vol_x, 80.0);
                            self.dispatch_click_action(&ClickAction::VolumeSet(vol), ctx);
                            self.dragging_volume = true;
                            return;
                        }
                        _ => {}
                    }

                    self.dispatch_click_action(&action, ctx);
                    return;
                }

                // Click didn't match any registered region — unfocus text inputs
                if self.state.ai_focused {
                    self.state.ai_focused = false;
                    ctx.request_paint();
                }

                // Click on video surface area — double-click detection
                if e.pos.y > overlay_h && e.pos.y < size.height - controls_h {
                    let now = std::time::Instant::now();
                    let is_double = self
                        .last_click_time
                        .is_some_and(|t| now.duration_since(t).as_millis() < 400);
                    self.last_click_time = Some(now);

                    if is_double {
                        self.dispatch_click_action(&ClickAction::ToggleFullscreen, ctx);
                    } else {
                        // Single click on video = toggle play/pause
                        self.dispatch_click_action(&ClickAction::PlayPause, ctx);
                    }
                }
            }
            PointerEvent::Move(e) => {
                // Update context menu hover highlight
                if let Some(ref menu) = self.state.context_menu {
                    let menu_w = 220.0;
                    let item_h = 28.0;
                    let info_section_h = 60.0;
                    let menu_h = info_section_h + menu.items.len() as f64 * item_h + 8.0;
                    let mx = menu.x.min(ctx.state.size.width - menu_w);
                    let my = menu.y.min(ctx.state.size.height - menu_h);
                    let menu_rect = Rect::new(mx, my, mx + menu_w, my + menu_h);
                    if menu_rect.contains(kurbo::Point::new(e.pos.x, e.pos.y)) {
                        let new_hover = if e.pos.y >= my + info_section_h {
                            let idx = ((e.pos.y - my - info_section_h) / item_h) as usize;
                            if idx < menu.items.len() {
                                Some(idx)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        if self.state.context_menu_hover != new_hover {
                            self.state.context_menu_hover = new_hover;
                            ctx.request_paint();
                        }
                    } else if self.state.context_menu_hover.is_some() {
                        self.state.context_menu_hover = None;
                        ctx.request_paint();
                    }
                }

                if self.dragging_seek {
                    let size = ctx.state.size;
                    let seekbar_margin = 16.0;
                    let side_panel_w = 320.0;
                    let video_right = video_surface::video_right(
                        size.width,
                        self.state.ai_panel_open,
                        self.state.drawer.is_some(),
                        side_panel_w,
                    );
                    let ratio = controls::seek_ratio(e.pos.x, seekbar_margin, video_right);
                    let pos = self.state.duration * ratio;
                    self.state.seek_to(pos);
                    if let Some(ref mut backend) = self.backend {
                        backend.seek(pos);
                    }
                    ctx.request_paint();
                } else if self.dragging_volume {
                    let seekbar_margin = 16.0;
                    let vol_x = seekbar_margin + 48.0 + 36.0 + 48.0 + 140.0 + 36.0;
                    let vol = controls::volume_ratio(e.pos.x, vol_x, 80.0);
                    self.state.set_volume(vol);
                    if let Some(ref mut backend) = self.backend {
                        backend.set_volume(vol);
                    }
                    ctx.request_paint();
                } else {
                    // Check if hovering over seekbar for thumbnail preview
                    let size = ctx.state.size;
                    let controls_h = 64.0;
                    let seekbar_margin = 16.0;
                    let side_panel_w = 320.0;
                    let video_right = video_surface::video_right(
                        size.width,
                        self.state.ai_panel_open,
                        self.state.drawer.is_some(),
                        side_panel_w,
                    );
                    let ctrl_y = size.height - controls_h;
                    let seekbar_y = ctrl_y + 4.0;
                    let seekbar_w = video_right - seekbar_margin * 2.0;
                    let seekbar_rect = Rect::new(
                        seekbar_margin,
                        seekbar_y - 6.0,
                        seekbar_margin + seekbar_w,
                        seekbar_y + 12.0,
                    );
                    if seekbar_rect.contains(kurbo::Point::new(e.pos.x, e.pos.y))
                        && self.state.duration > 0.0
                    {
                        let ratio = controls::seek_ratio(e.pos.x, seekbar_margin, video_right);
                        self.seek_hover_pos = Some(ratio);
                        // Generate thumbnail for this position
                        if let Some(ref mut backend) = self.backend {
                            let thumb_pos = self.state.duration * ratio;
                            if let Some(pixels) = backend.generate_thumbnail(thumb_pos) {
                                self.seek_thumbnail = Some(tench_ui::peniko::ImageData {
                                    data: pixels.into(),
                                    format: tench_ui::peniko::ImageFormat::Rgba8,
                                    alpha_type:
                                        tench_ui::peniko::ImageAlphaType::AlphaPremultiplied,
                                    width: 160,
                                    height: 90,
                                });
                            }
                        }
                        ctx.request_paint();
                    } else if self.seek_hover_pos.is_some() {
                        self.seek_hover_pos = None;
                        self.seek_thumbnail = None;
                        ctx.request_paint();
                    }
                }
            }
            PointerEvent::Scroll(e) => {
                let size = ctx.state.size;
                let controls_h = 64.0;
                let overlay_h = 40.0;
                let side_panel_w = 320.0;
                let video_right = video_surface::video_right(
                    size.width,
                    self.state.ai_panel_open,
                    self.state.drawer.is_some(),
                    side_panel_w,
                );

                if e.pos.x > video_right {
                    // Scroll in drawer panel
                    self.state.drawer_scroll_y =
                        (self.state.drawer_scroll_y + e.delta.y * 20.0).max(0.0);
                    ctx.request_paint();
                } else if e.pos.y > overlay_h && e.pos.y < size.height - controls_h {
                    // Mouse wheel controls volume when hovering over video area
                    let delta = e.delta.y;
                    if delta > 0.0 {
                        self.state.set_volume(self.state.volume + 0.05);
                    } else if delta < 0.0 {
                        self.state.set_volume(self.state.volume - 0.05);
                    }
                    if let Some(ref mut backend) = self.backend {
                        backend.set_volume(self.state.volume);
                    }
                    ctx.request_paint();
                }
            }
            PointerEvent::Up(_) => {
                self.dragging_seek = false;
                self.dragging_volume = false;
            }
            PointerEvent::Enter | PointerEvent::Leave => {}
        }
    }

    fn on_window_event(&mut self, ctx: &mut EventCtx, event: &WindowEvent) {
        if let WindowEvent::AnimFrame(_ts) = event {
            self.process_dialog_results();
            self.process_backend_events();

            if self.backend.is_none() && self.state.is_playing && self.state.duration > 0.0 {
                let next_time = self.state.current_time + (1.0 / 30.0);
                if let Some((a, b)) = self.state.ab_loop {
                    self.state
                        .seek_to(if next_time > b { a.max(0.0) } else { next_time });
                } else {
                    self.state.seek_to(next_time.min(self.state.duration));
                }
            }

            // If playing, request next animation frame for continuous rendering
            if self.state.is_playing {
                ctx.request_anim_frame();
            }

            ctx.request_paint();
        }
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        if let TextEvent::Keyboard(kb) = event {
            if kb.is_pressed {
                match &kb.logical_key {
                    LogicalKey::Named(NamedKey::Space) => {
                        self.dispatch_click_action(&ClickAction::PlayPause, ctx);
                    }
                    LogicalKey::Character(c) if c == " " => {
                        self.dispatch_click_action(&ClickAction::PlayPause, ctx);
                    }
                    LogicalKey::Named(NamedKey::ArrowLeft) => {
                        self.dispatch_click_action(&ClickAction::SeekRelative(-5.0), ctx);
                    }
                    LogicalKey::Named(NamedKey::ArrowRight) => {
                        self.dispatch_click_action(&ClickAction::SeekRelative(5.0), ctx);
                    }
                    LogicalKey::Named(NamedKey::ArrowUp) => {
                        self.state.set_volume(self.state.volume + 0.05);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::ArrowDown) => {
                        self.state.set_volume(self.state.volume - 0.05);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Escape) => {
                        // Close any open modal first
                        if self.state.help_open
                            || self.state.url_input_open
                            || self.state.subtitle_style_open
                            || self.state.subtitle_search_open
                            || self.state.gif_options_open
                            || self.state.eq_open
                            || self.state.show_add_chapter_modal
                        {
                            self.state.help_open = false;
                            self.state.url_input_open = false;
                            self.state.subtitle_style_open = false;
                            self.state.subtitle_search_open = false;
                            self.state.gif_options_open = false;
                            self.state.eq_open = false;
                            self.state.show_add_chapter_modal = false;
                            self.state.ai_focused = false;
                            self.state.url_input_focused = false;
                            self.state.subtitle_search_focused = false;
                            self.state.chapter_name_input_focused = false;
                        } else {
                            self.state.close_all_panels();
                        }
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Backspace) => {
                        if self.state.ai_focused {
                            self.state.ai_input_text.pop();
                            ctx.request_paint();
                        } else if self.state.url_input_focused {
                            self.state.url_input_text.pop();
                            ctx.request_paint();
                        } else if self.state.subtitle_search_focused {
                            self.state.subtitle_search_text.pop();
                            ctx.request_paint();
                        } else if self.state.chapter_name_input_focused {
                            self.state.chapter_name_input.pop();
                            ctx.request_paint();
                        }
                    }
                    LogicalKey::Named(NamedKey::Enter) => {
                        if self.state.ai_focused {
                            let text = std::mem::take(&mut self.state.ai_input_text);
                            if !text.is_empty() {
                                self.dispatch_click_action(&ClickAction::SendAiPrompt(text), ctx);
                            }
                        } else if self.state.url_input_focused {
                            self.dispatch_click_action(&ClickAction::SubmitUrl, ctx);
                        } else if self.state.subtitle_search_focused {
                            self.dispatch_click_action(&ClickAction::SearchSubtitleNext, ctx);
                        } else if self.state.chapter_name_input_focused {
                            self.dispatch_click_action(&ClickAction::ConfirmAddChapter, ctx);
                        }
                    }
                    LogicalKey::Character(c) => {
                        // When a text input is focused, route all character input there
                        if self.state.ai_focused {
                            self.state.ai_input_text.push_str(c);
                            ctx.request_paint();
                        } else if self.state.url_input_focused {
                            self.state.url_input_text.push_str(c);
                            ctx.request_paint();
                        } else if self.state.subtitle_search_focused {
                            self.state.subtitle_search_text.push_str(c);
                            ctx.request_paint();
                        } else if self.state.chapter_name_input_focused {
                            self.state.chapter_name_input.push_str(c);
                            ctx.request_paint();
                        } else {
                            // Shortcut keys (only when no text input is focused)
                            match c.as_str() {
                                "m" => self.dispatch_click_action(&ClickAction::ToggleMute, ctx),
                                "a" => self.dispatch_click_action(&ClickAction::ToggleAiPanel, ctx),
                                "t" => self.dispatch_click_action(&ClickAction::ToggleTheme, ctx),
                                "[" => self.dispatch_click_action(&ClickAction::ToggleABLoop, ctx),
                                "b" => {
                                    self.state.add_bookmark();
                                    ctx.request_paint();
                                }
                                "g" => {
                                    self.dispatch_click_action(&ClickAction::ToggleGifCapture, ctx)
                                }
                                "f" => self.dispatch_click_action(&ClickAction::Fullscreen, ctx),
                                "v" => {
                                    self.state.cycle_aspect();
                                    ctx.request_paint();
                                }
                                "r" => {
                                    self.state.cycle_repeat_mode();
                                    ctx.request_paint();
                                }
                                "s" => {
                                    self.state.toggle_shuffle();
                                    ctx.request_paint();
                                }
                                "," => self.dispatch_click_action(&ClickAction::StepBackward, ctx),
                                "." => self.dispatch_click_action(&ClickAction::StepForward, ctx),
                                "z" => self
                                    .dispatch_click_action(&ClickAction::SubtitleOffset(-100), ctx),
                                "x" => self
                                    .dispatch_click_action(&ClickAction::SubtitleOffset(100), ctx),
                                "+" => {
                                    self.state.subtitle_font_size =
                                        (self.state.subtitle_font_size + 2.0).min(48.0);
                                    self.state.show_toast(format!(
                                        "Subtitle size: {:.0}",
                                        self.state.subtitle_font_size
                                    ));
                                    ctx.request_paint();
                                }
                                "-" => {
                                    self.state.subtitle_font_size =
                                        (self.state.subtitle_font_size - 2.0).max(10.0);
                                    self.state.show_toast(format!(
                                        "Subtitle size: {:.0}",
                                        self.state.subtitle_font_size
                                    ));
                                    ctx.request_paint();
                                }
                                "1" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.1), ctx),
                                "2" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.2), ctx),
                                "3" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.3), ctx),
                                "4" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.4), ctx),
                                "5" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.5), ctx),
                                "6" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.6), ctx),
                                "7" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.7), ctx),
                                "8" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.8), ctx),
                                "9" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.9), ctx),
                                "0" => self
                                    .dispatch_click_action(&ClickAction::SeekToPercent(0.0), ctx),
                                "?" | "/" if kb.modifiers.shift => {
                                    self.state.help_open = !self.state.help_open;
                                    ctx.request_paint();
                                }
                                "?" | "/" => {}
                                "l" if kb.modifiers.control => {
                                    self.state.url_input_open = !self.state.url_input_open;
                                    ctx.request_paint();
                                }
                                "l" => {}
                                "j" => {
                                    if let Some(t) = self.state.jump_prev_subtitle() {
                                        if let Some(ref mut backend) = self.backend {
                                            backend.seek(t);
                                        }
                                        self.state.current_time = t;
                                        ctx.request_paint();
                                    }
                                }
                                "k" => {
                                    if let Some(t) = self.state.jump_next_subtitle() {
                                        if let Some(ref mut backend) = self.backend {
                                            backend.seek(t);
                                        }
                                        self.state.current_time = t;
                                        ctx.request_paint();
                                    }
                                }
                                "c" => {
                                    self.state.cycle_subtitle_track();
                                    ctx.request_paint();
                                }
                                "n" => self.dispatch_click_action(&ClickAction::PlayNext, ctx),
                                "p" => self.dispatch_click_action(&ClickAction::PlayPrevious, ctx),
                                "o" => {
                                    self.open_file_dialog();
                                }
                                "d" => {
                                    self.state.normalization_enabled =
                                        !self.state.normalization_enabled;
                                    self.state.show_toast(
                                        if self.state.normalization_enabled {
                                            "Audio normalization: ON"
                                        } else {
                                            "Audio normalization: OFF"
                                        }
                                        .to_string(),
                                    );
                                    ctx.request_paint();
                                }
                                "i" => {
                                    self.state.pip_mode = !self.state.pip_mode;
                                    ctx.request_paint();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
