// ---------------------------------------------------------------------------
// Click action dispatch entry point
// ---------------------------------------------------------------------------

use tench_ui::prelude::*;

use crate::ui::state::ClickAction;

use super::ViewApp;

impl ViewApp {
    pub(super) fn dispatch_click_action(&mut self, action: &ClickAction, ctx: &mut EventCtx) {
        let _ = self.dispatch_navigation_action(action, ctx)
            || self.dispatch_panel_action(action, ctx)
            || self.dispatch_context_action(action, ctx)
            || self.dispatch_edit_file_action(action, ctx)
            || self.dispatch_tool_action(action, ctx)
            || self.dispatch_batch_dialog_action(action, ctx)
            || self.dispatch_metadata_action(action, ctx);
    }
}
