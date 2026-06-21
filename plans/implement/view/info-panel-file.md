# Implementation Plan: Info Panel File

## Feature ID
view/info-panel-file

## Spec Reference
`plans/spec/view/info-panel-file.md`

## Design Reference
`plans/design/view/info-panel-file.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 정보 패널에서 파일 탭 렌더링
2. 파일명, 경로, 크기, 해상도, 포맷, 생성/수정 날짜 표시

### Find Strategies
- `grep -rn "current_image" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`info-panel-file`

### Verification
- cargo check -p view
