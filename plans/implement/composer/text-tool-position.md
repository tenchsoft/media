# Implementation Plan: Text Tool Position

## Feature ID
composer/text-tool-position

## Spec Reference
`plans/spec/composer/text-tool-position.md`

## Design Reference
`plans/design/composer/text-tool-position.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/preview.rs`
- Search: `grep -n "text_position\|text_drag" apps/composer/src-tauri/src/ui/preview.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `TextOverlay::x`, `TextOverlay::y`

### Change Recipe
1. preview.rs에서 텍스트 오버레이 hit-test
2. 드래그 → TextOverlay::x, TextOverlay::y 업데이트
3. 실시간 위치 이동
4. 드롭 → 최종 위치 확정

### Find Strategies
- `grep -rn "TextOverlay" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "text" apps/composer/src-tauri/src/ui/preview.rs`

### Debug ID
`text-tool-position`

### Verification
- cargo check -p composer
- 드래그 시 텍스트 위치 이동
