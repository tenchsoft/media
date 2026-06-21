# Implementation Plan: Timeline Zoom Slider

## Feature ID
composer/timeline-zoom-slider

## Spec Reference
`plans/spec/composer/timeline-zoom-slider.md`

## Design Reference
`plans/design/composer/timeline-zoom-slider.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "zoom" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::timeline_zoom`

### Change Recipe
1. timeline_panel.rs에서 zoom 슬라이더 hit-test 영역 확인
2. 드래그 이벤트 → timeline_zoom 값 업데이트
3. 타임라인 픽셀/초 비율 변경
4. 리렌더

### Find Strategies
- `grep -rn "timeline_zoom" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "zoom" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`timeline-zoom-slider`

### Verification
- cargo check -p composer
- 슬라이더 조정 시 타임라인 확대/축소
