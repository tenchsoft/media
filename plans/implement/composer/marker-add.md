# Implementation Plan: Marker Add

## Feature ID
composer/marker-add

## Spec Reference
`plans/spec/composer/marker-add.md`

## Design Reference
`plans/design/composer/marker-add.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "marker" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::markers`

### Change Recipe
1. 타임라인에서 마커 추가 액션 (M 키 또는 버튼)
2. 현재 playhead 위치에 새 Marker 생성
3. markers 벡터에 추가
4. 타임라인 상단에 마커 아이콘 표시

### Find Strategies
- `grep -rn "markers" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "marker" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`marker-add`

### Verification
- cargo check -p composer
- 마커 추가 시 타임라인에 표시
