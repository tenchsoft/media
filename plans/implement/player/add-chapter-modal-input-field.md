# Implementation Plan: Add Chapter Modal Input Field

## Feature ID
player/add-chapter-modal-input-field

## Spec Reference
`plans/spec/player/add-chapter-modal-input-field.md`

## Design Reference
`plans/design/player/add-chapter-modal-input-field.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::chapter_title_input`

### Change Recipe
1. 챕터 제목 입력 필드 렌더링
2. 키 입력 시 chapter_title_input 업데이트
3. 빈 입력 시 추가 버튼 비활성화

### Find Strategies
- `grep -rn "chapter_title_input" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`add-chapter-modal-input-field`

### Verification
- cargo check -p player
