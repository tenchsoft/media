# Implementation Plan: Bottom AB Loop Button

## Feature ID
player/bottom-ab-loop-button

## Spec Reference
`plans/spec/player/bottom-ab-loop-button.md`

## Design Reference
`plans/design/player/bottom-ab-loop-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "ab_loop\|loop_a\|loop_b" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::loop_a`, `PlayerState::loop_b`

### Change Recipe
1. paint_controls.rs에서 AB 루프 버튼 렌더링
2. 첫 클릭: loop_a = 현재 position
3. 두 번째 클릭: loop_b = 현재 position, 루프 활성화
4. 세 번째 클릭: 루프 해제, A/B 초기화
5. gst_backend.rs에서 loop_a~loop_b 구간 반복 재생

### Find Strategies
- `grep -rn "loop_a\|loop_b" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "ab_loop" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-ab-loop-button`

### Verification
- cargo check -p player
- A-B 구간 반복 재생 동작
