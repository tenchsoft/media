fn main() {
    tench_ui::run_native_with_config(
        tench_ui::NativeConfig {
            title: "Tench View".into(),
            width: 1280.0,
            height: 820.0,
            resizable: true,
        },
        |backend| backend.set_root(tench_view_lib::ui::ViewApp::new()),
    );
}
