# Implementation Plan: Playlist Row Play

## Feature ID
player/playlist-row-play

## Spec Reference
`plans/spec/player/playlist-row-play.md`

## Design Reference
`plans/design/player/playlist-row-play.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "playlist.*row\|playlist.*play" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playlist`, `PlayerState::current_track_index`

### Change Recipe
1. paint_panels.rs에서 재생 목록 행 클릭 가능하게 렌더링
2. 더블클릭 시 해당 트랙 재생
3. current_track_index 갱신
4. gst_backend.rs에서 해당 파일 로드

### Find Strategies
- `grep -rn "playlist\|current_track" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "playlist" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`playlist-row-play`

### Verification
- cargo check -p player
