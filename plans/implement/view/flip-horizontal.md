# Implementation Plan: Flip Horizontal

## Feature ID
view/flip-horizontal

## Spec Reference
`plans/spec/view/flip-horizontal.md`

## Design Reference
`plans/design/view/flip-horizontal.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::flip_h`

### Change Recipe
1. 좌우 반전 시 flip_h 토글
2. 캔버스 렌더링 시 flip_h 적용

### Find Strategies
- `grep -rn "flip_h" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`flip-horizontal`

### Verification
- cargo check -p view
