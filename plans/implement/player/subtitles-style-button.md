# Implementation Plan: Subtitles Style Button

## Feature ID
player/subtitles-style-button

## Spec Reference
`plans/spec/player/subtitles-style-button.md`

## Design Reference
`plans/design/player/subtitles-style-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle.*style\|style.*subtitle" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style_modal_open`

### Change Recipe
1. paint_panels.rs에서 자막 패널에 스타일 버튼 렌더링
2. 클릭 시 subtitle_style_modal_open = true
3. 자막 스타일 편집 모달 열기

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`subtitles-style-button`

### Verification
- cargo check -p player
