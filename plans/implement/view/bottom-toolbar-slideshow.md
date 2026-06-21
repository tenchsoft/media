# Implementation Plan: Bottom Toolbar Slideshow

## Feature ID
view/bottom-toolbar-slideshow

## Spec Reference
`plans/spec/view/bottom-toolbar-slideshow.md`

## Design Reference
`plans/design/view/bottom-toolbar-slideshow.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::slideshow_active`

### Change Recipe
1. 하단 도구 모음에서 슬라이드쇼 버튼 렌더링
2. 클릭 시 slideshow_active 토글
3. 슬라이드쇼 시작/정지

### Find Strategies
- `grep -rn "slideshow_active" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`bottom-toolbar-slideshow`

### Verification
- cargo check -p view
