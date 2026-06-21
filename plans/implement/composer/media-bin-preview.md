# Implementation Plan: Media Bin Preview

## Feature ID
composer/media-bin-preview

## Spec Reference
`plans/spec/composer/media-bin-preview.md`

## Design Reference
`plans/design/composer/media-bin-preview.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`
- Search: `grep -n "preview" apps/composer/src-tauri/src/ui/left_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::selected_media`

### Change Recipe
1. left_panel.rs에서 미디어 빈 아이템 클릭
2. selected_media 업데이트
3. 썸네일/프리뷰 영역에 선택된 미디어 표시
4. 메타데이터(해상도, 코덱, 지속 시간) 표시

### Find Strategies
- `grep -rn "selected_media" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "preview" apps/composer/src-tauri/src/ui/left_panel.rs`

### Debug ID
`media-bin-preview`

### Verification
- cargo check -p composer
- 파일 선택 시 미리보기 표시
