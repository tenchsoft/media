# Implementation Plan: Settings Image

## Feature ID
view/settings-image

## Spec Reference
`plans/spec/view/settings-image.md`

## Design Reference
`plans/design/view/settings-image.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::settings`

### Change Recipe
1. 설정 패널에서 이미지 탭 렌더링
2. 기본 줌 모드, 보간 방식, EXIF 자동 회전 설정
3. 변경 시 즉시 반영

### Find Strategies
- `grep -rn "settings" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`settings-image`

### Verification
- cargo check -p view
