use super::*;

impl Widget for PixelDesignApp {
    fn measure(&mut self, _ctx: &mut MeasureCtx, _axis: Axis, available: f64) -> f64 {
        available
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _size: Size) {}

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let size = ctx.size();
        let theme = ctx.theme();
        let mut p = Painter::new(scene);

        p.fill_background(size, theme.background);
        paint_top_bar(&self.state, &mut p, theme, size);
        paint_tool_strip(
            &self.state,
            &mut p,
            theme,
            TOP_BAR_H,
            TOOL_STRIP_W,
            size.height,
        );

        // Draw the canvas viewport (includes composited image rendering)
        let viewport = Rect::new(
            TOOL_STRIP_W,
            TOP_BAR_H,
            size.width - RIGHT_PANEL_W,
            size.height - STATUS_BAR_H,
        );
        paint_canvas_viewport(
            &self.state,
            &mut p,
            theme,
            viewport,
            Rect::new(
                TOOL_STRIP_W,
                size.height - STATUS_BAR_H,
                size.width - RIGHT_PANEL_W,
                size.height,
            ),
        );

        let panel = Rect::new(
            size.width - RIGHT_PANEL_W,
            TOP_BAR_H,
            size.width,
            size.height,
        );
        match self.state.persona {
            Persona::Edit => paint_edit_panel(&self.state, &mut p, theme, panel),
            Persona::AI => paint_ai_panel(&self.state, &mut p, theme, panel),
            Persona::Adjust => paint_adjust_panel(&self.state, &mut p, theme, panel),
            Persona::Export => paint_export_panel(&self.state, &mut p, theme, panel),
        }

