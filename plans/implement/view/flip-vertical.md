# Implementation Plan: Flip Vertical

## Feature ID
view/flip-vertical

## Spec Reference
`plans/spec/view/flip-vertical.md`

## Design Reference
`plans/design/view/flip-vertical.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::flip_v`

### Change Recipe
1. 상하 반전 시 flip_v 토글
2. 캔버스 렌더링 시 flip_v 적용

### Find Strategies
- `grep -rn "flip_v" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`flip-vertical`

### Verification
- cargo check -p view
