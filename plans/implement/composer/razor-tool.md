# Implementation Plan: Razor Tool

## Feature ID
composer/razor-tool

## Spec Reference
`plans/spec/composer/razor-tool.md`

## Design Reference
`plans/design/composer/razor-tool.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "razor" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::active_tool`

### Change Recipe
1. toolbar.rs에서 razor 버튼 hit-test
2. 클릭 → active_tool = Razor
3. 타임라인에서 클릭 시 clip-split-at-playhead 동작
4. 커서 아이콘 변경

### Find Strategies
- `grep -rn "active_tool" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "razor" apps/composer/src-tauri/src/ui/toolbar.rs`

### Debug ID
`razor-tool`

### Verification
- cargo check -p composer
- 선택 후 타임라인 클릭 시 클립 분할
