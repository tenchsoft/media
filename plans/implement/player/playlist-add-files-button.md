# Implementation Plan: Playlist Add Files Button

## Feature ID
player/playlist-add-files-button

## Spec Reference
`plans/spec/player/playlist-add-files-button.md`

## Design Reference
`plans/design/player/playlist-add-files-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "playlist.*add\|add.*playlist" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playlist`

### Change Recipe
1. paint_panels.rs에서 재생 목록에 파일 추가 버튼 렌더링
2. 클릭 시 파일 대화상자 열기 (다중 선택)
3. 선택한 파일을 playlist에 추가
4. 재생 목록 갱신

### Find Strategies
- `grep -rn "playlist" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "playlist" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`playlist-add-files-button`

### Verification
- cargo check -p player
