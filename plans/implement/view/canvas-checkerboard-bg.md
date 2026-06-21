# Implementation Plan: Canvas Checkerboard BG

## Feature ID
view/canvas-checkerboard-bg

## Spec Reference
`plans/spec/view/canvas-checkerboard-bg.md`

## Design Reference
`plans/design/view/canvas-checkerboard-bg.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::checkerboard_bg`

### Change Recipe
1. 체크무늬 배경 토글 버튼 렌더링
2. 클릭 시 checkerboard_bg 토글
3. 투명 PNG 등에서 체크무늬 배경 표시/숨김

### Find Strategies
- `grep -rn "checkerboard" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`canvas-checkerboard-bg`

### Verification
- cargo check -p view
