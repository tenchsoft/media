# Implementation Plan: Subtitle Search Modal Prev

## Feature ID
player/subtitle-search-modal-prev

## Spec Reference
`plans/spec/player/subtitle-search-modal-prev.md`

## Design Reference
`plans/design/player/subtitle-search-modal-prev.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle_search\|find_prev" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_search_results`, `PlayerState::subtitle_search_index`

### Change Recipe
1. paint_panels.rs에서 "이전 찾기" 버튼 렌더링
2. 클릭 시 subtitle_search_index -= 1
3. 해당 자막 위치로 seek
4. 첫 결과면 마지막으로 순환

### Find Strategies
- `grep -rn "subtitle_search" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "find_prev" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`subtitle-search-modal-prev`

### Verification
- cargo check -p player
