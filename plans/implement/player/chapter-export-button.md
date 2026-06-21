# Implementation Plan: Chapter Export Button

## Feature ID
player/chapter-export-button

## Spec Reference
`plans/spec/player/chapter-export-button.md`

## Design Reference
`plans/design/player/chapter-export-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "chapter\|export" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::chapters`

### Change Recipe
1. paint_panels.rs에서 챕터 내보내기 버튼 렌더링
2. 클릭 시 chapters를 파일로 저장
3. 지원 포맷: .srt, .txt
4. 파일 대화상자로 저장 경로 선택

### Find Strategies
- `grep -rn "chapters" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "chapter" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`chapter-export-button`

### Verification
- cargo check -p player
- 챕터 파일 내보내기 동작
