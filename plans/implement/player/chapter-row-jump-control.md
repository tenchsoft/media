# Implementation Plan: Chapter Row Jump Control

## Feature ID
player/chapter-row-jump-control

## Spec Reference
`plans/spec/player/chapter-row-jump-control.md`

## Design Reference
`plans/design/player/chapter-row-jump-control.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "chapter.*jump\|chapter.*click" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::chapters`

### Change Recipe
1. paint_panels.rs에서 챕터 행 클릭 가능하게 렌더링
2. 클릭 시 해당 챕터의 시작 시간으로 seek
3. gst_backend.rs에서 seek 호출

### Find Strategies
- `grep -rn "chapters" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "chapter" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`chapter-row-jump-control`

### Verification
- cargo check -p player
- 챕터 클릭 시 해당 위치로 이동
