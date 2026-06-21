// ---------------------------------------------------------------------------
// Automation debug id mapping
// ---------------------------------------------------------------------------

use crate::ui::state::ClickAction;

/// Maps a [`ClickAction`] to its automation `debug_id`.
pub(super) fn action_debug_id(action: &ClickAction) -> Option<String> {
    match action {
        // --- Top overlay ---
        ClickAction::OpenFileDialog => Some("view.top.open".into()),
        ClickAction::OpenFolderDialog => Some("view.top.folder".into()),
        ClickAction::OpenArchiveDialog => Some("view.top.archive".into()),
        ClickAction::ToggleFileInfo => Some("view.top.info".into()),
        ClickAction::ToggleQuickEdit => Some("view.top.edit".into()),
        ClickAction::SortByKey => Some("view.top.sort_key".into()),
        ClickAction::ToggleSortOrder => Some("view.top.sort_order".into()),
        ClickAction::ToggleMetadata => Some("view.top.files".into()),
        ClickAction::CopyPath => Some("view.top.copy_path".into()),
        ClickAction::CopyImage => Some("view.top.copy_img".into()),
        ClickAction::OpenFromUrl => Some("view.top.url".into()),
        ClickAction::ToggleBookmark => Some("view.top.bookmark".into()),
        ClickAction::ToggleSearch => Some("view.top.search".into()),

        // --- Bottom overlay ---
        ClickAction::ZoomFit => Some("view.bottom.fit".into()),
        ClickAction::ZoomActual => Some("view.bottom.100".into()),
        ClickAction::ZoomOut => Some("view.bottom.zoom_out".into()),
        ClickAction::ZoomIn => Some("view.bottom.zoom_in".into()),
        ClickAction::ToggleThumbnails => Some("view.bottom.filmstrip".into()),
        ClickAction::Rotate => Some("view.bottom.rotate".into()),
        ClickAction::CycleBgColor => Some("view.bottom.background".into()),
        ClickAction::ToggleFullscreen => Some("view.bottom.fullscreen".into()),

        // --- Navigation ---
        ClickAction::NavigateNext => Some("view.nav.next".into()),
        ClickAction::NavigatePrev => Some("view.nav.prev".into()),
        ClickAction::NavigateToIndex(i) => Some(format!("view.nav.index.{}", i)),

        // --- Empty state / recent files ---
        ClickAction::OpenRecentFile(i) => Some(format!("view.empty.recent.{}", i)),
        ClickAction::OpenRecentFromEmpty(i) => Some(format!("view.overlay.recent.{}", i)),

        // --- Slideshow ---
        ClickAction::ToggleSlideshow => Some("view.slideshow.toggle".into()),
        ClickAction::SlideshowCycleInterval => Some("view.slideshow.interval".into()),
        ClickAction::SlideshowToggleShuffle => Some("view.slideshow.shuffle".into()),
        ClickAction::SlideshowCycleTransition => Some("view.slideshow.transition".into()),
        ClickAction::SlideshowToggleLoop => Some("view.slideshow.loop".into()),
        ClickAction::DismissAll => Some("view.dismiss".into()),

        // --- Compare ---
        ClickAction::ToggleCompare => Some("view.compare.toggle".into()),
        ClickAction::CycleCompareMode => Some("view.compare.mode".into()),
        ClickAction::CompareDragStart => Some("view.compare.drag".into()),

        // --- Context menu ---
        ClickAction::ContextMenuAction(label) => Some(format!(
            "view.ctx.{}",
            label.to_lowercase().replace(' ', "_")
        )),

        // --- Batch panel ---
        ClickAction::ToggleBatch => Some("view.batch.close".into()),
        ClickAction::OpenBatch => Some("view.batch.trigger".into()),
        ClickAction::BatchModeResize => Some("view.batch.mode_resize".into()),
        ClickAction::BatchModeConvert => Some("view.batch.mode_convert".into()),
        ClickAction::BatchSelectFormat(fmt) => Some(format!("view.batch.format.{}", fmt)),
        ClickAction::BatchToggleSelectAll => Some("view.batch.select_all".into()),
        ClickAction::BatchToggleFile(i) => Some(format!("view.batch.file.{}", i)),
        ClickAction::BatchApply => Some("view.batch.apply".into()),
        ClickAction::BatchCancel => Some("view.batch.cancel".into()),
        ClickAction::BatchBrowseOutput => Some("view.batch.browse_output".into()),

        // --- Delete confirm ---
        ClickAction::DeleteCancel => Some("view.delete.cancel".into()),
        ClickAction::DeleteConfirm => Some("view.delete.confirm".into()),

        // --- Edit banner ---
        ClickAction::EditSave => Some("view.edit.save".into()),
        ClickAction::EditDiscard => Some("view.edit.discard".into()),

        // --- Rename dialog ---
        ClickAction::RenameConfirm => Some("view.rename.confirm".into()),
        ClickAction::RenameCancel => Some("view.rename.cancel".into()),

        // --- URL dialog ---
        ClickAction::LoadFromUrl => Some("view.url.load".into()),
        ClickAction::UrlCancel => Some("view.url.cancel".into()),

        // --- Print dialog ---
        ClickAction::PrintImage => Some("view.print.print".into()),
        ClickAction::PrintCancel => Some("view.print.cancel".into()),
        ClickAction::PrintSelectPaper(paper) => Some(format!(
            "view.print.paper.{}",
            paper.to_lowercase().replace('"', "").replace(' ', "_")
        )),
        ClickAction::PrintSelectOrientation(orientation) => Some(format!(
            "view.print.orientation.{}",
            orientation.to_lowercase()
        )),
        ClickAction::PrintSelectScaling(scaling) => Some(format!(
            "view.print.scaling.{}",
            scaling
                .to_lowercase()
                .replace(' ', "_")
                .replace('%', "_percent")
        )),

        // --- Annotation color picker ---
        ClickAction::ToggleAnnotationColorPicker => {
            Some("view.annotation.color_picker.close".into())
        }
        ClickAction::SetAnnotationColor(color) => Some(format!(
            "view.annotation.color.r{}.g{}.b{}",
            color.r(),
            color.g(),
            color.b()
        )),

        // --- Metadata panel ---
        ClickAction::SetRating(r) => Some(format!("view.metadata.rating.{}", r)),
        ClickAction::ToggleTag(tag) => Some(format!("view.metadata.tag.{}", tag.to_lowercase())),

        // --- Filter panel ---
        ClickAction::ToggleFilter => Some("view.filter.close".into()),
        ClickAction::FilterReset => Some("view.filter.reset".into()),
        ClickAction::FilterApply => Some("view.filter.apply".into()),

        // --- AI panel ---
        ClickAction::ToggleAi => Some("view.ai.close".into()),
        ClickAction::SelectAiFeature(f) => {
            Some(format!("view.ai.feature.{}", f.label().to_lowercase()))
        }
        ClickAction::RunAi => Some("view.ai.run".into()),

        // --- Quick edit overlay ---
        ClickAction::SelectAnnotationTool(tool) => {
            Some(format!("view.quick_edit.annotation.{}", tool.label()))
        }
        ClickAction::QuickEditMarkup => Some("view.quick_edit.markup".into()),
        ClickAction::ClearAnnotations => Some("view.quick_edit.clear_annotations".into()),

        // --- Help overlay ---
        ClickAction::ShowHelp => Some("view.help.close".into()),

        // --- Crop tool ---
        ClickAction::CropApply => Some("view.crop.apply".into()),
        ClickAction::CropCancel => Some("view.crop.cancel".into()),

        // --- Resize tool ---
        ClickAction::ResizeApply => Some("view.resize.apply".into()),
        ClickAction::ResizeCancel => Some("view.resize.cancel".into()),
        ClickAction::ResizeWidthMinus => Some("view.resize.width_minus".into()),
        ClickAction::ResizeWidthPlus => Some("view.resize.width_plus".into()),
        ClickAction::ResizeHeightMinus => Some("view.resize.height_minus".into()),
        ClickAction::ResizeHeightPlus => Some("view.resize.height_plus".into()),
        ClickAction::ResizeToggleAspect => Some("view.resize.aspect".into()),

        // --- Convert tool ---
        ClickAction::ConvertSelectFormat(fmt) => Some(format!("view.convert.format.{}", fmt)),
        ClickAction::ConvertApply => Some("view.convert.apply".into()),
        ClickAction::ConvertCancel => Some("view.convert.cancel".into()),
        ClickAction::ConvertBrowseOutput => Some("view.convert.browse_output".into()),

        // --- Search ---
        ClickAction::SearchSubmit => Some("view.search.submit".into()),

        // --- Bookmarks ---
        ClickAction::OpenBookmark(i) => Some(format!("view.bookmark.{}", i)),

        // --- Platform actions ---
        ClickAction::ShareImage => Some("view.bottom.share".into()),
        ClickAction::SetWallpaperAction => Some("view.bottom.wallpaper".into()),
        ClickAction::DeleteFromToolbar => Some("view.bottom.delete".into()),

        // --- Annotation undo/redo/save/exit ---
        ClickAction::AnnotationUndo => Some("view.annotation.undo".into()),
        ClickAction::AnnotationRedo => Some("view.annotation.redo".into()),
        ClickAction::AnnotationSave => Some("view.annotation.save".into()),
        ClickAction::AnnotationExit => Some("view.annotation.exit".into()),
        ClickAction::AnnotationExitConfirm => Some("view.annotation.exit_confirm".into()),
        ClickAction::AnnotationExitCancel => Some("view.annotation.exit_cancel".into()),
        ClickAction::AnnotationSetLineWidth(w) => {
            Some(format!("view.annotation.line_width.{}", (w * 10.0) as u32))
        }
        ClickAction::AnnotationEraseMode => Some("view.annotation.erase_mode".into()),
        ClickAction::AnnotationTextConfirm => Some("view.annotation.text_confirm".into()),

        // --- Settings panel ---
        ClickAction::ToggleSettings => Some("view.settings.toggle".into()),
        ClickAction::SettingsTab(tab) => Some(format!(
            "view.settings.tab.{}",
            match tab {
                super::super::state::SettingsTab::General => "general",
                super::super::state::SettingsTab::Image => "image",
                super::super::state::SettingsTab::Slideshow => "slideshow",
                super::super::state::SettingsTab::FileAssociation => "files",
            }
        )),
        ClickAction::SettingsClose => Some("view.settings.close".into()),

        // --- Canvas advanced ---
        ClickAction::ToggleCheckerboard => Some("view.canvas.checkerboard".into()),
        ClickAction::CropAspectRatio(w, h) => Some(format!("view.crop.aspect_ratio.{}_{}", w, h)),
        ClickAction::CropAspectRatioFree => Some("view.crop.aspect_ratio.free".into()),

        // --- Window ---
        ClickAction::ToggleMenu => Some("view.top.menu".into()),

        _ => None,
    }
}
