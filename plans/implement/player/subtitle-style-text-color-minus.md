# Implementation Plan: Subtitle Style Text Color Minus

## Feature ID
player/subtitle-style-text-color-minus

## Spec Reference
`plans/spec/player/subtitle-style-text-color-minus.md`

## Design Reference
`plans/design/player/subtitle-style-text-color-minus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style.text_color`

### Change Recipe
1. 클릭 시 미리 정의된 색상 목록에서 이전 색상 선택
2. 오버레이 자막 텍스트 색상 즉시 반영

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`subtitle-style-text-color-minus`

### Verification
- cargo check -p player
