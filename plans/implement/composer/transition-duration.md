# Implementation Plan: Transition Duration

## Feature ID
composer/transition-duration

## Spec Reference
`plans/spec/composer/transition-duration.md`

## Design Reference
`plans/design/composer/transition-duration.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "transition_duration" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Transition::duration`

### Change Recipe
1. inspector.rs에서 트랜지션 duration 슬라이더/입력 hit-test
2. 값 변경 → Transition::duration 업데이트
3. 타임라인 상 트랜지션 시각적 길이 변경
4. 리렌더

### Find Strategies
- `grep -rn "Transition" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "transition" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`transition-duration`

### Verification
- cargo check -p composer
- 지속 시간 변경 시 타임라인 반영
