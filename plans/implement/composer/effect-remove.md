# Implementation Plan: Effect Remove

## Feature ID
composer/effect-remove

## Spec Reference
`plans/spec/composer/effect-remove.md`

## Design Reference
`plans/design/composer/effect-remove.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "effect_remove\|remove_effect" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::effects`

### Change Recipe
1. inspector.rs에서 적용된 이펙트 목록의 삭제 버튼 hit-test
2. 클릭 → Clip::effects에서 해당 이펙트 제거
3. 프리뷰 업데이트
4. 리렌더

### Find Strategies
- `grep -rn "effects" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "remove" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`effect-remove`

### Verification
- cargo check -p composer
- 삭제 시 이펙트 제거 및 프리뷰 반영
