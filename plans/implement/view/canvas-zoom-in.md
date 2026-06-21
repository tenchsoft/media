# Implementation Plan: Canvas Zoom In

## Feature ID
view/canvas-zoom-in

## Spec Reference
`plans/spec/view/canvas-zoom-in.md`

## Design Reference
`plans/design/view/canvas-zoom-in.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::zoom_level`

### Change Recipe
1. 확대 버튼 또는 Ctrl+= 클릭 시 zoom_level *= 1.25
2. 최대 줌 레벨 제한 (예: 32x)
3. 캔버스 즉시 확대 반영

### Find Strategies
- `grep -rn "zoom_level" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`canvas-zoom-in`

### Verification
- cargo check -p view
