# Implementation Plan: Clip Split At Playhead

## Feature ID
composer/clip-split-at-playhead

## Spec Reference
`plans/spec/composer/clip-split-at-playhead.md`

## Design Reference
`plans/design/composer/clip-split-at-playhead.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "split" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::playhead_position`

### Change Recipe
1. razor 도구 선택 상태에서 타임라인 클릭
2. playhead 위치와 겹치는 클립 찾기
3. 해당 클립을 두 개로 분할
4. 첫 번째 클립: 기존 start_time ~ playhead 위치
5. 두 번째 클립: playhead 위치 ~ 기존 end_time
6. tracks 벡터 업데이트

### Find Strategies
- `grep -rn "split" apps/composer/src-tauri/src/ui/timeline.rs`
- `grep -rn "playhead" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`clip-split-at-playhead`

### Verification
- cargo check -p composer
- razor 도구로 클릭 시 클립 분할
