# Implementation Plan: Keyframe Delete

## Feature ID
composer/keyframe-delete

## Spec Reference
`plans/spec/composer/keyframe-delete.md`

## Design Reference
`plans/design/composer/keyframe-delete.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "keyframe_delete" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::keyframes`

### Change Recipe
1. inspector.rs에서 키프레임 선택 후 삭제 버튼 hit-test
2. 클릭 → Clip::keyframes에서 해당 키프레임 제거
3. 인접 키프레임 간 보간 재계산
4. 리렌더

### Find Strategies
- `grep -rn "keyframe" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "delete" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`keyframe-delete`

### Verification
- cargo check -p composer
- 삭제 시 키프레임 제거
