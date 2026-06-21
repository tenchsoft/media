# Implementation Plan: Top Bar Undo Button

## Feature ID
composer/top-bar-undo-button

## Spec Reference
`plans/spec/composer/top-bar-undo-button.md`

## Design Reference
`plans/design/composer/top-bar-undo-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "undo" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::history`

### Change Recipe
1. toolbar.rs에서 undo 버튼 hit-test 영역 확인
2. 클릭 이벤트 → history 스택에서 이전 상태 복원
3. 타임라인/클립/트랙 상태 롤백
4. 리렌더

### Find Strategies
- `grep -rn "undo" apps/composer/src-tauri/src/ui/toolbar.rs`
- `grep -rn "history" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`undo-button`

### Verification
- cargo check -p composer
- 편집 후 undo 클릭 시 이전 상태 복원
