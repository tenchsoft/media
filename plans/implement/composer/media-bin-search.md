# Implementation Plan: Media Bin Search

## Feature ID
composer/media-bin-search

## Spec Reference
`plans/spec/composer/media-bin-search.md`

## Design Reference
`plans/design/composer/media-bin-search.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`
- Search: `grep -n "search" apps/composer/src-tauri/src/ui/left_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::media_search_query`

### Change Recipe
1. left_panel.rs에서 검색 필드 hit-test
2. 키 입력 → media_search_query 업데이트
3. 쿼리에 맞는 파일만 필터링하여 표시
4. 빈 쿼리 시 전체 목록 표시

### Find Strategies
- `grep -rn "media_search" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "search" apps/composer/src-tauri/src/ui/left_panel.rs`

### Debug ID
`media-bin-search`

### Verification
- cargo check -p composer
- 검색어 입력 시 필터링된 목록 표시
