# Implementation Plan: Speed Menu Option

## Feature ID
player/speed-menu-option

## Spec Reference
`plans/spec/player/speed-menu-option.md`

## Design Reference
`plans/design/player/speed-menu-option.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "speed\|playback_rate" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playback_speed`

### Change Recipe
1. paint_controls.rs에서 속도 메뉴 팝업에 옵션 행 렌더링
2. 옵션: 0.25x, 0.5x, 0.75x, 1x, 1.25x, 1.5x, 2x, 4x
3. 클릭 시 playback_speed 설정
4. gst_backend.rs에서 재생 속도 변경

### Find Strategies
- `grep -rn "playback_speed" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "speed" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`speed-menu-option`

### Verification
- cargo check -p player
