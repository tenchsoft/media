# Automatic Gapless Next Track State

## Source Plan
- `plans/player/automatic-gapless-next-track-work-plan.md`

## Gap Analysis
The `MediaEvent::AboutToFinish` path calls `backend.set_next_uri(uri)` for the next playlist entry, but there is no targeted test that injects the event and observes the backend call. The current E2E only asserts that the generic `player.automatic.gapless_next` node exists and exercises playlist row clicks. See `apps/player/src-tauri/src/ui/app.rs:486`, `apps/player/src-tauri/tests/plan_ui_e2e.rs:128`, and `apps/player/src-tauri/tests/plan_ui_e2e.rs:246`.

The automatic gapless status node is always emitted with no value for prepared next URI, current playlist index, or whether a next item was available. Automation cannot assert that the automatic behavior is derived from the canonical playlist state. See `apps/player/src-tauri/src/ui/app.rs:2282`.

There is no coverage for layout changes after the gapless state is prepared. The playlist drawer highlights the current item, but tests do not verify that opening a drawer or resizing after `AboutToFinish` keeps the UI consistent. See `apps/player/src-tauri/src/ui/paint_panels.rs:239`.

## Plan Requirements Not Met
- Tests must verify `MediaEvent::AboutToFinish` calls `backend.set_next_uri` with the next playlist URI.
- Tests must verify no stale or incorrect URI is prepared when there is no next item.
- Tests must verify the prepared-next state is consistent after opening side panels or resizing.
- Automation must expose enough gapless state to assert the prepared next URI or next index.

## Required Test Shape
- Configure a playlist with `current_playlist_index = Some(0)`, inject `MediaEvent::AboutToFinish`, and assert the fake backend received `set_next_uri` for index 1.
- Repeat at the final playlist item and assert no `set_next_uri` call occurs.
- Open the playlist drawer or resize after preparing the next URI and assert current/next playlist state remains consistent and no extra backend call occurs.

## Required Changes
- Add a fake backend or backend-call spy for gapless tests.
- Expose prepared next URI, next index, or active state in `player.automatic.gapless_next`.
- Extend `plan_ui_e2e` or targeted UI tests for AboutToFinish, no-next-item, and layout-change behavior.

## Verification
- `cargo test -p tench-player --test plan_ui_e2e automatic_gapless_next`
- `cargo test -p tench-player`
