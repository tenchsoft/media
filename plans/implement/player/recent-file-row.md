# Implementation Plan: Recent File Row

## Feature ID
player/recent-file-row

## Spec Reference
`plans/spec/player/recent-file-row.md`

## Design Reference
`plans/design/player/recent-file-row.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- Search: `grep -n "recent\|recent_file" apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::recent_files`

### Change Recipe
1. paint_panels.rs에서 최근 파일 목록 행 렌더링
2. 각 행에 파일명 + 경로 + 재생 날짜 표시
3. 클릭 시 해당 파일 로드
4. 파일이 존재하지 않으면 회색 표시

### Find Strategies
- `grep -rn "recent_files" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "recent" apps/player/src-tauri/src/ui/paint_panels.rs`

### Debug ID
`recent-file-row`

### Verification
- cargo check -p player
