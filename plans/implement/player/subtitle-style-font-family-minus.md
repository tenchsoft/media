# Implementation Plan: Subtitle Style Font Family Minus

## Feature ID
player/subtitle-style-font-family-minus

## Spec Reference
`plans/spec/player/subtitle-style-font-family-minus.md`

## Design Reference
`plans/design/player/subtitle-style-font-family-minus.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_style.font_family_index`

### Change Recipe
1. 클릭 시 font_family_index -= 1 (순환)
2. 사용 가능한 글꼴 목록에서 이전 글꼴 선택

### Find Strategies
- `grep -rn "subtitle_style" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`subtitle-style-font-family-minus`

### Verification
- cargo check -p player
