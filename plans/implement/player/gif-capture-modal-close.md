# Implementation Plan: GIF Capture Modal Close

## Feature ID
player/gif-capture-modal-close

## Spec Reference
`plans/spec/player/gif-capture-modal-close.md`

## Design Reference
`plans/design/player/gif-capture-modal-close.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "gif_capture\|gif_modal" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::gif_capture_modal_open`

### Change Recipe
1. paint_panels.rs에서 GIF 캡처 모달 닫기 버튼 렌더링
2. 클릭 시 gif_capture_modal_open = false
3. 녹화 중이면 녹화도 중지

### Find Strategies
- `grep -rn "gif_capture" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "gif" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`gif-capture-modal-close`

### Verification
- cargo check -p player
