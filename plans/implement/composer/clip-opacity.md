# Implementation Plan: Clip Opacity

## Feature ID
composer/clip-opacity

## Spec Reference
`plans/spec/composer/clip-opacity.md`

## Design Reference
`plans/design/composer/clip-opacity.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "opacity" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::opacity`

### Change Recipe
1. inspector.rs에서 opacity 슬라이더 hit-test
2. 드래그 → Clip::opacity 값 변경 (0~100%)
3. 프리뷰 실시간 반영
4. 리렌더

### Find Strategies
- `grep -rn "opacity" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "opacity" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`clip-opacity`

### Verification
- cargo check -p composer
- 슬라이더 조정 시 클립 투명도 변경
