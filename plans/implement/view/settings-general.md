# Implementation Plan: Settings General

## Feature ID
view/settings-general

## Spec Reference
`plans/spec/view/settings-general.md`

## Design Reference
`plans/design/view/settings-general.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::settings`

### Change Recipe
1. 설정 패널에서 일반 탭 렌더링
2. 테마, 언어, 기본 동작 설정
3. 변경 시 즉시 반영

### Find Strategies
- `grep -rn "settings" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`settings-general`

### Verification
- cargo check -p view
