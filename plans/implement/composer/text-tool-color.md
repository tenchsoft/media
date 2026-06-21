# Implementation Plan: Text Tool Color

## Feature ID
composer/text-tool-color

## Spec Reference
`plans/spec/composer/text-tool-color.md`

## Design Reference
`plans/design/composer/text-tool-color.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "text_color" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `TextOverlay::color`

### Change Recipe
1. inspector.rs에서 텍스트 색상 스와치 hit-test
2. 클릭 → 색상 피커 표시
3. 색상 선택 → TextOverlay::color 업데이트
4. 프리뷰 실시간 반영

### Find Strategies
- `grep -rn "TextOverlay" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "color" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`text-tool-color`

### Verification
- cargo check -p composer
- 색상 변경 시 텍스트 색상 변경
