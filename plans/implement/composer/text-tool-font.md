# Implementation Plan: Text Tool Font

## Feature ID
composer/text-tool-font

## Spec Reference
`plans/spec/composer/text-tool-font.md`

## Design Reference
`plans/design/composer/text-tool-font.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "font" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `TextOverlay::font_family`

### Change Recipe
1. inspector.rs에서 폰트 드롭다운 hit-test
2. 선택 → TextOverlay::font_family 업데이트
3. 시스템 폰트 목록 표시
4. 프리뷰 실시간 반영

### Find Strategies
- `grep -rn "font_family" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "font" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`text-tool-font`

### Verification
- cargo check -p composer
- 폰트 변경 시 텍스트 외관 변경
