# Implementation Plan: Play Button

## Feature ID
composer/play-button

## Spec Reference
`plans/spec/composer/play-button.md`

## Design Reference
`plans/design/composer/play-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "play" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::is_playing`

### Change Recipe
1. timeline_panel.rs에서 play 버튼 hit-test 영역 확인
2. 클릭 이벤트 → is_playing = true
3. playhead_position 프레임 단위 전진
4. playhead가 마지막 클립 끝에 도달하면 정지

### Find Strategies
- `grep -rn "is_playing" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "play" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`play-button`

### Verification
- cargo check -p composer
- 클릭 시 재생 시작
