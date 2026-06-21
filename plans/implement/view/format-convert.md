# Implementation Plan: Format Convert

## Feature ID
view/format-convert

## Spec Reference
`plans/spec/view/format-convert.md`

## Design Reference
`plans/design/view/format-convert.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/controls.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 포맷 변환 메뉴/버튼 렌더링
2. 대상 포맷 선택 (PNG, JPEG, BMP, WebP, TIFF 등)
3. 현재 이미지를 선택된 포맷으로 인코딩하여 저장

### Find Strategies
- `grep -rn "convert" apps/view/src-tauri/src/commands.rs`

### Debug ID
`format-convert`

### Verification
- cargo check -p view
