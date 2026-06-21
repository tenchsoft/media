//! E2E tests for View product.
//!
//! Each test mounts ViewApp in a headless TestHarness, simulates user
//! interactions via selector-based automation, and asserts that:
//!   - The UI renders non-blank pixels.
//!   - Clicking a button changes the expected state.
//!   - The automation tree exposes the expected debug_id selectors.

use tench_ui_automation_core::{
    find_node, UiAutomationAction, UiAutomationCaptureRequest, UiAutomationSelector,
};
use tench_ui_test::{harness::HarnessConfig, snapshot::is_nonblank, TestHarness};
use tench_view_lib::ui::ViewApp;

/// Helper: read the root ViewApp state from the harness after interactions.
fn read_state(harness: &mut TestHarness) -> tench_view_lib::ui::state::ViewState {
    let pod = harness.root_mut();
    let app: &mut ViewApp = pod.widget.downcast_mut().expect("root is ViewApp");
    app.state_mut().clone()
}

fn view_harness() -> TestHarness {
    TestHarness::with_config(ViewApp::new(), HarnessConfig::with_viewport(1280.0, 720.0))
}

fn view_harness_with_image() -> TestHarness {
    let mut app = ViewApp::new();
    app.inject_test_image(200, 150);
    TestHarness::with_config(app, HarnessConfig::with_viewport(1280.0, 720.0))
}

#[path = "view_e2e/ai_panel.rs"]
mod ai_panel;
#[path = "view_e2e/annotation_actions.rs"]
mod annotation_actions;
#[path = "view_e2e/automatic.rs"]
mod automatic;
#[path = "view_e2e/batch.rs"]
mod batch;
#[path = "view_e2e/bottom_toolbar_new.rs"]
mod bottom_toolbar_new;
#[path = "view_e2e/canvas_advanced.rs"]
mod canvas_advanced;
#[path = "view_e2e/compare_panel.rs"]
mod compare_panel;
#[path = "view_e2e/context_menu.rs"]
mod context_menu;
#[path = "view_e2e/context_menu_new.rs"]
mod context_menu_new;
#[path = "view_e2e/convert_tool.rs"]
mod convert_tool;
#[path = "view_e2e/crop_tool.rs"]
mod crop_tool;
#[path = "view_e2e/delete_confirm.rs"]
mod delete_confirm;
#[path = "view_e2e/edit_banner.rs"]
mod edit_banner;
#[path = "view_e2e/file_info.rs"]
mod file_info;
#[path = "view_e2e/filter_panel.rs"]
mod filter_panel;
#[path = "view_e2e/help_overlay.rs"]
mod help_overlay;
#[path = "view_e2e/image_stage.rs"]
mod image_stage;
#[path = "view_e2e/keyboard_f2.rs"]
mod keyboard_f2;
#[path = "view_e2e/metadata_panel.rs"]
mod metadata_panel;
#[path = "view_e2e/navigation.rs"]
mod navigation;
#[path = "view_e2e/overlay_empty.rs"]
mod overlay_empty;
#[path = "view_e2e/print_backend.rs"]
mod print_backend;
#[path = "view_e2e/print_dialog.rs"]
mod print_dialog;
#[path = "view_e2e/quick_edit.rs"]
mod quick_edit;
#[path = "view_e2e/rename_dialog.rs"]
mod rename_dialog;
#[path = "view_e2e/resize_tool.rs"]
mod resize_tool;
#[path = "view_e2e/set_wallpaper.rs"]
mod set_wallpaper;
#[path = "view_e2e/settings_panel.rs"]
mod settings_panel;
#[path = "view_e2e/slideshow.rs"]
mod slideshow;
#[path = "view_e2e/slideshow_advanced.rs"]
mod slideshow_advanced;
#[path = "view_e2e/top_overlay.rs"]
mod top_overlay;
#[path = "view_e2e/url_dialog.rs"]
mod url_dialog;
#[path = "view_e2e/window_features.rs"]
mod window_features;
