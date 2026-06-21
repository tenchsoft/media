# Implementation Plan: Automatic Empty State Drop Prompt

## Feature ID
player/automatic-empty-state-drop-prompt

## Spec Reference
`plans/spec/player/automatic-empty-state-drop-prompt.md`

## Design Reference
`plans/design/player/automatic-empty-state-drop-prompt.md`
## Background Reference
`plans/background/player/automatic-empty-state-drop-prompt.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_video.rs`
- Search: `grep -n "empty\|drop" apps/player/src-tauri/src/ui/paint_video.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::has_media`

### Change Recipe
1. paint_video.rs에서 has_media가 false일 때 빈 상태 렌더링
2. 드래그 앤 드롭 안내 텍스트/아이콘 표시
3. 파일이 로드되면 has_media=true로 전환

### Find Strategies
- `grep -rn "has_media" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "empty" apps/player/src-tauri/src/ui/paint_video.rs`

### Debug ID
`automatic-empty-state-drop-prompt`

### Verification
- cargo check -p player
- 미디어 없을 때 드롭 안내 표시
