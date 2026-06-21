use super::*;

impl PlayerState {
    // ── Panels ──

    pub fn toggle_drawer(&mut self, tab: DrawerTab) {
        self.drawer = if self.drawer == Some(tab) {
            None
        } else {
            Some(tab)
        };
        self.drawer_scroll_y = 0.0;
    }

    pub fn close_all_panels(&mut self) {
        self.drawer = None;
        self.ai_panel_open = false;
        self.gif_capture_open = false;
        self.show_speed_menu = false;
        self.subtitle_style_open = false;
        self.subtitle_search_open = false;
        self.url_input_open = false;
        self.eq_open = false;
        self.help_open = false;
        self.gif_options_open = false;
        self.chapter_rename_idx = None;
    }
}
