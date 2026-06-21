# Implementation Plan: Builtin Subtitle None Button

## Feature ID
player/builtin-subtitle-none-button

## Spec Reference
`plans/spec/player/builtin-subtitle-none-button.md`

## Design Reference
`plans/design/player/builtin-subtitle-none-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle\|subtitle_none" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::active_subtitle_track`

### Change Recipe
1. paint_panels.rs에서 자막 패널에 "없음" 옵션 렌더링
2. 클릭 시 active_subtitle_track = None
3. gst_backend.rs에서 자막 트랙 비활성화
4. 오버레이 자막 제거

### Find Strategies
- `grep -rn "active_subtitle_track" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`builtin-subtitle-none-button`

### Verification
- cargo check -p player
- 자막 없음 선택 시 자막 사라짐
