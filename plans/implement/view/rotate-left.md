# Implementation Plan: Rotate Left

## Feature ID
view/rotate-left

## Spec Reference
`plans/spec/view/rotate-left.md`

## Design Reference
`plans/design/view/rotate-left.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::rotation`

### Change Recipe
1. 왼쪽 회전 시 rotation -= 90도
2. 캔버스 렌더링 시 rotation 적용

### Find Strategies
- `grep -rn "rotation" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`rotate-left`

### Verification
- cargo check -p view
