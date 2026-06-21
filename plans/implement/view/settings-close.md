# Implementation Plan: Settings Close

## Feature ID
view/settings-close

## Spec Reference
`plans/spec/view/settings-close.md`

## Design Reference
`plans/design/view/settings-close.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_settings`

### Change Recipe
1. 설정 패널 닫기 버튼 렌더링
2. 클릭 시 show_settings = false
3. 설정 자동 저장

### Find Strategies
- `grep -rn "show_settings" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`settings-close`

### Verification
- cargo check -p view
