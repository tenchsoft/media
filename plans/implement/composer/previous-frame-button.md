# Implementation Plan: Previous Frame Button

## Feature ID
composer/previous-frame-button

## Spec Reference
`plans/spec/composer/previous-frame-button.md`

## Design Reference
`plans/design/composer/previous-frame-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "prev_frame\|previous_frame" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::playhead_position`

### Change Recipe
1. timeline_panel.rs에서 prev-frame 버튼 hit-test 영역 확인
2. 클릭 이벤트 → playhead_position -= 1프레임 (최소 0)
3. 프리뷰 해당 프레임 표시

### Find Strategies
- `grep -rn "prev_frame\|previous_frame" apps/composer/src-tauri/src/ui/`
- `grep -rn "frame_rate" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`previous-frame-button`

### Verification
- cargo check -p composer
- 클릭 시 1프레임 후진
