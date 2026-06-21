# Implementation Plan: Subtitle Style Position Plus

## Feature ID
player/subtitle-style-position-plus

## Spec Reference
`plans/spec/player/subtitle-style-position-plus.md`

## Design Reference
`plans/design/player/subtitle-style-position-plus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style.position`

### Change Recipe
1. 클릭 시 자막 위치를 아래쪽으로 이동
2. 오버레이 자막 위치 즉시 반영

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`subtitle-style-position-plus`

### Verification
- cargo check -p player
