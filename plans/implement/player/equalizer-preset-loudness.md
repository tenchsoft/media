# Implementation Plan: Equalizer Preset Loudness

## Feature ID
player/equalizer-preset-loudness

## Spec Reference
`plans/spec/player/equalizer-preset-loudness.md`

## Design Reference
`plans/design/player/equalizer-preset-loudness.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "equalizer\|preset" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::eq_bands`

### Change Recipe
1. paint_panels.rs에서 EQ 프리셋 "Loudness" 버튼 렌더링
2. 클릭 시 eq_bands = [+4, +2, -2, +2, +4]
3. gst_backend.rs에서 밴드 게인 적용

### Find Strategies
- `grep -rn "eq_bands" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "preset" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`equalizer-preset-loudness`

### Verification
- cargo check -p player
