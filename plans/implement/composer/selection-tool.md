# Implementation Plan: Selection Tool

## Feature ID
composer/selection-tool

## Spec Reference
`plans/spec/composer/selection-tool.md`

## Design Reference
`plans/design/composer/selection-tool.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "selection" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::active_tool`

### Change Recipe
1. toolbar.rs에서 selection 버튼 hit-test
2. 클릭 → active_tool = Selection
3. 타임라인에서 클립 클릭 시 선택
4. 다중 선택 지원 (Shift+클릭)
5. 커서 아이콘 변경

### Find Strategies
- `grep -rn "active_tool" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "selection" apps/composer/src-tauri/src/ui/toolbar.rs`

### Debug ID
`selection-tool`

### Verification
- cargo check -p composer
- 선택 후 클립 클릭 시 선택 표시
