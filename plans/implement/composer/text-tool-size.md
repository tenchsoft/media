# Implementation Plan: Text Tool Size

## Feature ID
composer/text-tool-size

## Spec Reference
`plans/spec/composer/text-tool-size.md`

## Design Reference
`plans/design/composer/text-tool-size.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "font_size" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `TextOverlay::font_size`

### Change Recipe
1. inspector.rs에서 폰트 크기 슬라이더/입력 hit-test
2. 값 변경 → TextOverlay::font_size 업데이트
3. 프리뷰 실시간 반영

### Find Strategies
- `grep -rn "font_size" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "font_size" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`text-tool-size`

### Verification
- cargo check -p composer
- 크기 변경 시 텍스트 크기 변경
