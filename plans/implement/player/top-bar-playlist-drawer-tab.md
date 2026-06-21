# Implementation Plan: Top Bar Playlist Drawer Tab

## Feature ID
player/top-bar-playlist-drawer-tab

## Spec Reference
`plans/spec/player/top-bar-playlist-drawer-tab.md`

## Design Reference
`plans/design/player/top-bar-playlist-drawer-tab.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::active_drawer_tab`

### Change Recipe
1. 상단 바 Playlist 탭 버튼 렌더링
2. 클릭 시 active_drawer_tab = Playlist
3. 재생 목록 패널 열림

### Find Strategies
- `grep -rn "active_drawer_tab" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`top-bar-playlist-drawer-tab`

### Verification
- cargo check -p player
