# Implementation Plan: Add Chapter Modal Add

## Feature ID
player/add-chapter-modal-add

## Spec Reference
`plans/spec/player/add-chapter-modal-add.md`

## Design Reference
`plans/design/player/add-chapter-modal-add.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::chapters`, `PlayerState::show_add_chapter_modal`

### Change Recipe
1. 추가 버튼 클릭 시 입력된 제목 + 현재 재생 위치로 새 챕터 생성
2. chapters 벡터에 추가 (시간순 정렬)
3. show_add_chapter_modal = false, 모달 닫힘

### Find Strategies
- `grep -rn "show_add_chapter_modal" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`add-chapter-modal-add`

### Verification
- cargo check -p player
