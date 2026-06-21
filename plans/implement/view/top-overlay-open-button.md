# Implementation Plan: Top Overlay Open Button

## Feature ID
view/top-overlay-open-button

## Spec Reference
`plans/spec/view/top-overlay-open-button.md`

## Design Reference
`plans/design/view/top-overlay-open-button.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/overlays.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 빈 상태 오버레이에서 열기 버튼 렌더링
2. 클릭 시 파일 열기 대화상자 표시
3. 이미지 로드 후 오버레이 숨김

### Find Strategies
- `grep -rn "open" apps/view/src-tauri/src/ui/overlays.rs`

### Debug ID
`top-overlay-open-button`

### Verification
- cargo check -p view
