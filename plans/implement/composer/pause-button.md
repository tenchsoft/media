# Implementation Plan: Pause Button

## Feature ID
composer/pause-button

## Spec Reference
`plans/spec/composer/pause-button.md`

## Design Reference
`plans/design/composer/pause-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "pause" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::is_playing`

### Change Recipe
1. timeline_panel.rs에서 pause 버튼 hit-test 영역 확인
2. 클릭 이벤트 → is_playing = false
3. playhead_position 현재 위치 유지
4. 프리뷰 정지

### Find Strategies
- `grep -rn "is_playing" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "pause" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`pause-button`

### Verification
- cargo check -p composer
- 재생 중 클릭 시 일시정지
