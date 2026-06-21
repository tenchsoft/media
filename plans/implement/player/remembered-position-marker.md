# Implementation Plan: Remembered Position Marker

## Feature ID
player/remembered-position-marker

## Spec Reference
`plans/spec/player/remembered-position-marker.md`

## Design Reference
`plans/design/player/remembered-position-marker.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "remembered\|bookmark" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::remembered_position`

### Change Recipe
1. paint_controls.rs에서 시크바에 기억된 위치 마커 렌더링
2. 파일 로드 시 remembered_position 복원
3. 마커 위치에 작은 표시기 표시
4. 파일 종료 시 현재 position 저장

### Find Strategies
- `grep -rn "remembered_position" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "remembered" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`remembered-position-marker`

### Verification
- cargo check -p player
