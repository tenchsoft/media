# Implementation Plan: Context Menu Open File Item

## Feature ID
player/context-menu-open-file-item

## Spec Reference
`plans/spec/player/context-menu-open-file-item.md`

## Design Reference
`plans/design/player/context-menu-open-file-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|open_file" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::current_file`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 파일 열기 항목 추가
2. 클릭 시 파일 대화상자 열기
3. 선택한 파일을 gst_backend로 로드

### Find Strategies
- `grep -rn "current_file" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-open-file-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 파일 열기
