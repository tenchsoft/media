# Implementation Plan: Clip Nest

## Feature ID
composer/clip-nest

## Spec Reference
`plans/spec/composer/clip-nest.md`

## Design Reference
`plans/design/composer/clip-nest.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "nest" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::nested_sequence`

### Change Recipe
1. 다중 클립 선택 후 nest 액션
2. 선택된 클립들을 새 중첩 시퀀스로 이동
3. 원래 위치에 중첩 클립 생성
4. 더블클릭 시 중첩 시퀀스 편집 진입

### Find Strategies
- `grep -rn "nested" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "nest" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`clip-nest`

### Verification
- cargo check -p composer
- nest 시 중첩 시퀀스 생성
