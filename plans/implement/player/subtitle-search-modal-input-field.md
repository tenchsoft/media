# Implementation Plan: Subtitle Search Modal Input Field

## Feature ID
player/subtitle-search-modal-input-field

## Spec Reference
`plans/spec/player/subtitle-search-modal-input-field.md`

## Design Reference
`plans/design/player/subtitle-search-modal-input-field.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle_search\|search_input" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_search_query`

### Change Recipe
1. paint_panels.rs에서 자막 검색 입력 필드 렌더링
2. 타이핑 시 subtitle_search_query 갱신
3. 실시간으로 자막에서 검색
4. 매칭 결과 목록 표시

### Find Strategies
- `grep -rn "subtitle_search_query" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle_search" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`subtitle-search-modal-input-field`

### Verification
- cargo check -p player
