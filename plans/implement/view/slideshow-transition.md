# Implementation Plan: Slideshow Transition

## Feature ID
view/slideshow-transition

## Spec Reference
`plans/spec/view/slideshow-transition.md`

## Design Reference
`plans/design/view/slideshow-transition.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::slideshow_transition`

### Change Recipe
1. 전환 효과 선택 드롭다운 렌더링 (없음, 페이드, 슬라이드 등)
2. 선택 시 slideshow_transition 업데이트
3. 슬라이드쇼 전환 시 선택된 효과 적용

### Find Strategies
- `grep -rn "slideshow_transition" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`slideshow-transition`

### Verification
- cargo check -p view
