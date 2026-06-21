# Implementation Plan: Top Overlay Drop Zone

## Feature ID
view/top-overlay-drop-zone

## Spec Reference
`plans/spec/view/top-overlay-drop-zone.md`

## Design Reference
`plans/design/view/top-overlay-drop-zone.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/overlays.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::current_image`

### Change Recipe
1. 빈 상태 오버레이에서 드롭존 렌더링
2. 파일 드롭 시 이미지 로드
3. 드래그 중 시각적 피드백 (하이라이트)

### Find Strategies
- `grep -rn "drop" apps/view/src-tauri/src/ui/overlays.rs`

### Debug ID
`top-overlay-drop-zone`

### Verification
- cargo check -p view
