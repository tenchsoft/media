# Implementation Plan: Equalizer Preset Flat

## Feature ID
player/equalizer-preset-flat

## Spec Reference
`plans/spec/player/equalizer-preset-flat.md`

## Design Reference
`plans/design/player/equalizer-preset-flat.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "equalizer\|preset" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::eq_bands`

### Change Recipe
1. paint_panels.rs에서 EQ 프리셋 "Flat" 버튼 렌더링
2. 클릭 시 모든 eq_bands = 0dB
3. gst_backend.rs에서 모든 밴드 리셋

### Find Strategies
- `grep -rn "eq_bands" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "preset" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`equalizer-preset-flat`

### Verification
- cargo check -p player
