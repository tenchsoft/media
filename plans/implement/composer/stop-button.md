# Implementation Plan: Stop Button

## Feature ID
composer/stop-button

## Spec Reference
`plans/spec/composer/stop-button.md`

## Design Reference
`plans/design/composer/stop-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "stop" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::is_playing`, `playhead_position`

### Change Recipe
1. timeline_panel.rs에서 stop 버튼 hit-test 영역 확인
2. 클릭 이벤트 → is_playing = false, playhead_position = 0
3. 프리뷰 첫 프레임 표시
4. 리렌더

### Find Strategies
- `grep -rn "stop" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- `grep -rn "playhead_position" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`stop-button`

### Verification
- cargo check -p composer
- 클릭 시 재생 정지 및 playhead 처음으로
