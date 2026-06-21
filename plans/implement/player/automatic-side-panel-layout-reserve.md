# Implementation Plan: Automatic Side Panel Layout Reserve

## Feature ID
player/automatic-side-panel-layout-reserve

## Spec Reference
`plans/spec/player/automatic-side-panel-layout-reserve.md`

## Design Reference
`plans/design/player/automatic-side-panel-layout-reserve.md`
## Background Reference
`plans/background/player/automatic-side-panel-layout-reserve.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "panel\|side_panel\|layout" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::active_panel`

### Change Recipe
1. paint_panels.rs에서 사이드 패널 레이아웃 계산
2. 패널 열림/닫힘 시 비디오 영역 리사이즈
3. 패널 너비를 레이아웃에서 예약
4. 애니메이션 없이 즉시 전환

### Find Strategies
- `grep -rn "active_panel" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "panel" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`automatic-side-panel-layout-reserve`

### Verification
- cargo check -p player
- 패널 토글 시 비디오 영역 조정
