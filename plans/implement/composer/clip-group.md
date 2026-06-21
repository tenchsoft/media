# Implementation Plan: Clip Group

## Feature ID
composer/clip-group

## Spec Reference
`plans/spec/composer/clip-group.md`

## Design Reference
`plans/design/composer/clip-group.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "group" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::group_id`

### Change Recipe
1. 다중 클립 선택 후 Ctrl+G
2. 선택된 클립들에 동일 group_id 할당
3. 그룹 내 클립은 함께 이동/트리밍
4. Ctrl+Shift+G로 그룹 해제

### Find Strategies
- `grep -rn "group_id" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "group" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`clip-group`

### Verification
- cargo check -p composer
- 그룹화 시 클립 함께 이동
