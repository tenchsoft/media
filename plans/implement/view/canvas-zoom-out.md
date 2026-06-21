# Implementation Plan: Canvas Zoom Out

## Feature ID
view/canvas-zoom-out

## Spec Reference
`plans/spec/view/canvas-zoom-out.md`

## Design Reference
`plans/design/view/canvas-zoom-out.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::zoom_level`

### Change Recipe
1. 축소 버튼 또는 Ctrl+- 클릭 시 zoom_level /= 1.25
2. 최소 줌 레벨 제한 (예: 0.1x)
3. 캔버스 즉시 축소 반영

### Find Strategies
- `grep -rn "zoom_level" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`canvas-zoom-out`

### Verification
- cargo check -p view