        if self.state.show_color_picker {
            color_picker::paint_color_picker_modal(&self.state, &mut p, theme, size);
        }
    }

    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        if self.handle_color_picker_event(ctx, event) {
            ctx.request_paint();
            return;
        }

        if let PointerEvent::Down(e) = event {
            let size = ctx.state.size;
            let handled = self.handle_top_bar_click(e.pos.x, e.pos.y, size.width)
                || self.handle_tool_strip_click(e.pos.x, e.pos.y)
                || self.handle_right_panel_click(e.pos.x, e.pos.y, size.width)
                || self.handle_status_bar_click(e.pos.x, e.pos.y, size.width, size.height);
            if handled {
                ctx.request_paint();
                return;
            }
        }

        if self.handle_canvas_event(ctx, event) {
            ctx.request_paint();
        }
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        let TextEvent::Keyboard(kb) = event else {
            return;
        };
        if !kb.is_pressed {
            return;
        }

        match &kb.logical_key {
            LogicalKey::Character(c) if kb.modifiers.control && c.eq_ignore_ascii_case("z") => {
                if kb.modifiers.shift {
                    self.state.redo();
                } else {
                    self.state.undo();
                }
                self.refresh_flattened();
                ctx.request_paint();
            }
            LogicalKey::Character(c) if kb.modifiers.control && c.eq_ignore_ascii_case("y") => {
                self.state.redo();
                self.refresh_flattened();
                ctx.request_paint();
            }
            LogicalKey::Character(c) if kb.modifiers.control && c.eq_ignore_ascii_case("s") => {
                self.save_document();
                ctx.request_paint();
            }
            LogicalKey::Character(c) if kb.modifiers.control && c.eq_ignore_ascii_case("o") => {
                self.state.status_msg = "Open image file".into();
                self.state.pending_file_action = Some(state::FileAction::Open);
                ctx.request_paint();
            }
            LogicalKey::Character(c) if kb.modifiers.control && c.eq_ignore_ascii_case("g") => {
                self.state.toggle_grid();
                ctx.request_paint();
            }
            LogicalKey::Character(c) if kb.modifiers.control && c.eq_ignore_ascii_case("r") => {
                self.state.toggle_rulers();
                ctx.request_paint();
            }
            // Phase 7: Ctrl+0 fit, Ctrl+1 100%
            LogicalKey::Character(c) if kb.modifiers.control && c == "0" => {
                self.state.zoom_fit();
                ctx.request_paint();
            }
            LogicalKey::Character(c) if kb.modifiers.control && c == "1" => {
                self.state.zoom_actual();
                ctx.request_paint();
            }
            LogicalKey::Character(c) if c == "+" || c == "=" => {
                self.state.zoom_in();
                ctx.request_paint();
            }
            LogicalKey::Character(c) if c == "-" => {
                self.state.zoom_out();
                ctx.request_paint();
            }
            // AI prompt input
            LogicalKey::Character(c) if self.state.ai_prompt_focused => {
                self.state.ai_prompt.push_str(c);
                ctx.request_paint();
            }
            // Text input
            LogicalKey::Character(c) if self.state.show_text_input => {
                self.state.text_input.push_str(c);
                ctx.request_paint();
            }
            // Phase 7: [ and ] for brush size (must be before catch-all)
            LogicalKey::Character(c)
                if !kb.modifiers.control
                    && !self.state.show_text_input
                    && !self.state.ai_prompt_focused
                    && c == "[" =>
            {
                self.state.brush_size = self.state.brush_size.saturating_sub(4).max(1);
                self.state.status_msg = format!("Brush size: {}", self.state.brush_size);
                ctx.request_paint();
            }
            LogicalKey::Character(c)
                if !kb.modifiers.control
                    && !self.state.show_text_input
                    && !self.state.ai_prompt_focused
                    && c == "]" =>
            {
                self.state.brush_size = (self.state.brush_size + 4).min(200);
                self.state.status_msg = format!("Brush size: {}", self.state.brush_size);
                ctx.request_paint();
            }
            // Phase 7: Number keys for brush opacity (1=10% ... 0=100%)
            LogicalKey::Character(c)
                if !kb.modifiers.control
                    && !self.state.show_text_input
                    && !self.state.ai_prompt_focused =>
            {
                if let Some(digit) = c.chars().next().and_then(|ch| ch.to_digit(10)) {
                    if let Some(tool) = Tool::from_shortcut(c) {
                        self.state.set_active_tool(tool);
                    } else {
                        let opacity = if digit == 0 { 100 } else { digit * 10 };
                        self.state.brush_opacity = opacity;
                        self.state.status_msg = format!("Brush opacity: {opacity}%");
                    }
                    ctx.request_paint();
                } else if let Some(tool) = Tool::from_shortcut(c) {
                    self.state.set_active_tool(tool);
                    ctx.request_paint();
                }
            }
            LogicalKey::Character(c) => {
                if let Some(tool) = Tool::from_shortcut(c) {
                    self.state.set_active_tool(tool);
                    ctx.request_paint();
                }
            }
            LogicalKey::Named(NamedKey::Enter) if self.state.show_text_input => {
                self.state.commit_text_input();
                self.refresh_flattened();
                ctx.request_paint();
            }
            LogicalKey::Named(NamedKey::Enter) if self.state.ai_prompt_focused => {
                self.state.ai_prompt_focused = false;
                ctx.request_paint();
            }
            LogicalKey::Named(NamedKey::Escape) => {
                self.state.cancel_modal_action();
                ctx.request_paint();
            }
            LogicalKey::Named(NamedKey::Backspace) if self.state.show_text_input => {
                self.state.text_input.pop();
                ctx.request_paint();
            }
            LogicalKey::Named(NamedKey::Backspace) if self.state.ai_prompt_focused => {
                self.state.ai_prompt.pop();
                ctx.request_paint();
            }
            // Phase 7: Space for temporary pan
            LogicalKey::Named(NamedKey::Space) => {
                self.state.space_held = true;
                ctx.request_paint();
            }
            // Phase 7: Tab for panel toggle
            LogicalKey::Named(NamedKey::Tab) => {
                self.state.toggle_panels();
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn on_window_event(&mut self, ctx: &mut EventCtx, event: &WindowEvent) {
        if let WindowEvent::Resize { .. } = event {
            ctx.request_paint();
        }
    }

    fn debug_id(&self) -> Option<&str> {
        Some("pixel-design.root")
    }

    fn accessibility_tree(&self, _state: &WidgetState) -> AccessibilityNode {
        AccessibilityNode {
            role: AccessRole::Unknown,
            label: Some("Pixel Design".into()),
            value: None,
            focused: true,
            disabled: false,
            children: Vec::new(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn automation_children(&self, state: &WidgetState) -> Vec<tench_ui::UiAutomationNode> {
        let size = state.size;
        let mut nodes = Vec::new();

        // Top bar persona buttons
        for (idx, persona) in Persona::ALL.iter().enumerate() {
            let rect = Rect::new(
                12.0 + idx as f64 * 70.0,
                8.0,
                76.0 + idx as f64 * 70.0,
                40.0,
            );
            nodes.push(make_btn(
                format!("pd.top.persona.{}", persona.label().to_lowercase()),
                persona.label(),
                rect,
            ));
        }

        // Top bar action buttons (Undo, Redo, Open, Save)
        let action_x = size.width - 148.0;
        for (idx, label) in ["undo", "redo", "open", "save"].iter().enumerate() {
            let x = action_x + idx as f64 * 34.0;
            let rect = Rect::new(x, 9.0, x + 28.0, 37.0);
            nodes.push(make_btn(format!("pd.top.{}", label), label, rect));
        }

        // Tool strip buttons (Edit persona)
        if self.state.persona == Persona::Edit {
            for (idx, tool) in Tool::ALL.iter().enumerate() {
                let rect = Rect::new(
                    6.0,
                    TOP_BAR_H + 8.0 + idx as f64 * 42.0,
                    42.0,
                    TOP_BAR_H + 44.0 + idx as f64 * 42.0,
                );
                nodes.push(make_btn(
                    format!("pd.tool.{}", tool.label().to_lowercase()),
                    tool.label(),
                    rect,
                ));
            }
        } else if self.state.persona == Persona::AI {
            for (idx, tool) in AiTool::ALL.iter().enumerate() {
                let rect = Rect::new(
                    6.0,
                    TOP_BAR_H + 8.0 + idx as f64 * 46.0,
                    42.0,
                    TOP_BAR_H + 44.0 + idx as f64 * 46.0,
                );
                nodes.push(make_btn(
                    format!(
                        "pd.ai_tool.{}",
                        tool.label().to_lowercase().replace(' ', "_")
                    ),
                    tool.label(),
                    rect,
                ));
            }
        }

        // FG/BG color swatches
        let fg_rect = Rect::new(
            8.0,
            TOP_BAR_H + COLOR_FG_Y,
            36.0,
            TOP_BAR_H + COLOR_FG_Y + 28.0,
        );
        nodes.push(make_btn("pd.color.fg".into(), "FG", fg_rect));
        let bg_rect = Rect::new(
            16.0,
            TOP_BAR_H + COLOR_BG_Y,
            44.0,
            TOP_BAR_H + COLOR_BG_Y + 28.0,
        );
        nodes.push(make_btn("pd.color.bg".into(), "BG", bg_rect));
        for idx in 0..self.state.recent_colors.len().min(6) {
            let col = idx % 3;
            let row = idx / 3;
            let x = 4.0 + col as f64 * 14.0;
            let y = TOP_BAR_H + RECENT_COLOR_Y + row as f64 * 14.0;
            nodes.push(make_btn(
                format!("pd.color.recent.{}", idx + 1),
                &format!("Recent {}", idx + 1),
                Rect::new(x, y, x + 12.0, y + 12.0),
            ));
        }

        // Right panel buttons based on persona
        let panel_x = size.width - RIGHT_PANEL_W;
        match self.state.persona {
            Persona::Edit => {
                // Panel tabs
                for (idx, tab) in PanelTab::ALL.iter().enumerate() {
                    let rect = Rect::new(
                        panel_x + 10.0 + idx as f64 * 88.0,
                        TOP_BAR_H + 10.0,
                        panel_x + 92.0 + idx as f64 * 88.0,
                        TOP_BAR_H + 40.0,
                    );
                    nodes.push(make_btn(
                        format!("pd.tab.{}", tab.label().to_lowercase()),
                        tab.label(),
                        rect,
                    ));
                }
                match self.state.panel_tab {
                    PanelTab::Layers => {
                        let x = panel_x + 16.0;
                        nodes.push(make_btn(
                            "pd.layer.opacity".into(),
                            "Layer Opacity",
                            Rect::new(x, TOP_BAR_H + 92.0, panel_x + 268.0, TOP_BAR_H + 120.0),
                        ));
                        for idx in 0..self.state.document.layers.len() {
                            let y = TOP_BAR_H + 134.0 + idx as f64 * 42.0;
                            nodes.push(make_btn(
                                format!("pd.layer.row.{}", idx),
                                &format!("Layer {}", idx + 1),
                                Rect::new(panel_x + 56.0, y, panel_x + 238.0, y + 36.0),
                            ));
                            nodes.push(make_btn(
                                format!("pd.layer.visibility.{}", idx),
                                &format!("Layer {} Visibility", idx + 1),
                                Rect::new(panel_x + 14.0, y, panel_x + 46.0, y + 36.0),
                            ));
                            nodes.push(make_btn(
                                format!("pd.layer.lock.{}", idx),
                                &format!("Layer {} Lock", idx + 1),
                                Rect::new(panel_x + 240.0, y + 20.0, panel_x + 272.0, y + 36.0),
                            ));
                        }
                        nodes.push(make_btn(
                            "pd.layer.add".into(),
                            "+ Layer",
                            Rect::new(x, TOP_BAR_H + 262.0, x + 116.0, TOP_BAR_H + 294.0),
                        ));
                        nodes.push(make_btn(
                            "pd.layer.delete".into(),
                            "Delete",
                            Rect::new(x + 130.0, TOP_BAR_H + 262.0, x + 254.0, TOP_BAR_H + 294.0),
                        ));
                        nodes.push(make_btn(
                            "pd.layer.up".into(),
                            "Move Up",
                            Rect::new(x, TOP_BAR_H + 300.0, x + 116.0, TOP_BAR_H + 328.0),
                        ));
                        nodes.push(make_btn(
                            "pd.layer.down".into(),
                            "Move Down",
                            Rect::new(x + 130.0, TOP_BAR_H + 300.0, x + 254.0, TOP_BAR_H + 328.0),
                        ));
                        nodes.push(make_btn(
                            "pd.layer.dup".into(),
                            "Duplicate",
                            Rect::new(x, TOP_BAR_H + 336.0, x + 116.0, TOP_BAR_H + 364.0),
                        ));
                        nodes.push(make_btn(
                            "pd.layer.flatten".into(),
                            "Flatten",
                            Rect::new(x + 130.0, TOP_BAR_H + 336.0, x + 254.0, TOP_BAR_H + 364.0),
                        ));
                    }
                    PanelTab::Properties => {
                        let x = panel_x + 18.0;
                        nodes.push(make_btn(
                            "pd.props.size".into(),
                            "Size",
                            Rect::new(x, TOP_BAR_H + 92.0, panel_x + 268.0, TOP_BAR_H + 122.0),
                        ));
                        nodes.push(make_btn(
                            "pd.props.opacity".into(),
                            "Opacity",
                            Rect::new(x, TOP_BAR_H + 140.0, panel_x + 268.0, TOP_BAR_H + 170.0),
                        ));
                        nodes.push(make_btn(
                            "pd.props.hardness".into(),
                            "Hardness",
                            Rect::new(x, TOP_BAR_H + 188.0, panel_x + 268.0, TOP_BAR_H + 218.0),
                        ));
                    }
                    PanelTab::History => {
                        let x = panel_x + 20.0;
                        nodes.push(make_btn(
                            "pd.history.undo".into(),
                            "Undo",
                            Rect::new(x, TOP_BAR_H + 92.0, x + 110.0, TOP_BAR_H + 124.0),
                        ));
                        nodes.push(make_btn(
                            "pd.history.redo".into(),
                            "Redo",
                            Rect::new(x + 126.0, TOP_BAR_H + 92.0, x + 236.0, TOP_BAR_H + 124.0),
                        ));
                        for idx in 0..self.state.history.len().min(8) {
                            let y = TOP_BAR_H + 140.0 + idx as f64 * 20.0;
                            nodes.push(make_btn(
                                format!("pd.history.step.{}", idx),
                                &self.state.history[idx].label,
                                Rect::new(x, y, x + 180.0, y + 18.0),
                            ));
                        }
                    }
                }
                // Brush presets
                if self.state.active_tool.uses_brush() {
                    for (idx, preset) in PixelDesignState::brush_presets().iter().enumerate() {
                        let col = idx % 2;
                        let row = idx / 2;
                        let rect = Rect::new(
                            panel_x + 16.0 + col as f64 * 128.0,
                            TOP_BAR_H + 370.0 + row as f64 * 58.0,
                            panel_x + 132.0 + col as f64 * 128.0,
                            TOP_BAR_H + 420.0 + row as f64 * 58.0,
                        );
                        nodes.push(make_btn(
                            format!("pd.brush.{}", preset.id),
                            preset.name,
                            rect,
                        ));
                    }
                }
            }
            Persona::AI => {
                let x = panel_x + 18.0;
                nodes.push(make_btn(
                    "pd.ai.prompt".into(),
                    "AI Prompt",
                    Rect::new(x, TOP_BAR_H + 94.0, panel_x + 268.0, TOP_BAR_H + 148.0),
                ));
                nodes.push(make_btn(
                    "pd.ai.run".into(),
                    "Run AI Job",
                    Rect::new(x, TOP_BAR_H + 160.0, panel_x + 268.0, TOP_BAR_H + 196.0),
                ));
                nodes.push(make_btn(
                    "pd.ai.cancel".into(),
                    "Cancel",
                    Rect::new(x, TOP_BAR_H + 202.0, panel_x + 268.0, TOP_BAR_H + 230.0),
                ));
                for (idx, tool) in AiTool::ALL.iter().enumerate() {
                    let rect = Rect::new(
                        x,
                        TOP_BAR_H + 262.0 + idx as f64 * 34.0,
                        panel_x + 268.0,
                        TOP_BAR_H + 290.0 + idx as f64 * 34.0,
                    );
                    nodes.push(make_btn(
                        format!(
                            "pd.ai.panel.{}",
                            tool.label().to_lowercase().replace(' ', "_")
                        ),
                        tool.label(),
                        rect,
                    ));
                }
            }
            Persona::Adjust => {
                let x = panel_x + 18.0;
                for (idx, preset) in PixelDesignState::adjust_presets().iter().enumerate() {
                    let col = idx % 2;
                    let row = idx / 2;
                    let rect = Rect::new(
                        x + col as f64 * 124.0,
                        TOP_BAR_H + 60.0 + row as f64 * 38.0,
                        x + 114.0 + col as f64 * 124.0,
                        TOP_BAR_H + 90.0 + row as f64 * 38.0,
                    );
                    nodes.push(make_btn(
                        format!("pd.adjust.{}", preset.to_lowercase()),
                        preset,
                        rect,
                    ));
                }
                for idx in 0..8 {
                    let rect = Rect::new(
                        x,
                        TOP_BAR_H + 260.0 + idx as f64 * 40.0,
                        panel_x + 266.0,
                        TOP_BAR_H + 290.0 + idx as f64 * 40.0,
                    );
                    nodes.push(make_btn(
                        format!("pd.adjust.slider.{}", idx),
                        &format!("Slider {}", idx),
                        rect,
                    ));
                }
            }
            Persona::Export => {
                let x = panel_x + 18.0;
                nodes.push(make_btn(
                    "pd.export.format".into(),
                    "Format",
                    Rect::new(x, TOP_BAR_H + 58.0, panel_x + 268.0, TOP_BAR_H + 90.0),
                ));
                nodes.push(make_btn(
                    "pd.export.quality".into(),
                    "Quality",
                    Rect::new(x, TOP_BAR_H + 116.0, panel_x + 268.0, TOP_BAR_H + 146.0),
                ));
                nodes.push(make_btn(
                    "pd.export.scale".into(),
                    "Scale",
                    Rect::new(x, TOP_BAR_H + 174.0, panel_x + 268.0, TOP_BAR_H + 204.0),
                ));
                nodes.push(make_btn(
                    "pd.export.button".into(),
                    "Export",
                    Rect::new(x, TOP_BAR_H + 310.0, panel_x + 268.0, TOP_BAR_H + 348.0),
                ));
            }
        }

        // Status bar zoom controls
        let zoom_x = size.width - RIGHT_PANEL_W - 142.0;
        let status_y = size.height - STATUS_BAR_H;
        nodes.push(make_btn(
            "pd.status.zoom_out".into(),
            "Zoom Out",
            Rect::new(zoom_x, status_y, zoom_x + 28.0, status_y + STATUS_BAR_H),
        ));
        nodes.push(make_btn(
            "pd.status.zoom_slider".into(),
            "Zoom Slider",
            Rect::new(
                zoom_x + 30.0,
                status_y,
                zoom_x + 96.0,
                status_y + STATUS_BAR_H,
            ),
        ));
        nodes.push(make_btn(
            "pd.status.zoom_in".into(),
            "Zoom In",
            Rect::new(
                zoom_x + 100.0,
                status_y,
                zoom_x + 128.0,
                status_y + STATUS_BAR_H,
            ),
        ));

        let viewport = Rect::new(
            TOOL_STRIP_W,
            TOP_BAR_H,
            size.width - RIGHT_PANEL_W,
            size.height - STATUS_BAR_H,
        );
        let doc = canvas_document_rect(&self.state, viewport);
        nodes.push(make_node("pd.canvas".into(), "canvas", "Canvas", None, doc));
        for id in [
            "brush_stroke",
            "eraser_stroke",
            "fill_click",
            "text_placement",
            "select_drag",
            "crop_drag",
            "gradient_drag",
            "shape_drag",
            "move_layer_drag",
            "hand_pan",
            "eyedropper_click",
        ] {
            nodes.push(make_node(
                format!("pd.canvas.{}", id),
                "canvas",
                id,
                None,
                doc,
            ));
        }

        nodes.push(make_node(
            "pd.auto.checkerboard".into(),
            "image",
            "Checkerboard Transparency",
            None,
            viewport,
        ));
        nodes.push(make_node(
            "pd.auto.status_bar".into(),
            "status",
            "Status Bar",
            Some(format!(
                "{} | x:{} y:{}",
                self.state.status_msg,
                self.state.mouse_pos.x.round(),
                self.state.mouse_pos.y.round()
            )),
            Rect::new(
                TOOL_STRIP_W,
                status_y,
                size.width - RIGHT_PANEL_W,
                size.height,
            ),
        ));
        nodes.push(make_node(
            "pd.auto.zoom_percent".into(),
            "status",
            "Zoom Percent",
            Some(format!("{}%", self.state.zoom)),
            Rect::new(
                zoom_x + 36.0,
                status_y,
                zoom_x + 96.0,
                status_y + STATUS_BAR_H,
            ),
        ));
        nodes.push(make_node(
            "pd.auto.active_control_highlight".into(),
            "status",
            "Active Control Highlight",
            Some(self.state.active_tool.label().to_string()),
            Rect::new(6.0, TOP_BAR_H + 8.0, 42.0, TOP_BAR_H + 44.0),
        ));
        nodes.push(make_node(
            "pd.auto.tool_context_chips".into(),
            "status",
            "Tool Context Chips",
            Some(self.state.active_tool.label().to_string()),
            Rect::new(470.0, 8.0, (size.width - 520.0).max(590.0), 40.0),
        ));
        nodes.push(make_node(
            "pd.auto.canvas_layout".into(),
            "layout",
            "Canvas Layout",
            Some(format!("{:.0}x{:.0}", doc.width(), doc.height())),
            doc,
        ));
        if self.state.composited_image.is_some() {
            nodes.push(make_node(
                "pd.auto.composited_canvas".into(),
                "image",
                "Composited Canvas",
                None,
                doc,
            ));
        }
        if !self.state.layer_thumbnails.is_empty() {
            nodes.push(make_node(
                "pd.auto.layer_thumbnail".into(),
                "image",
                "Layer Thumbnail",
                None,
                Rect::new(
                    panel_x + 56.0,
                    TOP_BAR_H + 140.0,
                    panel_x + 78.0,
                    TOP_BAR_H + 162.0,
                ),
            ));
        }
        if self.state.selection.is_some() {
            nodes.push(make_node(
                "pd.auto.selection_overlay".into(),
                "image",
                "Selection Overlay",
                None,
                doc,
            ));
        }
        if self.state.show_text_input {
            nodes.push(make_node(
                "pd.auto.text_input_overlay".into(),
                "textbox",
                "Text Input Overlay",
                Some(self.state.text_input.clone()),
                doc,
            ));
        }
        if self.state.show_rulers {
            nodes.push(make_node(
                "pd.auto.rulers".into(),
                "image",
                "Rulers",
                None,
                viewport,
            ));
        }
        if self.state.show_grid {
            nodes.push(make_node(
                "pd.auto.grid".into(),
                "image",
                "Grid",
                None,
                viewport,
            ));
        }
        if self.state.document.dirty {
            nodes.push(make_node(
                "pd.auto.dirty_dot".into(),
                "status",
                "Dirty Dot",
                None,
                Rect::new(size.width * 0.5 - 52.0, 9.0, size.width * 0.5 - 44.0, 17.0),
            ));
        }
        if !self.state.ai_jobs.is_empty() {
            nodes.push(make_node(
                "pd.auto.ai_job_list".into(),
                "list",
                "AI Job List",
                Some(self.state.ai_jobs.len().to_string()),
                Rect::new(
                    panel_x + 18.0,
                    TOP_BAR_H + 520.0,
                    panel_x + 268.0,
                    size.height,
                ),
            ));
        }

        if self.state.show_color_picker {
            let modal = Self::color_picker_modal(size);
            nodes.push(make_btn(
                "pd.color_picker.hue".into(),
                "Hue",
                Self::color_picker_hue_rect(modal),
            ));
            nodes.push(make_btn(
                "pd.color_picker.sv".into(),
                "Saturation Value",
                Self::color_picker_sv_rect(modal),
            ));
            nodes.push(make_btn(
                "pd.color_picker.apply".into(),
                "Apply",
                Self::color_picker_apply_rect(modal),
            ));
            nodes.push(make_btn(
                "pd.color_picker.cancel".into(),
                "Cancel",
                Self::color_picker_cancel_rect(modal),
            ));
        }

        nodes
    }
}

fn make_btn(debug_id: String, label: &str, rect: Rect) -> tench_ui::UiAutomationNode {
    make_node(debug_id, "button", label, None, rect)
}

fn make_node(
    debug_id: String,
    role: &str,
    label: &str,
    value: Option<String>,
    rect: Rect,
) -> tench_ui::UiAutomationNode {
    tench_ui::UiAutomationNode {
        id: 0,
        debug_id: Some(debug_id),
        role: role.to_string(),
        label: Some(label.to_string()),
        value,
        bounds: tench_ui::UiAutomationRect {
            x: rect.x0,
            y: rect.y0,
            width: rect.width(),
            height: rect.height(),
        },
        enabled: true,
        focused: false,
        hovered: false,
        children: Vec::new(),
    }
}
