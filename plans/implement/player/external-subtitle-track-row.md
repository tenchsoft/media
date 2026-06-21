# Implementation Plan: External Subtitle Track Row

## Feature ID
player/external-subtitle-track-row

## Spec Reference
`plans/spec/player/external-subtitle-track-row.md`

## Design Reference
`plans/design/player/external-subtitle-track-row.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "external_subtitle\|subtitle.*row" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::external_subtitles`

### Change Recipe
1. paint_panels.rs에서 외부 자막 목록 행 렌더링
2. 각 행에 파일명 표시
3. 클릭 시 해당 자막 로드
4. 활성 자막 행 하이라이트

### Find Strategies
- `grep -rn "external_subtitles" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "subtitle" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`external-subtitle-track-row`

### Verification
- cargo check -p player
