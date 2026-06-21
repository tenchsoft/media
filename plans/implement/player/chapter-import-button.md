# Implementation Plan: Chapter Import Button

## Feature ID
player/chapter-import-button

## Spec Reference
`plans/spec/player/chapter-import-button.md`

## Design Reference
`plans/design/player/chapter-import-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "chapter\|import" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::chapters`

### Change Recipe
1. paint_panels.rs에서 챕터 가져오기 버튼 렌더링
2. 클릭 시 파일 대화상자 열기
3. 선택한 파일 파싱 (.srt, .txt)
4. 파싱된 챕터를 state.chapters에 추가

### Find Strategies
- `grep -rn "chapters" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "import" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`chapter-import-button`

### Verification
- cargo check -p player
- 챕터 파일 가져오기 동작
