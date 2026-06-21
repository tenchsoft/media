# History Step Row Control State

## Source Plan
- `plans/pixel-design/history-step-row-control-work-plan.md`

## Gap Analysis
History rows are rendered with active styling, but automation step nodes do not expose active/selected row state. Tests cannot verify that clicking a row moves the highlight to that row through the UI tree. See `apps/pixel-design/src-tauri/src/ui/layers.rs:253` and `apps/pixel-design/src-tauri/src/ui/mod.rs:1155`.

Clicking a history step updates `history_index`, clones the snapshot into the document, and refreshes flattened output, but it does not set a status message. Without active-row automation metadata, there is no stable acknowledgement exposed for the row-click action. See `apps/pixel-design/src-tauri/src/ui/mod.rs:441`.

Only the first eight history rows are rendered and exposed to automation, while the history model keeps up to 50 snapshots. Snapshots beyond the first eight cannot be clicked through the current dynamic row control. See `apps/pixel-design/src-tauri/src/ui/mod.rs:442` and `apps/pixel-design/src-tauri/src/ui/state.rs:1271`.

The current E2E coverage clicks `pd.history.step.0` and only asserts `history_index == 0`. It does not verify document pixels, layer metadata, active row highlight, canvas refresh, thumbnail refresh, clicking the current row as a no-op, or forward-history truncation after a new edit. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:347`.

## Plan Requirements Not Met
- History step automation must expose active/selected row state.
- Clicking a history row must expose a deterministic acknowledgement through status or active-row metadata.
- All reachable history snapshots must have a row interaction model or scrolling/paging behavior.
- Tests must verify clicking an older row jumps directly to that snapshot's document state.
- Tests must verify clicking the current row creates no extra history entry.
- Tests must verify a new edit after jumping backward truncates forward history.
- Tests must verify canvas and thumbnails refresh after row jumps.

## Required Test Shape
- Create multiple distinct edits, click an older history row, and assert history index, active row metadata, status or acknowledgement, document pixels, layer metadata, flattened capture, and thumbnails.
- Click the current active row and assert history length, history index, document, dirty state, and visible capture remain unchanged.
- Jump backward, make a new edit, and assert forward history is truncated according to the history model.
- Build more than eight history entries and assert the product-defined scrolling, paging, or truncation behavior for rows beyond the first visible set.

## Required Changes
- Expose active state on history step automation nodes.
- Add status or equivalent acknowledgement when a history step row is selected.
- Define and implement row access for history entries beyond the first eight.
- Add History Step Row E2E tests for snapshot jumps, current-row no-op behavior, forward truncation, canvas refresh, thumbnail refresh, and long-history access.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e history_step_row`
- `cargo test -p tench-pixel-design`
