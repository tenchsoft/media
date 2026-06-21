# Implementation Plan: Playlist Row Remove

## Feature ID
player/playlist-row-remove

## Spec Reference
`plans/spec/player/playlist-row-remove.md`

## Design Reference
`plans/design/player/playlist-row-remove.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "playlist.*remove\|playlist.*delete" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playlist`

### Change Recipe
1. paint_panels.rs에서 각 재생 목록 행에 제거 버튼 렌더링
2. 클릭 시 playlist에서 해당 항목 제거
3. 현재 재생 중인 트랙이면 다음 트랙으로 전환

### Find Strategies
- `grep -rn "playlist" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "playlist" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`playlist-row-remove`

### Verification
- cargo check -p player
