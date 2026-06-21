# Implementation Plan: PIP Indicator Toggle

## Feature ID
player/pip-indicator-toggle

## Spec Reference
`plans/spec/player/pip-indicator-toggle.md`

## Design Reference
`plans/design/player/pip-indicator-toggle.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "pip\|picture_in_picture" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::pip_mode`

### Change Recipe
1. paint_overlays.rs에서 PIP 인디케이터 렌더링
2. 클릭 시 pip_mode 토글
3. 활성화 시 미니 플레이어 창 모드
4. 비활성화 시 일반 모드 복원

### Find Strategies
- `grep -rn "pip_mode" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "pip" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`pip-indicator-toggle`

### Verification
- cargo check -p player
