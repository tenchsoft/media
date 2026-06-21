# Implementation Plan: In Point Set

## Feature ID
composer/in-point-set

## Spec Reference
`plans/spec/composer/in-point-set.md`

## Design Reference
`plans/design/composer/in-point-set.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "in_point" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::in_point`

### Change Recipe
1. timeline_panel.rs에서 I 버튼 또는 단축키 hit-test
2. 클릭 → in_point = 현재 playhead_position
3. 타임라인에 인 포인트 마커 표시
4. 인~아웃 구간 하이라이트

### Find Strategies
- `grep -rn "in_point" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "in_point" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`in-point-set`

### Verification
- cargo check -p composer
- 설정 시 인 포인트 마커 표시
