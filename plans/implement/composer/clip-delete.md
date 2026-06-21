# Implementation Plan: Clip Delete

## Feature ID
composer/clip-delete

## Spec Reference
`plans/spec/composer/clip-delete.md`

## Design Reference
`plans/design/composer/clip-delete.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "delete" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::clips`

### Change Recipe
1. 클립 선택 후 Delete 키 또는 컨텍스트 메뉴
2. 선택된 클립을 Track::clips에서 제거
3. 빈 공간 유지 (ripple 아님)
4. 리렌더

### Find Strategies
- `grep -rn "delete" apps/composer/src-tauri/src/ui/timeline.rs`
- `grep -rn "selected_clip" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`clip-delete`

### Verification
- cargo check -p composer
- 삭제 시 클립 제거, 빈 공간 유지
