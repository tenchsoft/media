# Implementation Plan: Info Panel Close

## Feature ID
view/info-panel-close

## Spec Reference
`plans/spec/view/info-panel-close.md`

## Design Reference
`plans/design/view/info-panel-close.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_info_panel`

### Change Recipe
1. 정보 패널 닫기 버튼 렌더링
2. 클릭 시 show_info_panel = false

### Find Strategies
- `grep -rn "show_info_panel" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`info-panel-close`

### Verification
- cargo check -p view
