# Implementation Plan: Canvas Actual Size

## Feature ID
view/canvas-actual-size

## Spec Reference
`plans/spec/view/canvas-actual-size.md`

## Design Reference
`plans/design/view/canvas-actual-size.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::zoom_level`

### Change Recipe
1. 원본 크기 버튼 클릭 시 zoom_level = 1.0
2. 캔버스 즉시 100% 크기로 반영

### Find Strategies
- `grep -rn "zoom_level" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`canvas-actual-size`

### Verification
- cargo check -p view
