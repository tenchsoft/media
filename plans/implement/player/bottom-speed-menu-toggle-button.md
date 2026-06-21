# Implementation Plan: Bottom Speed Menu Toggle Button

## Feature ID
player/bottom-speed-menu-toggle-button

## Spec Reference
`plans/spec/player/bottom-speed-menu-toggle-button.md`

## Design Reference
`plans/design/player/bottom-speed-menu-toggle-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "speed\|playback_rate" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playback_speed`, `PlayerState::speed_menu_open`

### Change Recipe
1. paint_controls.rs에서 속도 메뉴 버튼 렌더링
2. 클릭 시 speed_menu_open 토글
3. 팝업 메뉴에 속도 옵션 표시 (0.25x ~ 4x)
4. 선택 시 gst_backend.rs에서 재생 속도 변경

### Find Strategies
- `grep -rn "playback_speed\|speed_menu" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "speed" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-speed-menu-toggle-button`

### Verification
- cargo check -p player
- 속도 메뉴 열기/속도 변경 동작
