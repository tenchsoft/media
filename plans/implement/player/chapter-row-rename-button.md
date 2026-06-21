# Implementation Plan: Chapter Row Rename Button

## Feature ID
player/chapter-row-rename-button

## Spec Reference
`plans/spec/player/chapter-row-rename-button.md`

## Design Reference
`plans/design/player/chapter-row-rename-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "chapter.*rename\|rename.*chapter" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::chapters`, `PlayerState::editing_chapter`

### Change Recipe
1. paint_panels.rs에서 각 챕터 행에 이름 편집 버튼 렌더링
2. 클릭 시 인라인 텍스트 편집 모드 진입
3. Enter 시 챕터 이름 갱신
4. Escape 시 편집 취소

### Find Strategies
- `grep -rn "editing_chapter" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "chapter" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`chapter-row-rename-button`

### Verification
- cargo check -p player
- 챕터 이름 편집 동작
