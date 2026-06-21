# Implementation Plan: Clip Ripple Delete

## Feature ID
composer/clip-ripple-delete

## Spec Reference
`plans/spec/composer/clip-ripple-delete.md`

## Design Reference
`plans/design/composer/clip-ripple-delete.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "ripple" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::clips`

### Change Recipe
1. 클립 선택 후 ripple-delete 액션
2. 선택된 클립 제거
3. 제거된 클립 이후의 모든 클립을 앞으로 이동
4. 빈 공간 없이 시프트
5. 리렌더

### Find Strategies
- `grep -rn "ripple" apps/composer/src-tauri/src/ui/timeline.rs`
- `grep -rn "clips" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`clip-ripple-delete`

### Verification
- cargo check -p composer
- ripple delete 시 후속 클립 앞으로 이동
