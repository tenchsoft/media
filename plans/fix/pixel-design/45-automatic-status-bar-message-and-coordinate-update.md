# Automatic Status Bar Message And Coordinate Update

## Source Plan
- `plans/pixel-design/automatic-status-bar-message-and-coordinate-update-work-plan.md`

## Gap Analysis
Status bar painting reads `status_msg`, document size, and `mouse_pos`, and canvas pointer down/move updates `mouse_pos`. However, the automation node for `pd.auto.status_bar` only exposes status text and coordinates; it omits the document size that is painted in the status bar. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:252`, `apps/pixel-design/src-tauri/src/ui/state.rs:799`, and `apps/pixel-design/src-tauri/src/ui/mod.rs:1341`.

Some command paths still mutate visible state without setting a user-facing status message, so the status bar cannot always reflect the latest action. Examples include adjust slider filter application and AI panel tool row selection. See `apps/pixel-design/src-tauri/src/ui/state.rs:1405` and `apps/pixel-design/src-tauri/src/ui/mod.rs:503`.

The current E2E coverage asserts `state.status_msg` for several tool actions and only asserts that the status bar node exists. It does not verify status bar automation value, pointer-coordinate updates on hover/move, document size updates after crop/load, save/export status, alternate paths, persona switches, or resize. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:175` and `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:389`.

## Plan Requirements Not Met
- `pd.auto.status_bar` must expose document size in addition to message and pointer coordinates.
- All user-visible command paths must set a status message or define why no message is expected.
- Tests must verify pointer document coordinates update on pointer movement over the canvas.
- Tests must verify status bar document size updates after crop and file load.
- Tests must verify load/save/export status messages through the status bar UI, not only state fields.
- Tests must verify equivalent state changes from alternate paths produce identical status bar output.
- Tests must verify status bar output remains correct after persona switches and viewport resize.

## Required Test Shape
- Add a Pixel Design UI automation test that moves the pointer over known canvas fractions and asserts `pd.auto.status_bar` coordinate values.
- Crop or load a fixture image and assert the status bar document size value changes.
- Run tool actions, save, export, and failed load/save paths, then assert the status bar automation value contains the expected message.
- Trigger equivalent actions through UI and keyboard paths and assert identical status bar values.
- Switch personas and resize, then assert status bar message, document size, and coordinates remain correct.

## Required Changes
- Extend `pd.auto.status_bar` value or child nodes to include document size as a separate verifiable field.
- Set status messages for adjust slider changes and AI panel tool row selections, or explicitly define those actions as status-neutral.
- Add status bar E2E tests covering message, document size, and coordinate updates.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_status_bar_message_and_coordinate_update`
- `cargo test -p tench-pixel-design`
