# Implementation Plan: Equalizer 60Hz Plus

## Feature ID
player/equalizer-60hz-plus

## Spec Reference
`plans/spec/player/equalizer-60hz-plus.md`

## Design Reference
`plans/design/player/equalizer-60hz-plus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "equalizer\|eq_60" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::eq_bands`

### Change Recipe
1. paint_panels.rs에서 EQ 패널에 60Hz + 버튼 렌더링
2. 클릭 시 eq_bands[0] += 1dB (최대 +12dB)
3. gst_backend.rs에서 해당 밴드 게인 적용

### Find Strategies
- `grep -rn "eq_bands" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "equalizer" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`equalizer-60hz-plus`

### Verification
- cargo check -p player
- 60Hz 게인 증가 동작
