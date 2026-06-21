# Implementation Plan: Subtitle Style Background Opacity Plus

## Feature ID
player/subtitle-style-background-opacity-plus

## Spec Reference
`plans/spec/player/subtitle-style-background-opacity-plus.md`

## Design Reference
`plans/design/player/subtitle-style-background-opacity-plus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style.bg_opacity`

### Change Recipe
1. 클릭 시 bg_opacity += 0.1 (최대 1.0)
2. 오버레이 자막 배경 불투명도 즉시 반영

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`subtitle-style-background-opacity-plus`

### Verification
- cargo check -p player
