# Implementation Plan: Media Bin Sort

## Feature ID
composer/media-bin-sort

## Spec Reference
`plans/spec/composer/media-bin-sort.md`

## Design Reference
`plans/design/composer/media-bin-sort.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`
- Search: `grep -n "sort" apps/composer/src-tauri/src/ui/left_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::media_sort`

### Change Recipe
1. left_panel.rs에서 정렬 드롭다운/버튼 hit-test
2. 클릭 → 정렬 기준 변경 (이름, 날짜, 크기, 유형)
3. media_bin 벡터 재정렬
4. 리렌더

### Find Strategies
- `grep -rn "media_sort" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "sort" apps/composer/src-tauri/src/ui/left_panel.rs`

### Debug ID
`media-bin-sort`

### Verification
- cargo check -p composer
- 정렬 변경 시 목록 순서 변경
