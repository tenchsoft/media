# Implementation Plan: Marker Delete

## Feature ID
composer/marker-delete

## Spec Reference
`plans/spec/composer/marker-delete.md`

## Design Reference
`plans/design/composer/marker-delete.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "marker_delete" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::markers`

### Change Recipe
1. 마커 선택 후 삭제 액션
2. markers 벡터에서 해당 마커 제거
3. 타임라인 리렌더

### Find Strategies
- `grep -rn "markers" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "marker" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`marker-delete`

### Verification
- cargo check -p composer
- 삭제 시 마커 제거
