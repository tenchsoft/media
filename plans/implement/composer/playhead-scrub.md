# Implementation Plan: Playhead Scrub

## Feature ID
composer/playhead-scrub

## Spec Reference
`plans/spec/composer/playhead-scrub.md`

## Design Reference
`plans/design/composer/playhead-scrub.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "playhead\|scrub" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::playhead_position`

### Change Recipe
1. timeline.rs에서 타임라인 상단 룰러 hit-test 영역 확인
2. 클릭/드래그 → playhead_position 값 업데이트
3. 프리뷰 해당 프레임 렌더
4. 드래그 중 실시간 스크러빙

### Find Strategies
- `grep -rn "playhead" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "scrub" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`playhead-scrub`

### Verification
- cargo check -p composer
- 드래그 시 playhead 이동 및 프리뷰 업데이트
