# Implementation Plan: Settings Slideshow

## Feature ID
view/settings-slideshow

## Spec Reference
`plans/spec/view/settings-slideshow.md`

## Design Reference
`plans/design/view/settings-slideshow.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::settings`

### Change Recipe
1. 설정 패널에서 슬라이드쇼 탭 렌더링
2. 기본 속도, 전환 효과, 루프 설정
3. 변경 시 즉시 반영

### Find Strategies
- `grep -rn "settings" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`settings-slideshow`

### Verification
- cargo check -p view
