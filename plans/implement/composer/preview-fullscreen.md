# Implementation Plan: Preview Fullscreen

## Feature ID
composer/preview-fullscreen

## Spec Reference
`plans/spec/composer/preview-fullscreen.md`

## Design Reference
`plans/design/composer/preview-fullscreen.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/preview_panel.rs`
- Search: `grep -n "fullscreen" apps/composer/src-tauri/src/ui/preview_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::preview_fullscreen`

### Change Recipe
1. preview_panel.rs에서 전체화면 버튼 hit-test
2. 클릭 → preview_fullscreen 토글
3. 전체화면 시 프리뷰가 전체 창 차지
4. Esc 키로 전체화면 해제

### Find Strategies
- `grep -rn "preview_fullscreen" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "fullscreen" apps/composer/src-tauri/src/ui/preview_panel.rs`

### Debug ID
`preview-fullscreen`

### Verification
- cargo check -p composer
- 전체화면 전환/해제 동작
