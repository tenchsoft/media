# Implementation Plan: Subtitle Style Shadow Offset Minus

## Feature ID
player/subtitle-style-shadow-offset-minus

## Spec Reference
`plans/spec/player/subtitle-style-shadow-offset-minus.md`

## Design Reference
`plans/design/player/subtitle-style-shadow-offset-minus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style.shadow_offset`

### Change Recipe
1. 클릭 시 shadow_offset -= 1px (최소 0px)
2. 오버레이 자막 그림자 오프셋 즉시 반영

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`subtitle-style-shadow-offset-minus`

### Verification
- cargo check -p player
