# Implementation Plan: Top Bar Redo Button

## Feature ID
composer/top-bar-redo-button

## Spec Reference
`plans/spec/composer/top-bar-redo-button.md`

## Design Reference
`plans/design/composer/top-bar-redo-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "redo" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::history`

### Change Recipe
1. toolbar.rs에서 redo 버튼 hit-test 영역 확인
2. 클릭 이벤트 → history 스택에서 다음 상태 복원
3. 타임라인/클립/트랙 상태 재적용
4. 리렌더

### Find Strategies
- `grep -rn "redo" apps/composer/src-tauri/src/ui/toolbar.rs`
- `grep -rn "history" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`redo-button`

### Verification
- cargo check -p composer
- undo 후 redo 클릭 시 다시 실행 상태 복원
