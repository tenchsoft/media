use super::*;
use tench_ui::core::events::{LogicalKey, NamedKey};
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::UiAutomationNode;

impl Widget for ComposerApp {
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

        self.process_dialog_results();
        self.state.check_notice_expiry();
        self.state.check_auto_save();
        self.clear_click_regions();

        p.fill_background(size, theme.background);

        // Collect click regions into a local buffer, then merge after painting.
        // This avoids the closure-captures-self borrow conflict.
        let mut regions: Vec<(Rect, ClickAction)> = Vec::new();
        let mut register = |rect: Rect, action: ClickAction| {
            regions.push((rect, action));
        };

        toolbar::paint_toolbar(
            &mut p,
            &self.state,
            size,
            theme,
            &mut register,
            &mut self.text_cache,
        );
        left_panel::paint_left_panel(
            &mut p,
            &self.state,
            size,
            theme,
            &mut register,
            &mut self.text_cache,
        );
        preview_panel::paint_preview(&mut p, &self.state, size, theme, &mut self.text_cache);
        right_panel::paint_right_panel(
            &mut p,
            &self.state,
            size,
            theme,
            &mut register,
            &mut self.text_cache,
        );
        timeline_panel::paint_timeline(
            &mut p,
            &self.state,
            size,
            theme,
            &mut register,
            &mut self.text_cache,
        );
        timeline_panel::paint_render_queue(
            &mut p,
            &self.state,
            size,
            theme,
            &mut register,
            &mut self.text_cache,
        );
        timeline_panel::paint_ai_panel(
            &mut p,
            &self.state,
            size,
            theme,
            &mut register,
            &mut self.text_cache,
        );
        timeline_panel::paint_quick_actions(
            &mut p,
            &self.state,
            size,
            theme,
            &mut register,
            &mut self.text_cache,
        );
        paint_context_menu(&mut p, &self.state, theme, &mut self.text_cache);

