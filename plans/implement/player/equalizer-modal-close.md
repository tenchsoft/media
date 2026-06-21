# Implementation Plan: Equalizer Modal Close

## Feature ID
player/equalizer-modal-close

## Spec Reference
`plans/spec/player/equalizer-modal-close.md`

## Design Reference
`plans/design/player/equalizer-modal-close.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "equalizer\|eq_modal" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::eq_modal_open`

### Change Recipe
1. paint_panels.rs에서 EQ 모달 닫기 버튼 렌더링
2. 클릭 시 eq_modal_open = false
3. X 버튼 + Escape 키 지원

### Find Strategies
- `grep -rn "eq_modal_open" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "equalizer" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`equalizer-modal-close`

### Verification
- cargo check -p player
