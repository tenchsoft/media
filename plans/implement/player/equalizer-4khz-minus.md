# Implementation Plan: Equalizer 4kHz Minus

## Feature ID
player/equalizer-4khz-minus

## Spec Reference
`plans/spec/player/equalizer-4khz-minus.md`

## Design Reference
`plans/design/player/equalizer-4khz-minus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "equalizer\|eq_4k" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::eq_bands`

### Change Recipe
1. paint_panels.rs에서 EQ 패널에 4kHz - 버튼 렌더링
2. 클릭 시 eq_bands[3] -= 1dB (최소 -12dB)
3. gst_backend.rs에서 해당 밴드 게인 적용

### Find Strategies
- `grep -rn "eq_bands" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "equalizer" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`equalizer-4khz-minus`

### Verification
- cargo check -p player
