# Implementation Plan: Slideshow Start

## Feature ID
view/slideshow-start

## Spec Reference
`plans/spec/view/slideshow-start.md`

## Design Reference
`plans/design/view/slideshow-start.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::slideshow_active`

### Change Recipe
1. 슬라이드쇼 시작 시 slideshow_active = true
2. 설정된 간격으로 자동 다음 이미지 전환
3. 마지막 이미지에서 루프 옵션 적용

### Find Strategies
- `grep -rn "slideshow" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`slideshow-start`

### Verification
- cargo check -p view
