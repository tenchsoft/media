# Automatic Zoom Percent Display Update

## Source Plan
- `plans/pixel-design/automatic-zoom-percent-display-update-work-plan.md`

## Gap Analysis
The zoom label and `pd.auto.zoom_percent` render `state.zoom`, and all zoom input paths mutate that state. However, canvas layout applies an additional fit factor after multiplying by `state.zoom`, so the displayed zoom percent can disagree with the actual rendered document scale. See `apps/pixel-design/src-tauri/src/ui/canvas.rs:7` and `apps/pixel-design/src-tauri/src/ui/canvas.rs:13`.

`zoom_fit` and `zoom_actual` both set `zoom = 100`, while Fit to screen does not compute a distinct zoom percentage from viewport and document dimensions. This makes the fit shortcut label ambiguous and can make canvas scale and displayed percent diverge. See `apps/pixel-design/src-tauri/src/ui/state.rs:1228`.

The current E2E coverage clicks zoom in/out and the slider, then only asserts the stored zoom value changed. It does not verify `pd.auto.zoom_percent`, scroll wheel zoom, fit/actual shortcuts, label/canvas-scale agreement, alternate paths, persona switches, or viewport resize. See `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:517`.

## Plan Requirements Not Met
- Displayed zoom percent must agree with the actual rendered canvas scale.
- Fit to screen must define and expose the zoom percentage it represents.
- Tests must verify zoom percent label after buttons, slider, scroll wheel, keyboard zoom, fit, and actual shortcuts.
- Tests must verify equivalent zoom states from alternate paths produce identical labels and canvas layout.
- Tests must verify zoom percent remains correct after persona switches and viewport resize.
- Automation must expose enough canvas-scale data to compare label and rendered scale.

## Required Test Shape
- Add a Pixel Design UI automation test that uses zoom buttons, slider, scroll wheel, `+`, `-`, Ctrl+0, and Ctrl+1, then asserts `pd.auto.zoom_percent` after each path.
- Compare `pd.auto.zoom_percent` with document rect scale from canvas layout metadata.
- Set the same final zoom through two paths and assert identical zoom label and canvas layout.
- Resize the viewport and switch personas after zooming, then assert zoom label and rendered scale stay consistent.
- Use capture assertions to verify valid, nonblank output and expected visual scale changes.

## Required Changes
- Define whether `state.zoom` is absolute document scale or fit-adjusted display scale.
- Update canvas layout and fit/actual shortcuts so zoom label and rendered scale agree.
- Expose rendered canvas scale through automation metadata.
- Add zoom percent display E2E and state tests.

## Verification
- `cargo test -p tench-pixel-design --test pixel_design_e2e automatic_zoom_percent_display_update`
- `cargo test -p tench-pixel-design`
