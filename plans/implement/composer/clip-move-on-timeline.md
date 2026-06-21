# Implementation Plan: Clip Move On Timeline

## Feature ID
composer/clip-move-on-timeline

## Spec Reference
`plans/spec/composer/clip-move-on-timeline.md`

## Design Reference
`plans/design/composer/clip-move-on-timeline.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "clip_move\|move_clip" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::start_time`

### Change Recipe
1. timeline.rs에서 클립 hit-test 영역 확인
2. 드래그 시작 → 클립 선택 + 드래그 오프셋 계산
3. 드래그 중 → Clip::start_time 업데이트 (스냅 적용)
4. 드롭 → 최종 위치 확정
5. 다른 트랙으로 드래그 시 트랙 이동

### Find Strategies
- `grep -rn "start_time" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "clip_move" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`clip-move-on-timeline`

### Verification
- cargo check -p composer
- 클립 드래그 시 위치 변경
