# Implementation Plan: Top Bar Info Drawer Tab

## Feature ID
player/top-bar-info-drawer-tab

## Spec Reference
`plans/spec/player/top-bar-info-drawer-tab.md`

## Design Reference
`plans/design/player/top-bar-info-drawer-tab.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::active_drawer_tab`

### Change Recipe
1. 상단 바 Info 탭 버튼 렌더링
2. 클릭 시 active_drawer_tab = Info
3. 미디어 정보 패널 열림

### Find Strategies
- `grep -rn "active_drawer_tab" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`top-bar-info-drawer-tab`

### Verification
- cargo check -p player
