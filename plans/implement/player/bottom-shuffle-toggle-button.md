# Implementation Plan: Bottom Shuffle Toggle Button

## Feature ID
player/bottom-shuffle-toggle-button

## Spec Reference
`plans/spec/player/bottom-shuffle-toggle-button.md`

## Design Reference
`plans/design/player/bottom-shuffle-toggle-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "shuffle" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::shuffle`

### Change Recipe
1. paint_controls.rs에서 셔플 버튼 렌더링
2. 클릭 시 state.shuffle 토글
3. 활성화 시 재생 목록 순서 랜덤화
4. 비활성화 시 원래 순서 복원

### Find Strategies
- `grep -rn "shuffle" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "shuffle" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-shuffle-toggle-button`

### Verification
- cargo check -p player
- 셔플 토글 동작
