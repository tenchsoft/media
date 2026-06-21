# Implementation Plan: Canvas Pan

## Feature ID
view/canvas-pan

## Spec Reference
`plans/spec/view/canvas-pan.md`

## Design Reference
`plans/design/view/canvas-pan.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::pan_offset`

### Change Recipe
1. 마우스 가운데 버튼 드래그 또는 Space+드래그로 pan_offset 업데이트
2. 캔버스 즉시 팬 반영

### Find Strategies
- `grep -rn "pan_offset" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`canvas-pan`

### Verification
- cargo check -p view
