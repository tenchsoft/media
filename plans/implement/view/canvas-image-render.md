# Implementation Plan: Canvas Image Render

## Feature ID
view/canvas-image-render

## Spec Reference
`plans/spec/view/canvas-image-render.md`

## Design Reference
`plans/design/view/canvas-image-render.md`

## Background Reference
`plans/background/view/canvas-image-render.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/image_stage.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`, `ViewState::zoom_level`, `ViewState::pan_offset`, `ViewState::rotation`, `ViewState::flip_h`, `ViewState::flip_v`

### Change Recipe
1. 이미지 로드 시 캔버스에 렌더링
2. 줌, 팬, 회전, 반전 변환 적용
3. 체크무늬 배경 (투명 이미지)
4. 창 크기 변경 시 자동 리렌더

### Find Strategies
- `grep -rn "image_stage" apps/view/src-tauri/src/ui/image_stage.rs`

### Debug ID
`canvas-image-render`

### Verification
- cargo check -p view
