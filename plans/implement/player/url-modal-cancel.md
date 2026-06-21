# Implementation Plan: URL Modal Cancel

## Feature ID
player/url-modal-cancel

## Spec Reference
`plans/spec/player/url-modal-cancel.md`

## Design Reference
`plans/design/player/url-modal-cancel.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::show_url_modal`

### Change Recipe
1. 취소 버튼 클릭 시 show_url_modal = false
2. 입력 필드 초기화, 모달 닫힘

### Find Strategies
- `grep -rn "show_url_modal" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`url-modal-cancel`

### Verification
- cargo check -p player
