# Implementation Plan: Slideshow Speed

## Feature ID
view/slideshow-speed

## Spec Reference
`plans/spec/view/slideshow-speed.md`

## Design Reference
`plans/design/view/slideshow-speed.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::slideshow_speed`

### Change Recipe
1. 슬라이드쇼 속도 슬라이더 렌더링
2. 드래그 시 slideshow_speed 업데이트 (초 단위)
3. 활성 슬라이드쇼에 즉시 반영

### Find Strategies
- `grep -rn "slideshow_speed" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`slideshow-speed`

### Verification
- cargo check -p view
