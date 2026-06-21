# Implementation Plan: Seekbar Position

## Feature ID
player/seekbar-position

## Spec Reference
`plans/spec/player/seekbar-position.md`

## Design Reference
`plans/design/player/seekbar-position.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "seekbar\|seek" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::position`, `PlayerState::duration`

### Change Recipe
1. paint_controls.rs에서 시크바 렌더링
2. 드래그 시 position 갱신
3. gst_backend.rs에서 seek 호출
4. 현재 위치/총 시간 텍스트 표시

### Find Strategies
- `grep -rn "position\|duration" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "seekbar" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`seekbar-position`

### Verification
- cargo check -p player
