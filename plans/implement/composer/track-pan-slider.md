# Implementation Plan: Track Pan Slider

## Feature ID
composer/track-pan-slider

## Spec Reference
`plans/spec/composer/track-pan-slider.md`

## Design Reference
`plans/design/composer/track-pan-slider.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "pan" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::pan`

### Change Recipe
1. timeline_panel.rs에서 트랙 행의 팬 슬라이더 hit-test 영역 확인
2. 드래그 이벤트 → Track::pan 값 업데이트 (-1.0~1.0)
3. 실시간 오디오 패닝 변경
4. 리렌더

### Find Strategies
- `grep -rn "pan" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "pan" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`track-pan-slider`

### Verification
- cargo check -p composer
- 슬라이더 조정 시 패닝 변경
