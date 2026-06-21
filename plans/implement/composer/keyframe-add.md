# Implementation Plan: Keyframe Add

## Feature ID
composer/keyframe-add

## Spec Reference
`plans/spec/composer/keyframe-add.md`

## Design Reference
`plans/design/composer/keyframe-add.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "keyframe" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::keyframes`

### Change Recipe
1. inspector.rs에서 속성별 다이아몬드 버튼 hit-test
2. 클릭 → 현재 playhead 위치에 새 Keyframe 추가
3. 현재 속성 값을 키프레임에 저장
4. 타임라인 키프레임 트랙에 마커 표시

### Find Strategies
- `grep -rn "keyframe" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "keyframe" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`keyframe-add`

### Verification
- cargo check -p composer
- 클릭 시 키프레임 생성
