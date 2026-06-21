# Implementation Plan: Subtitle Style Stroke Width Minus

## Feature ID
player/subtitle-style-stroke-width-minus

## Spec Reference
`plans/spec/player/subtitle-style-stroke-width-minus.md`

## Design Reference
`plans/design/player/subtitle-style-stroke-width-minus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style.stroke_width`

### Change Recipe
1. 클릭 시 stroke_width -= 1px (최소 0px)
2. 오버레이 자막 외곽선 두께 즉시 반영

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`subtitle-style-stroke-width-minus`

### Verification
- cargo check -p player
