# Implementation Plan: Clip Drag To Timeline

## Feature ID
composer/clip-drag-to-timeline

## Spec Reference
`plans/spec/composer/clip-drag-to-timeline.md`

## Design Reference
`plans/design/composer/clip-drag-to-timeline.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`, `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "drag" apps/composer/src-tauri/src/ui/left_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::tracks`

### Change Recipe
1. left_panel.rs에서 미디어 빈 아이템 드래그 시작
2. timeline.rs에서 드롭 영역 hit-test
3. 드롭 시 새 Clip 생성 → 해당 Track의 clips 벡터에 추가
4. 시작 시간, 지속 시간 계산
5. 리렌더

### Find Strategies
- `grep -rn "drag" apps/composer/src-tauri/src/ui/left_panel.rs`
- `grep -rn "drop" apps/composer/src-tauri/src/ui/timeline.rs`
- `grep -rn "Clip" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`clip-drag-to-timeline`

### Verification
- cargo check -p composer
- 미디어 빈에서 타임라인으로 드래그 시 클립 생성
