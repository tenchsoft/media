# Implementation Plan: Equalizer Preset Vocal

## Feature ID
player/equalizer-preset-vocal

## Spec Reference
`plans/spec/player/equalizer-preset-vocal.md`

## Design Reference
`plans/design/player/equalizer-preset-vocal.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "equalizer\|preset" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::eq_bands`

### Change Recipe
1. paint_panels.rs에서 EQ 프리셋 "Vocal" 버튼 렌더링
2. 클릭 시 eq_bands = [-2, 0, +4, +3, 0]
3. gst_backend.rs에서 밴드 게인 적용

### Find Strategies
- `grep -rn "eq_bands" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "preset" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`equalizer-preset-vocal`

### Verification
- cargo check -p player
