# Implementation Plan: Settings File Association

## Feature ID
view/settings-file-association

## Spec Reference
`plans/spec/view/settings-file-association.md`

## Design Reference
`plans/design/view/settings-file-association.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::settings`
- Platform: `apps/view/src-tauri/src/platform_util.rs`

### Change Recipe
1. 설정 패널에서 파일 연결 탭 렌더링
2. 지원 포맷 체크박스 (PNG, JPEG, GIF, BMP, WebP, TIFF, SVG 등)
3. OS에 파일 연결 등록

### Find Strategies
- `grep -rn "file_association" apps/view/src-tauri/src/`

### Debug ID
`settings-file-association`

### Verification
- cargo check -p view
