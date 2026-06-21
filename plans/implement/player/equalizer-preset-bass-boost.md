# Implementation Plan: Equalizer Preset Bass Boost

## Feature ID
player/equalizer-preset-bass-boost

## Spec Reference
`plans/spec/player/equalizer-preset-bass-boost.md`

## Design Reference
`plans/design/player/equalizer-preset-bass-boost.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "equalizer\|preset" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::eq_bands`

### Change Recipe
1. paint_panels.rs에서 EQ 프리셋 "Bass Boost" 버튼 렌더링
2. 클릭 시 eq_bands = [+6, +4, 0, 0, 0]
3. gst_backend.rs에서 밴드 게인 적용

### Find Strategies
- `grep -rn "eq_bands" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "preset" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`equalizer-preset-bass-boost`

### Verification
- cargo check -p player
