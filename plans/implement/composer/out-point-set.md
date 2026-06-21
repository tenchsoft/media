# Implementation Plan: Out Point Set

## Feature ID
composer/out-point-set

## Spec Reference
`plans/spec/composer/out-point-set.md`

## Design Reference
`plans/design/composer/out-point-set.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "out_point" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::out_point`

### Change Recipe
1. timeline_panel.rs에서 O 버튼 또는 단축키 hit-test
2. 클릭 → out_point = 현재 playhead_position
3. 타임라인에 아웃 포인트 마커 표시
4. 인~아웃 구간 하이라이트

### Find Strategies
- `grep -rn "out_point" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "out_point" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`out-point-set`

### Verification
- cargo check -p composer
- 설정 시 아웃 포인트 마커 표시
