# Implementation Plan: Skip Forward Button

## Feature ID
composer/skip-forward-button

## Spec Reference
`plans/spec/composer/skip-forward-button.md`

## Design Reference
`plans/design/composer/skip-forward-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "skip_forward" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::playhead_position`

### Change Recipe
1. timeline_panel.rs에서 skip-forward 버튼 hit-test 영역 확인
2. 클릭 이벤트 → playhead_position을 다음 클립 경계로 이동
3. 마지막이면 타임라인 끝으로
4. 프리뷰 업데이트

### Find Strategies
- `grep -rn "skip_forward" apps/composer/src-tauri/src/ui/`
- `grep -rn "playhead_position" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`skip-forward-button`

### Verification
- cargo check -p composer
- 클릭 시 다음 경계로 점프
