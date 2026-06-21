# Implementation Plan: Redo Shortcut

## Feature ID
composer/redo-shortcut

## Spec Reference
`plans/spec/composer/redo-shortcut.md`

## Design Reference
`plans/design/composer/redo-shortcut.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/mod.rs`
- Search: `grep -n "redo\|Ctrl.*Shift.*Z" apps/composer/src-tauri/src/ui/mod.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::history`

### Change Recipe
1. mod.rs에서 Ctrl+Shift+Z 키 이벤트 감지
2. history 스택에서 다음 상태 복원
3. 타임라인/클립 상태 재적용
4. 리렌더

### Find Strategies
- `grep -rn "redo" apps/composer/src-tauri/src/ui/mod.rs`
- `grep -rn "history" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`redo-shortcut`

### Verification
- cargo check -p composer
- Ctrl+Shift+Z 시 재실행 상태 복원
