# Implementation Plan: Media Bin File List

## Feature ID
composer/media-bin-file-list

## Spec Reference
`plans/spec/composer/media-bin-file-list.md`

## Design Reference
`plans/design/composer/media-bin-file-list.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`
- Search: `grep -n "media_bin" apps/composer/src-tauri/src/ui/left_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::media_bin`

### Change Recipe
1. left_panel.rs에서 미디어 빈 리스트 렌더링
2. 각 파일 행: 썸네일 + 파일명 + 지속 시간
3. 클릭 시 미리보기 표시
4. 드래그 시작 지원

### Find Strategies
- `grep -rn "media_bin" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "media_bin" apps/composer/src-tauri/src/ui/left_panel.rs`

### Debug ID
`media-bin-file-list`

### Verification
- cargo check -p composer
- import된 파일 목록 표시
