# Implementation Plan: Undo Shortcut

## Feature ID
composer/undo-shortcut

## Spec Reference
`plans/spec/composer/undo-shortcut.md`

## Design Reference
`plans/design/composer/undo-shortcut.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/mod.rs`
- Search: `grep -n "undo\|Ctrl.*Z" apps/composer/src-tauri/src/ui/mod.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::history`

### Change Recipe
1. mod.rs에서 Ctrl+Z 키 이벤트 감지
2. history 스택에서 이전 상태 복원
3. 타임라인/클립 상태 롤백
4. 리렌더

### Find Strategies
- `grep -rn "undo" apps/composer/src-tauri/src/ui/mod.rs`
- `grep -rn "history" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`undo-shortcut`

### Verification
- cargo check -p composer
- Ctrl+Z 시 이전 상태 복원
