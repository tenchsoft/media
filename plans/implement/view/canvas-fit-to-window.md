# Implementation Plan: Canvas Fit to Window

## Feature ID
view/canvas-fit-to-window

## Spec Reference
`plans/spec/view/canvas-fit-to-window.md`

## Design Reference
`plans/design/view/canvas-fit-to-window.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::zoom_level`

### Change Recipe
1. 창에 맞춤 버튼 클릭 시 이미지가 창 크기에 맞게 축소/확대
2. zoom_level = min(window_w / img_w, window_h / img_h)
3. 캔버스 즉시 반영

### Find Strategies
- `grep -rn "zoom_level" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`canvas-fit-to-window`

### Verification
- cargo check -p view
