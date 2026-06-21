# Implementation Plan: Builtin Subtitle Track Row

## Feature ID
player/builtin-subtitle-track-row

## Spec Reference
`plans/spec/player/builtin-subtitle-track-row.md`

## Design Reference
`plans/design/player/builtin-subtitle-track-row.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "subtitle_track\|subtitle_row" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::subtitle_tracks`, `PlayerState::active_subtitle_track`

### Change Recipe
1. paint_panels.rs에서 내장 자막 트랙 목록 렌더링
2. 각 행에 트랙 언어/이름 표시
3. 클릭 시 active_subtitle_track = 해당 트랙 인덱스
4. gst_backend.rs에서 해당 자막 트랙 활성화

### Find Strategies
- `grep -rn "subtitle_tracks" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`builtin-subtitle-track-row`

### Verification
- cargo check -p player
- 자막 트랙 선택 시 해당 자막 표시
