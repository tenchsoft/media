# Implementation Plan: Subtitle Search Modal Close

## Feature ID
player/subtitle-search-modal-close

## Spec Reference
`plans/spec/player/subtitle-search-modal-close.md`

## Design Reference
`plans/design/player/subtitle-search-modal-close.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle_search\|search_modal" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_search_open`

### Change Recipe
1. paint_panels.rs에서 자막 검색 모달 닫기 버튼 렌더링
2. 클릭 시 subtitle_search_open = false
3. Escape 키로도 닫기

### Find Strategies
- `grep -rn "subtitle_search" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle_search" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`subtitle-search-modal-close`

### Verification
- cargo check -p player
