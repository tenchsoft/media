# Implementation Plan: Subtitle Encoding Option

## Feature ID
player/subtitle-encoding-option

## Spec Reference
`plans/spec/player/subtitle-encoding-option.md`

## Design Reference
`plans/design/player/subtitle-encoding-option.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "encoding\|subtitle_encoding" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_encoding`

### Change Recipe
1. paint_panels.rs에서 자막 패널에 인코딩 옵션 드롭다운 렌더링
2. 옵션: UTF-8, EUC-KR, CP949, Shift-JIS 등
3. 선택 시 subtitle_encoding 변경
4. 자막 다시 파싱

### Find Strategies
- `grep -rn "subtitle_encoding" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "encoding" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`subtitle-encoding-option`

### Verification
- cargo check -p player
