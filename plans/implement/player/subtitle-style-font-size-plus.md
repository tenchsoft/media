# Implementation Plan: Subtitle Style Font Size Plus

## Feature ID
player/subtitle-style-font-size-plus

## Spec Reference
`plans/spec/player/subtitle-style-font-size-plus.md`

## Design Reference
`plans/design/player/subtitle-style-font-size-plus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle_style\|font_size" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style.font_size`

### Change Recipe
1. paint_panels.rs에서 자막 스타일 모달에 글꼴 크기 + 버튼 렌더링
2. 클릭 시 font_size += 2px (최대 72px)
3. 오버레이 자막에 즉시 반영

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "font_size" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`subtitle-style-font-size-plus`

### Verification
- cargo check -p player
