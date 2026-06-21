# Implementation Plan: Chapter Row Delete Button

## Feature ID
player/chapter-row-delete-button

## Spec Reference
`plans/spec/player/chapter-row-delete-button.md`

## Design Reference
`plans/design/player/chapter-row-delete-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "chapter.*delete\|delete.*chapter" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::chapters`

### Change Recipe
1. paint_panels.rs에서 각 챕터 행에 삭제 버튼 렌더링
2. 클릭 시 chapters에서 해당 인덱스 제거
3. 챕터 목록 즉시 갱신

### Find Strategies
- `grep -rn "chapters" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "chapter" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`chapter-row-delete-button`

### Verification
- cargo check -p player
- 챕터 삭제 동작
