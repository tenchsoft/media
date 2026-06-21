# Implementation Plan: Automatic Seek Hover Thumbnail Preview

## Feature ID
player/automatic-seek-hover-thumbnail-preview

## Spec Reference
`plans/spec/player/automatic-seek-hover-thumbnail-preview.md`

## Design Reference
`plans/design/player/automatic-seek-hover-thumbnail-preview.md`
## Background Reference
`plans/background/player/automatic-seek-hover-thumbnail-preview.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "thumbnail\|preview\|seek_hover" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::seek_hover_position`

### Change Recipe
1. paint_controls.rs에서 시크바 위 마우스 위치 감지
2. 마우스 X 위치를 타임스탬프로 변환
3. 해당 위치의 썸네일 이미지 생성 (gst_backend 요청)
4. 툴팁으로 썸네일 + 시간 표시

### Find Strategies
- `grep -rn "seek_hover" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "thumbnail" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`automatic-seek-hover-thumbnail-preview`

### Verification
- cargo check -p player
- 시크바 호버 시 썸네일 표시
