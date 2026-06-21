#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tench_ui::run_native_with_config(
        tench_ui::NativeConfig {
            title: "Tench Composer".into(),
            width: 1440.0,
            height: 900.0,
            resizable: true,
        },
        |backend| backend.set_root(tench_composer_lib::ui::ComposerApp::new()),
    );
}
