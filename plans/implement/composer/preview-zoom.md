# Implementation Plan: Preview Zoom

## Feature ID
composer/preview-zoom

## Spec Reference
`plans/spec/composer/preview-zoom.md`

## Design Reference
`plans/design/composer/preview-zoom.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/preview_panel.rs`
- Search: `grep -n "zoom" apps/composer/src-tauri/src/ui/preview_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::preview_zoom`

### Change Recipe
1. preview_panel.rs에서 줌 컨트롤 hit-test
2. 스크롤/버튼 → preview_zoom 값 변경
3. 프리뷰 확대/축소
4. Fit, 50%, 100%, 200% 프리셋

### Find Strategies
- `grep -rn "preview_zoom" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "zoom" apps/composer/src-tauri/src/ui/preview_panel.rs`

### Debug ID
`preview-zoom`

### Verification
- cargo check -p composer
- 줌 변경 시 프리뷰 확대/축소
