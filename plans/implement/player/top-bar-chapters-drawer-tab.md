# Implementation Plan: Top Bar Chapters Drawer Tab

## Feature ID
player/top-bar-chapters-drawer-tab

## Spec Reference
`plans/spec/player/top-bar-chapters-drawer-tab.md`

## Design Reference
`plans/design/player/top-bar-chapters-drawer-tab.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::active_drawer_tab`

### Change Recipe
1. 상단 바 Chapters 탭 버튼 렌더링
2. 클릭 시 active_drawer_tab = Chapters
3. 챕터 목록 패널 열림

### Find Strategies
- `grep -rn "active_drawer_tab" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`top-bar-chapters-drawer-tab`

### Verification
- cargo check -p player
