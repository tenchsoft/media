# Implementation Plan: Automatic Click Region Refresh

## Feature ID
player/automatic-click-region-refresh

## Spec Reference
`plans/spec/player/automatic-click-region-refresh.md`

## Design Reference
`plans/design/player/automatic-click-region-refresh.md`
## Background Reference
`plans/background/player/automatic-click-region-refresh.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/app.rs`
- Search: `grep -n "hit_test\|click_region" apps/player/src-tauri/src/ui/app.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState`

### Change Recipe
1. state.rs에서 레이아웃/크기 변경 감지
2. app.rs의 hit-test 영역 재계산
3. 컨트롤 표시/숨김 상태에 따라 클릭 영역 업데이트
4. 리사이즈, 전체화면 전환 시 자동 갱신

### Find Strategies
- `grep -rn "hit_test" apps/player/src-tauri/src/ui/app.rs`
- `grep -rn "layout" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`automatic-click-region-refresh`

### Verification
- cargo check -p player
- 창 크기 변경 시 클릭 영역 정상 동작
