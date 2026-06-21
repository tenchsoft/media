# Implementation Plan: Subtitle Style Modal Close

## Feature ID
player/subtitle-style-modal-close

## Spec Reference
`plans/spec/player/subtitle-style-modal-close.md`

## Design Reference
`plans/design/player/subtitle-style-modal-close.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::show_subtitle_style_modal`

### Change Recipe
1. 닫기 버튼 클릭 시 show_subtitle_style_modal = false
2. 모달 닫힘, 이전 패널 복원

### Find Strategies
- `grep -rn "show_subtitle_style_modal" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`subtitle-style-modal-close`

### Verification
- cargo check -p player
