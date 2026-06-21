# Implementation Plan: Automatic Toast Lifecycle

## Feature ID
player/automatic-toast-lifecycle

## Spec Reference
`plans/spec/player/automatic-toast-lifecycle.md`

## Design Reference
`plans/design/player/automatic-toast-lifecycle.md`
## Background Reference
`plans/background/player/automatic-toast-lifecycle.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "toast" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::toasts`

### Change Recipe
1. state.toasts에 토스트 추가/만료 로직
2. paint_overlays.rs에서 활성 토스트 렌더링
3. 만료 시간(3초) 후 자동 제거
4. 여러 토스트 스택 표시

### Find Strategies
- `grep -rn "toast" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "toast" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`automatic-toast-lifecycle`

### Verification
- cargo check -p player
- 토스트가 3초 후 자동 사라짐
