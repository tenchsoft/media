# Implementation Plan: Insert Clip

## Feature ID
composer/insert-clip

## Spec Reference
`plans/spec/composer/insert-clip.md`

## Design Reference
`plans/design/composer/insert-clip.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "insert" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::clips`

### Change Recipe
1. playhead 위치에서 insert 액션
2. 소스 클립을 playhead 위치에 삽입
3. 기존 클립들을 뒤로 밀어냄
4. 리렌더

### Find Strategies
- `grep -rn "insert" apps/composer/src-tauri/src/ui/timeline.rs`
- `grep -rn "clips" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`insert-clip`

### Verification
- cargo check -p composer
- 삽입 시 기존 클립 뒤로 이동
