# Implementation Plan: Track Volume Slider

## Feature ID
composer/track-volume-slider

## Spec Reference
`plans/spec/composer/track-volume-slider.md`

## Design Reference
`plans/design/composer/track-volume-slider.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "volume" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::volume`

### Change Recipe
1. timeline_panel.rs에서 트랙 행의 볼륨 슬라이더 hit-test 영역 확인
2. 드래그 이벤트 → Track::volume 값 업데이트 (0.0~1.0)
3. 실시간 오디오 레벨 변경
4. 리렌더

### Find Strategies
- `grep -rn "volume" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "volume" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`track-volume-slider`

### Verification
- cargo check -p composer
- 슬라이더 조정 시 볼륨 변경
