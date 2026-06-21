# Implementation Plan: Transition Drag Between Clips

## Feature ID
composer/transition-drag-between-clips

## Spec Reference
`plans/spec/composer/transition-drag-between-clips.md`

## Design Reference
`plans/design/composer/transition-drag-between-clips.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`, `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "transition" apps/composer/src-tauri/src/ui/`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::transitions`

### Change Recipe
1. left_panel.rs에서 트랜지션 아이템 드래그 시작
2. timeline.rs에서 두 클립 사이 갭에 드롭
3. 새 Transition 생성 → Track::transitions에 추가
4. 기본 지속 시간 설정
5. 리렌더

### Find Strategies
- `grep -rn "transition" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "transition" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`transition-drag-between-clips`

### Verification
- cargo check -p composer
- 드롭 시 트랜지션 생성
