# Implementation Plan: Subtitles Search Button

## Feature ID
player/subtitles-search-button

## Spec Reference
`plans/spec/player/subtitles-search-button.md`

## Design Reference
`plans/design/player/subtitles-search-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle.*search\|search.*subtitle" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_search_open`

### Change Recipe
1. paint_panels.rs에서 자막 패널에 검색 버튼 렌더링
2. 클릭 시 subtitle_search_open = true
3. 검색 모달 열기

### Find Strategies
- `grep -rn "subtitle_search_open" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`subtitles-search-button`

### Verification
- cargo check -p player
