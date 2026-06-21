# Implementation Plan: External Subtitle Offset Minus

## Feature ID
player/external-subtitle-offset-minus

## Spec Reference
`plans/spec/player/external-subtitle-offset-minus.md`

## Design Reference
`plans/design/player/external-subtitle-offset-minus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle_offset\|offset.*minus" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_offset_ms`

### Change Recipe
1. paint_panels.rs에서 자막 오프셋 - 버튼 렌더링
2. 클릭 시 subtitle_offset_ms -= 100ms
3. 자막 타이밍에 오프셋 적용

### Find Strategies
- `grep -rn "subtitle_offset" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "offset" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`external-subtitle-offset-minus`

### Verification
- cargo check -p player
