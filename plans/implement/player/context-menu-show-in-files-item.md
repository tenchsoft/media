# Implementation Plan: Context Menu Show In Files Item

## Feature ID
player/context-menu-show-in-files-item

## Spec Reference
`plans/spec/player/context-menu-show-in-files-item.md`

## Design Reference
`plans/design/player/context-menu-show-in-files-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|show_in_files" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::current_file`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 "파일 탐색기에서 보기" 항목 추가
2. 클릭 시 OS 파일 탐색기에서 파일 위치 열기
3. Tauri shell API 사용

### Find Strategies
- `grep -rn "current_file" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "show_in_files" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-show-in-files-item`

### Verification
- cargo check -p player
- 파일 탐색기에서 파일 위치 열기
