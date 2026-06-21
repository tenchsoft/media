# Implementation Plan: Automatic Playback Position Progress

## Feature ID
player/automatic-playback-position-progress

## Spec Reference
`plans/spec/player/automatic-playback-position-progress.md`

## Design Reference
`plans/design/player/automatic-playback-position-progress.md`
## Background Reference
`plans/background/player/automatic-playback-position-progress.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "seekbar\|progress\|position" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::position`, `PlayerState::duration`

### Change Recipe
1. gst_backend.rs에서 position 콜백으로 state.position 갱신
2. paint_controls.rs에서 position/duration 비율로 시크바 채움
3. 재생 중 실시간 갱신 (타이머 기반)
4. 일시정지 시 마지막 위치 유지

### Find Strategies
- `grep -rn "position" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "seekbar\|progress" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`automatic-playback-position-progress`

### Verification
- cargo check -p player
- 재생 중 시크바 진행
