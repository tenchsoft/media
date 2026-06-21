# Implementation Plan: Slideshow Loop

## Feature ID
view/slideshow-loop

## Spec Reference
`plans/spec/view/slideshow-loop.md`

## Design Reference
`plans/design/view/slideshow-loop.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::slideshow_loop`

### Change Recipe
1. 루프 토글 버튼 렌더링
2. 클릭 시 slideshow_loop 토글
3. 마지막 이미지 후 처음으로 돌아갈지 결정

### Find Strategies
- `grep -rn "slideshow_loop" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`slideshow-loop`

### Verification
- cargo check -p view
