// ---------------------------------------------------------------------------
// Click action dispatch: metadata
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::{self, ClickAction};

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_metadata_action(
        &mut self,
        action: &ClickAction,
        ctx: &mut EventCtx,
    ) -> bool {
        match action {
            ClickAction::SetRating(rating) => {
                self.state.image_rating = *rating;
                self.state.status_message = format!(
                    "Rated {} star{}",
                    rating,
                    if *rating != 1 { "s" } else { "" }
                );
                self.state.status_message_time = Some(std::time::Instant::now());
                ctx.request_paint();
                true
            }

            // --- Tags ---
            ClickAction::ToggleTag(tag) => {
                if let Some(pos) = self.state.image_tags.iter().position(|t| t == tag) {
                    self.state.image_tags.remove(pos);
                } else {
                    self.state.image_tags.push(tag.clone());
                }
                ctx.request_paint();
                true
            }

            // --- Print ---
            ClickAction::PrintImage => {
                if self.state.document.is_some() {
                    self.state.show_print_dialog = true;
                    self.state.status_message = "Print dialog opened".into();
                }
                self.state.status_message_time = Some(std::time::Instant::now());
                ctx.request_paint();
                true
            }
            ClickAction::PrintCancel => {
                self.state.show_print_dialog = false;
                ctx.request_paint();
                true
            }
            ClickAction::PrintSelectPaper(paper) => {
                self.state.print_paper_size = paper.clone();
                self.state.status_message = format!("Paper: {}", paper);
                ctx.request_paint();
                true
            }
            ClickAction::PrintSelectOrientation(orientation) => {
                self.state.print_orientation = orientation.clone();
                self.state.status_message = format!("Orientation: {}", orientation);
                ctx.request_paint();
                true
            }
            ClickAction::PrintSelectScaling(scaling) => {
                self.state.print_scaling = scaling.clone();
                self.state.status_message = format!("Scaling: {}", scaling);
                ctx.request_paint();
                true
            }

            // --- Slideshow shuffle ---
            ClickAction::SlideshowToggleShuffle => {
                self.state.slideshow_shuffle = !self.state.slideshow_shuffle;
                self.state.status_message = if self.state.slideshow_shuffle {
                    "Shuffle on".into()
                } else {
                    "Shuffle off".into()
                };
                self.state.status_message_time = Some(std::time::Instant::now());
                ctx.request_paint();
                true
            }

            // --- Batch cancel ---
            ClickAction::BatchCancel => {
                self.state.batch_running = false;
                self.state.batch_progress = None;
                self.state.status_message = "Batch cancelled".into();
                self.state.status_message_time = Some(std::time::Instant::now());
                ctx.request_paint();
                true
            }

            // --- Batch output folder ---
            ClickAction::BatchBrowseOutput => {
                self.open_batch_output_dialog();
                ctx.request_paint();
                true
            }

            // --- Help ---
            ClickAction::ShowHelp => {
                self.state.show_help = !self.state.show_help;
                ctx.request_paint();
                true
            }
            ClickAction::ToggleSearch => {
                self.state.show_search = !self.state.show_search;
                if !self.state.show_search {
                    self.state.search_query.clear();
                }
                ctx.request_paint();
                true
            }
            ClickAction::SearchSubmit => {
                // TODO: integrate with file system search
                self.state.status_message = format!("Searching: {}...", self.state.search_query);
                ctx.request_paint();
                true
            }
            ClickAction::ToggleBookmark => {
                if let Some(ref doc) = self.state.document {
                    let path = doc.path.clone();
                    let folder = std::path::Path::new(&path)
                        .parent()
                        .map(|p| p.to_string_lossy().to_string());
                    if let Some(folder) = folder {
                        if let Some(pos) = self
                            .state
                            .folder_bookmarks
                            .iter()
                            .position(|b| b == &folder)
                        {
                            self.state.folder_bookmarks.remove(pos);
                            self.state.status_message = "Bookmark removed".to_string();
                        } else {
                            self.state.folder_bookmarks.push(folder);
                            self.state.status_message = "Folder bookmarked".to_string();
                        }
                        ctx.request_paint();
                    }
                }
                true
            }
            ClickAction::OpenBookmark(ref idx) => {
                if let Some(path) = self.state.folder_bookmarks.get(*idx).cloned() {
                    self.open_folder(&path);
                    ctx.request_paint();
                }
                true
            }
            ClickAction::SelectAnnotationTool(ref tool) => {
                if self.state.active_annotation_tool == Some(*tool) {
                    self.state.active_annotation_tool = None;
                } else {
                    self.state.active_annotation_tool = Some(*tool);
                }
                ctx.request_paint();
                true
            }
            ClickAction::QuickEditMarkup => {
                self.state.active_annotation_tool = Some(state::AnnotationTool::Arrow);
                ctx.request_paint();
                true
            }
            ClickAction::ClearAnnotations => {
                self.state.annotations.clear();
                self.state.status_message = "Annotations cleared".to_string();
                ctx.request_paint();
                true
            }
            ClickAction::CycleCompareMode => {
                use state::CompareMode;
                self.state.compare_mode = match self.state.compare_mode {
                    CompareMode::Split => CompareMode::SideBySide,
                    CompareMode::SideBySide => CompareMode::Difference,
                    CompareMode::Difference => CompareMode::Split,
                };
                self.state.status_message = format!(
                    "Compare: {}",
                    match self.state.compare_mode {
                        CompareMode::Split => "Split",
                        CompareMode::SideBySide => "Side by Side",
                        CompareMode::Difference => "Difference",
                    }
                );
                ctx.request_paint();
                true
            }
            ClickAction::SlideshowCycleTransition => {
                self.state.slideshow_transition = self.state.slideshow_transition.cycle();
                self.state.status_message =
                    format!("Transition: {}", self.state.slideshow_transition.label());
                ctx.request_paint();
                true
            }
            ClickAction::SlideshowToggleLoop => {
                self.state.slideshow_loop = !self.state.slideshow_loop;
                self.state.status_message = if self.state.slideshow_loop {
                    "Loop: On".to_string()
                } else {
                    "Loop: Off".to_string()
                };
                ctx.request_paint();
                true
            }
            ClickAction::ToggleAnnotationColorPicker => {
                self.state.show_annotation_color_picker = !self.state.show_annotation_color_picker;
                ctx.request_paint();
                true
            }
            ClickAction::SetAnnotationColor(color) => {
                self.state.annotation_color = *color;
                self.state.status_message =
                    format!("Color: R:{} G:{} B:{}", color.r(), color.g(), color.b());
                ctx.request_paint();
                true
            }
            ClickAction::OpenRecentFromEmpty(idx) => {
                if let Some(path) = self.state.recent_files.get(*idx).cloned() {
                    self.load_image_from_path(&path);
                    ctx.request_paint();
                }
                true
            }

            // --- Annotation undo ---
            ClickAction::AnnotationUndo => {
                if let Some(snapshot) = self.state.annotation_undo_stack.pop() {
                    self.state
                        .annotation_redo_stack
                        .push(self.state.annotations.clone());
                    self.state.annotations = snapshot;
                    self.state.status_message = "Undo".to_string();
                }
                ctx.request_paint();
                true
            }

            // --- Annotation redo ---
            ClickAction::AnnotationRedo => {
                if let Some(snapshot) = self.state.annotation_redo_stack.pop() {
                    self.state
                        .annotation_undo_stack
                        .push(self.state.annotations.clone());
                    self.state.annotations = snapshot;
                    self.state.status_message = "Redo".to_string();
                }
                ctx.request_paint();
                true
            }

            // --- Annotation save (burn into image) ---
            ClickAction::AnnotationSave => {
                // TODO: composite annotations onto current_image_data pixel buffer
                // For now, mark as saved and clear undo/redo stacks
                self.state.annotation_undo_stack.clear();
                self.state.annotation_redo_stack.clear();
                self.state.status_message = "Annotations saved".to_string();
                self.state.status_message_time = Some(std::time::Instant::now());
                ctx.request_paint();
                true
            }

            // --- Annotation exit ---
            ClickAction::AnnotationExit => {
                if !self.state.annotations.is_empty() {
                    self.state.show_annotation_exit_confirm = true;
                } else {
                    self.state.active_annotation_tool = None;
                    self.state.annotation_text_input = None;
                }
                ctx.request_paint();
                true
            }

            // --- Annotation exit confirm (discard) ---
            ClickAction::AnnotationExitConfirm => {
                self.state.annotations.clear();
                self.state.annotation_undo_stack.clear();
                self.state.annotation_redo_stack.clear();
                self.state.active_annotation_tool = None;
                self.state.annotation_text_input = None;
                self.state.show_annotation_exit_confirm = false;
                self.state.status_message = "Annotations discarded".to_string();
                self.state.status_message_time = Some(std::time::Instant::now());
                ctx.request_paint();
                true
            }

            // --- Annotation exit cancel ---
            ClickAction::AnnotationExitCancel => {
                self.state.show_annotation_exit_confirm = false;
                ctx.request_paint();
                true
            }

            // --- Annotation set line width ---
            ClickAction::AnnotationSetLineWidth(w) => {
                self.state.annotation_line_width = *w;
                self.state.status_message = format!("Line width: {:.1}", w);
                self.state.status_message_time = Some(std::time::Instant::now());
                ctx.request_paint();
                true
            }

            // --- Annotation erase mode ---
            ClickAction::AnnotationEraseMode => {
                if self.state.active_annotation_tool == Some(state::AnnotationTool::Eraser) {
                    self.state.active_annotation_tool = None;
                } else {
                    self.state.active_annotation_tool = Some(state::AnnotationTool::Eraser);
                }
                ctx.request_paint();
                true
            }

            // --- Annotation text confirm ---
            ClickAction::AnnotationTextConfirm => {
                if let (Some(ref text), Some((px, py))) = (
                    &self.state.annotation_text_input,
                    self.state.annotation_drag_start,
                ) {
                    if !text.is_empty() {
                        self.state
                            .annotation_undo_stack
                            .push(self.state.annotations.clone());
                        self.state.annotation_redo_stack.clear();
                        self.state.annotations.push(state::Annotation {
                            tool: state::AnnotationTool::Text,
                            x: px,
                            y: py,
                            w: 0.0,
                            h: 0.0,
                            text: text.clone(),
                            color: self.state.annotation_color,
                            line_width: self.state.annotation_line_width,
                        });
                    }
                }
                self.state.annotation_text_input = None;
                self.state.annotation_drag_start = None;
                ctx.request_paint();
                true
            }

            _ => false,
        }
    }
}
