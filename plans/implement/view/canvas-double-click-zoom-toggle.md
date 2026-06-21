# Implementation Plan: Canvas Double Click Zoom Toggle

## Feature ID
view/canvas-double-click-zoom-toggle

## Spec Reference
`plans/spec/view/canvas-double-click-zoom-toggle.md`

## Design Reference
`plans/design/view/canvas-double-click-zoom-toggle.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::zoom_level`

### Change Recipe
1. 캔버스 더블클릭 시 zoom_level 토글 (100% <-> fit-to-window)
2. 줌 레벨 즉시 전환

### Find Strategies
- `grep -rn "zoom_level" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`canvas-double-click-zoom-toggle`

### Verification
- cargo check -p view
