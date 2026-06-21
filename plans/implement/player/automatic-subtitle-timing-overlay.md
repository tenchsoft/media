# Implementation Plan: Automatic Subtitle Timing Overlay

## Feature ID
player/automatic-subtitle-timing-overlay

## Spec Reference
`plans/spec/player/automatic-subtitle-timing-overlay.md`

## Design Reference
`plans/design/player/automatic-subtitle-timing-overlay.md`
## Background Reference
`plans/background/player/automatic-subtitle-timing-overlay.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "subtitle\|caption" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::current_subtitle`

### Change Recipe
1. gst_backend.rs에서 자막 텍스트/타이밍 파싱
2. 현재 position과 매칭되는 자막 선택
3. state.current_subtitle에 저장
4. paint_overlays.rs에서 자막 텍스트를 비디오 위에 오버레이

### Find Strategies
- `grep -rn "subtitle" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`automatic-subtitle-timing-overlay`

### Verification
- cargo check -p player
- 자막이 타이밍에 맞게 오버레이
