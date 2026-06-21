# Implementation Plan: Skip Backward Button

## Feature ID
composer/skip-backward-button

## Spec Reference
`plans/spec/composer/skip-backward-button.md`

## Design Reference
`plans/design/composer/skip-backward-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "skip_backward" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::playhead_position`

### Change Recipe
1. timeline_panel.rs에서 skip-backward 버튼 hit-test 영역 확인
2. 클릭 이벤트 → playhead_position을 이전 클립 경계로 이동
3. 처음이면 0으로
4. 프리뷰 업데이트

### Find Strategies
- `grep -rn "skip_backward" apps/composer/src-tauri/src/ui/`
- `grep -rn "playhead_position" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`skip-backward-button`

### Verification
- cargo check -p composer
- 클릭 시 이전 경계로 점프
