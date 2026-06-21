# Implementation Plan: Add Chapter Modal Cancel

## Feature ID
player/add-chapter-modal-cancel

## Spec Reference
`plans/spec/player/add-chapter-modal-cancel.md`

## Design Reference
`plans/design/player/add-chapter-modal-cancel.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::show_add_chapter_modal`

### Change Recipe
1. 취소 버튼 클릭 시 show_add_chapter_modal = false
2. 입력 필드 초기화, 모달 닫힘

### Find Strategies
- `grep -rn "show_add_chapter_modal" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`add-chapter-modal-cancel`

### Verification
- cargo check -p player