        // Merge collected regions into the widget's click_regions list
        for (rect, action) in regions {
            self.click_regions.push(ClickRegion { rect, action });
        }
    }

    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::Down(e) => {
                // Right-click context menu
                if e.button == tench_ui::core::events::PointerButton::Secondary {
                    if let Some(ClickAction::SelectClip(Some(clip_id))) =
                        self.click_action_at(e.pos).cloned()
                    {
                        self.state.selected_clip_id = Some(clip_id);
                        self.state.context_menu = Some(state::ContextMenuState {
                            x: e.pos.x,
                            y: e.pos.y,
                            items: vec![
                                state::ContextMenuItem {
                                    label: "Cut".into(),
                                    action: ClickAction::CutClip(clip_id),
                                    enabled: true,
                                },
                                state::ContextMenuItem {
                                    label: "Copy".into(),
                                    action: ClickAction::CopyClip(clip_id),
                                    enabled: true,
                                },
                                state::ContextMenuItem {
                                    label: "Paste".into(),
                                    action: ClickAction::PasteClip,
                                    enabled: self.state.clipboard.clip.is_some(),
                                },
                                state::ContextMenuItem {
                                    label: "Duplicate".into(),
                                    action: ClickAction::DuplicateClip(clip_id),
                                    enabled: true,
                                },
                                state::ContextMenuItem {
                                    label: "Delete".into(),
                                    action: ClickAction::DeleteClip(clip_id),
                                    enabled: true,
                                },
                            ],
                        });
                        ctx.request_paint();
                        return;
                    }
                    return;
                }

                // Left click
                if e.button == tench_ui::core::events::PointerButton::Primary {
                    // Check context menu clicks first
                    if let Some(ref menu) = self.state.context_menu {
                        let mut item_y = menu.y;
                        let mut clicked_action: Option<ClickAction> = None;
                        for item in &menu.items {
                            let item_rect =
                                Rect::new(menu.x, item_y, menu.x + 160.0, item_y + 24.0);
                            if item_rect.contains(e.pos) && item.enabled {
                                clicked_action = Some(item.action.clone());
                                break;
                            }
                            item_y += 24.0;
                        }
                        if let Some(action) = clicked_action {
                            self.state.context_menu = None;
                            self.dispatch_click_action(&action);
                            ctx.request_paint();
                            return;
                        }
                        // Clicked outside menu: dismiss
                        self.state.context_menu = None;
                        ctx.request_paint();
                        return;
                    }

                    if let Some(action) = self.click_action_at(e.pos).cloned() {
                        self.dispatch_click_action(&action);
                        ctx.request_paint();
                        return;
                    }

                    // Start drag or seek
                    let size = ctx.state.size;
                    self.handle_pointer_down(e.pos, size);

                    // Timeline click for seeking
                    let left_w = self.state.left_panel_w;
                    let timeline_h = self.state.timeline_h;
                    let tl_y = size.height - timeline_h;
                    if e.pos.y >= tl_y && e.pos.x >= left_w {
                        let tl_content_w = timeline::content_width(size.width, left_w);
                        let total_frames = self.state.total_frames();
                        let frame =
                            timeline::x_to_frame(e.pos.x, left_w, tl_content_w, total_frames);
                        self.state.seek_to_frame(frame);
                        if let Some(track_idx) = timeline::hit_test_track(
                            e.pos,
                            tl_y,
                            timeline_h,
                            self.state.tracks().len(),
                        ) {
                            self.state.select_clip_at_frame(track_idx);
                        }
                        ctx.request_paint();
                        return;
                    }

                    // Preview click for play/pause
                    let right_w = self.state.right_panel_w;
                    let center_right = size.width - right_w;
                    if preview::hit_test_preview(e.pos, left_w, center_right, 48.0, tl_y) {
                        self.state.toggle_playback();
                        ctx.request_paint();
                    }
                }
            }
            PointerEvent::Move(e) => {
                let size = ctx.state.size;
                if self.state.drag.is_some() {
                    self.handle_pointer_move(e.pos, size);
                    ctx.request_paint();
                }
            }
            PointerEvent::Up(e)
                if e.button == tench_ui::core::events::PointerButton::Primary
                    && self.state.drag.is_some() =>
            {
                let size = ctx.state.size;
                self.handle_pointer_up(e.pos, size);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        if matches!(
            self.state.input_focus,
            ComposerInputFocus::EffectsSearch | ComposerInputFocus::TransitionsSearch
        ) {
            if let TextEvent::Keyboard(kb) = event {
                if kb.is_pressed {
                    match &kb.logical_key {
                        LogicalKey::Named(NamedKey::Escape) => {
                            self.state.input_focus = ComposerInputFocus::None;
                            ctx.request_paint();
                            return;
                        }
                        LogicalKey::Named(NamedKey::Backspace) => {
                            match self.state.input_focus {
                                ComposerInputFocus::EffectsSearch => {
                                    self.state.effects_search.pop();
                                }
                                ComposerInputFocus::TransitionsSearch => {
                                    self.state.transitions_search.pop();
                                }
                                _ => {}
                            }
                            ctx.request_paint();
                            return;
                        }
                        LogicalKey::Character(c) if !kb.modifiers.control && !kb.modifiers.alt => {
                            match self.state.input_focus {
                                ComposerInputFocus::EffectsSearch => {
                                    self.state.effects_search.push_str(c);
                                }
                                ComposerInputFocus::TransitionsSearch => {
                                    self.state.transitions_search.push_str(c);
                                }
                                _ => {}
                            }
                            ctx.request_paint();
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Subtitle editor keyboard input routing (Phase 9)
        if self.state.subtitle_focused {
            if let TextEvent::Keyboard(kb) = event {
                if kb.is_pressed {
                    match &kb.logical_key {
                        LogicalKey::Named(NamedKey::Escape) => {
                            self.state.subtitle_focused = false;
                            ctx.request_paint();
                            return;
                        }
                        LogicalKey::Named(NamedKey::Backspace) => {
                            self.state.subtitle_text.pop();
                            ctx.request_paint();
                            return;
                        }
                        LogicalKey::Named(NamedKey::Enter) => {
                            self.state.subtitle_text.push('\n');
                            ctx.request_paint();
                            return;
                        }
                        _ => {}
                    }
                }
            }
            if let TextEvent::Ime(tench_ui::core::events::ImeEvent::Commit(text)) = event {
                self.state.subtitle_text.push_str(text);
                ctx.request_paint();
                return;
            }
            if let TextEvent::Keyboard(kb) = event {
                if kb.is_pressed {
                    if let LogicalKey::Character(c) = &kb.logical_key {
                        if !kb.modifiers.control && !kb.modifiers.alt {
                            self.state.subtitle_text.push_str(c);
                            ctx.request_paint();
                            return;
                        }
                    }
                }
            }
        }

        if let TextEvent::Keyboard(kb) = event {
            if kb.is_pressed {
                match &kb.logical_key {
                    LogicalKey::Named(NamedKey::Space) => {
                        self.state.toggle_playback();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == " " => {
                        self.state.toggle_playback();
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::ArrowLeft) => {
                        self.state.step_frame(-1);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::ArrowRight) => {
                        self.state.step_frame(1);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::ArrowUp) => {
                        self.state.step_frame(-24);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::ArrowDown) => {
                        self.state.step_frame(24);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Delete)
                    | LogicalKey::Named(NamedKey::Backspace)
                        if !self.state.subtitle_focused =>
                    {
                        self.state.delete_selected_clip();
                        ctx.request_paint();
                    }
                    // JKL shuttle (Phase 3): J = reverse with acceleration, K = stop, L = forward with acceleration
                    LogicalKey::Character(c) if c == "j" => {
                        self.state.shuttle_reverse();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "k" => {
                        self.state.shuttle_stop();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "l" => {
                        self.state.shuttle_forward();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "c" => {
                        self.state.split_at_playhead();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "s" && !kb.modifiers.control => {
                        self.state.snap = !self.state.snap;
                        self.state.set_notice(format!(
                            "Snap: {}",
                            if self.state.snap { "ON" } else { "OFF" }
                        ));
                        ctx.request_paint();
                    }
                    // Phase 7: Ctrl+S actually saves the project
                    LogicalKey::Character(c) if c == "s" && kb.modifiers.control => {
                        if self.state.save_path.is_some() {
                            self.state.save_project();
                        } else {
                            // No save path set — request a save dialog.
                            // In Tauri mode, the commands.rs open_media_dialog pattern
                            // handles this. For now, save to a default location.
                            let default_path = format!("{}.composer", self.state.project.name);
                            self.state.save_project_as(default_path);
                        }
                        ctx.request_paint();
                    }
                    // Phase 2: Undo/Redo with actual snapshot history
                    LogicalKey::Character(c)
                        if c == "z" && kb.modifiers.control && !kb.modifiers.shift =>
                    {
                        self.state.undo();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c)
                        if c == "z" && kb.modifiers.control && kb.modifiers.shift =>
                    {
                        self.state.redo();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "i" && kb.modifiers.control => {
                        // Import via keyboard shortcut — opens real file dialog
                        request_media_import();
                    }
                    LogicalKey::Character(c) if c == "m" && kb.modifiers.control => {
                        self.state.enqueue_render();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "+" || c == "=" => {
                        self.state.zoom = (self.state.zoom + 10.0).min(200.0);
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "-" => {
                        self.state.zoom = (self.state.zoom - 10.0).max(10.0);
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "a" => {
                        self.state.show_ai_panel = !self.state.show_ai_panel;
                        ctx.request_paint();
                    }
                    // Phase 3: In/Out points with actual state
                    LogicalKey::Character(c) if c == "i" && !kb.modifiers.control => {
                        self.state.in_point = Some(self.state.current_frame);
                        self.state.set_notice(format!(
                            "In-point set at frame {}",
                            self.state.current_frame
                        ));
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c) if c == "o" => {
                        self.state.out_point = Some(self.state.current_frame);
                        self.state.set_notice(format!(
                            "Out-point set at frame {}",
                            self.state.current_frame
                        ));
                        ctx.request_paint();
                    }
                    // Phase 3: Loop playback toggle
                    LogicalKey::Character(c) if c == "b" => {
                        self.state.loop_playback = !self.state.loop_playback;
                        self.state.set_notice(format!(
                            "Loop: {}",
                            if self.state.loop_playback {
                                "ON"
                            } else {
                                "OFF"
                            }
                        ));
                        ctx.request_paint();
                    }
                    _ => {}
                }
            }
        }
    }

    fn on_window_event(&mut self, ctx: &mut EventCtx, event: &WindowEvent) {
        if let WindowEvent::AnimFrame(_) = event {
            if self.state.is_playing {
                self.state.advance_playback();
                ctx.request_paint();
            }
        }
    }

    fn debug_id(&self) -> Option<&str> {
        Some("composer.root")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn automation_children(&self, state: &WidgetState) -> Vec<UiAutomationNode> {
        automation::composer_automation_nodes(
            &self.state,
            &self.click_regions,
            state.size,
            state.id.to_raw(),
        )
    }
}

fn paint_context_menu(
    p: &mut Painter<'_>,
    state: &ComposerState,
    theme: &Theme,
    text_cache: &mut TextCache,
) {
    let Some(menu) = &state.context_menu else {
        return;
    };
    let rect = Rect::new(
        menu.x,
        menu.y,
        menu.x + 160.0,
        menu.y + menu.items.len() as f64 * 24.0,
    );
    p.fill_rounded_rect(rect, theme.surface, theme.border_radius);
    p.stroke_rounded_rect(rect, theme.border, 1.0, theme.border_radius);
    let mut y = menu.y;
    for item in &menu.items {
        let row = Rect::new(menu.x, y, menu.x + 160.0, y + 24.0);
        p.fill_rect(row, theme.background);
        p.draw_text_cached(
            text_cache,
            &item.label,
            row.x0 + 10.0,
            row.y0 + 16.0,
            if item.enabled {
                theme.on_surface
            } else {
                theme.disabled
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
            false,
        );
        y += 24.0;
    }
}
