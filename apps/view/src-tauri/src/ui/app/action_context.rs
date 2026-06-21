// ---------------------------------------------------------------------------
// Click action dispatch: context
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::{self, ClickAction};

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_context_action(
        &mut self,
        action: &ClickAction,
        ctx: &mut EventCtx,
    ) -> bool {
        match action {
            ClickAction::ContextMenuAction(label) => {
                self.state.show_context_menu = false;
                match label.as_str() {
                    "Open Image" => self.open_file_dialog(),
                    "Open Folder" => self.open_folder_dialog(),
                    "Filters" => {
                        self.state.show_filter = true;
                        ctx.request_paint();
                    }
                    "Metadata" => {
                        self.state.show_metadata = true;
                        ctx.request_paint();
                    }
                    "Show in Files" => {
                        if let Some(ref doc) = self.state.document.clone() {
                            let path = doc.path.clone();
                            std::thread::spawn(move || {
                                let _ = crate::platform_util::show_in_file_manager(&path);
                            });
                        }
                    }
                    "Copy Path" => {
                        if let Some(ref doc) = self.state.document {
                            let path = doc.path.clone();
                            std::thread::spawn(move || {
                                let _ = crate::platform_util::copy_to_clipboard_text(&path);
                            });
                            self.state.status_message = format!("Copied: {}", doc.path);
                        }
                        ctx.request_paint();
                    }
                    "Copy Image" => {
                        if let Some(ref image_data) = self.state.current_image_data {
                            let w = image_data.width as usize;
                            let h = image_data.height as usize;
                            let pixels = image_data.data.data().to_vec();
                            std::thread::spawn(move || {
                                let _ = crate::platform_util::copy_to_clipboard_image(w, h, pixels);
                            });
                            self.state.status_message = "Copied image to clipboard".into();
                        }
                        ctx.request_paint();
                    }
                    "Rename" => {
                        if let Some(ref doc) = self.state.document {
                            self.state.rename_original_name = doc.file_name.clone();
                            self.state.rename_input_text = doc.file_name.clone();
                            self.state.show_rename = true;
                        }
                        ctx.request_paint();
                    }
                    "Delete" => {
                        self.state.show_delete_confirm = true;
                        ctx.request_paint();
                    }
                    "Crop" => {
                        self.state.show_quick_edit = false;
                        self.state.active_edit_tool = Some(state::EditTool::Crop);
                        ctx.request_paint();
                    }
                    "Resize" => {
                        self.state.show_quick_edit = false;
                        self.state.active_edit_tool = Some(state::EditTool::Resize);
                        if let Some(ref img) = self.state.current_image_data {
                            self.state.resize_width = img.width;
                            self.state.resize_height = img.height;
                            self.state.resize_orig_width = img.width;
                            self.state.resize_orig_height = img.height;
                        }
                        ctx.request_paint();
                    }
                    "Convert" => {
                        self.state.show_quick_edit = false;
                        self.state.active_edit_tool = Some(state::EditTool::Convert);
                        ctx.request_paint();
                    }
                    "Print" => {
                        self.state.show_print_dialog = true;
                        ctx.request_paint();
                    }
                    "Rotate Left" => {
                        if let Some(ref image_data) = self.state.current_image_data.clone() {
                            if let Some(rotated) =
                                tench_image_runtime::view::util::rotate_image_data(image_data, 3)
                            {
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
                                self.state.status_message = "Rotated left".into();
                            }
                        }
                        ctx.request_paint();
                    }
                    "Rotate Right" => {
                        if let Some(ref image_data) = self.state.current_image_data.clone() {
                            if let Some(rotated) =
                                tench_image_runtime::view::util::rotate_image_data(image_data, 1)
                            {
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
                                self.state.status_message = "Rotated right".into();
                            }
                        }
                        ctx.request_paint();
                    }
                    "Set as Wallpaper" => {
                        if let Some(ref doc) = self.state.document.clone() {
                            let path = doc.path.clone();
                            std::thread::spawn(move || {
                                let _ = crate::platform_util::set_wallpaper(&path);
                            });
                        }
                        self.state.status_message = "Set as wallpaper".into();
                        ctx.request_paint();
                    }
                    "Open With..." => {
                        if let Some(ref doc) = self.state.document.clone() {
                            let path = doc.path.clone();
                            std::thread::spawn(move || {
                                let _ = crate::platform_util::open_with(&path);
                            });
                        }
                        ctx.request_paint();
                    }
                    "Properties" => {
                        self.state.show_file_info = true;
                        ctx.request_paint();
                    }
                    _ => {
                        ctx.request_paint();
                    }
                }
                true
            }

            _ => false,
        }
    }
}
