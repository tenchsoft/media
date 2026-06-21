# Implementation Plan: Help Modal Close

## Feature ID
player/help-modal-close

## Spec Reference
`plans/spec/player/help-modal-close.md`

## Design Reference
`plans/design/player/help-modal-close.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "help_modal\|help" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::help_modal_open`

### Change Recipe
1. paint_panels.rs에서 도움말 모달 닫기 버튼 렌더링
2. 클릭 시 help_modal_open = false
3. X 버튼 + Escape 키 지원

### Find Strategies
- `grep -rn "help_modal_open" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "help" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`help-modal-close`

### Verification
- cargo check -p player
