# Implementation Plan: Rotate Right

## Feature ID
view/rotate-right

## Spec Reference
`plans/spec/view/rotate-right.md`

## Design Reference
`plans/design/view/rotate-right.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::rotation`

### Change Recipe
1. 오른쪽 회전 시 rotation += 90도
2. 캔버스 렌더링 시 rotation 적용

### Find Strategies
- `grep -rn "rotation" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`rotate-right`

### Verification
- cargo check -p view
