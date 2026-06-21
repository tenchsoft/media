# Implementation Plan: Bottom Volume Slider

## Feature ID
player/bottom-volume-slider

## Spec Reference
`plans/spec/player/bottom-volume-slider.md`

## Design Reference
`plans/design/player/bottom-volume-slider.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "volume\|volume_slider" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::volume`

### Change Recipe
1. paint_controls.rs에서 볼륨 슬라이더 렌더링
2. 드래그 시 state.volume 갱신 (0.0 ~ 1.0)
3. gst_backend.rs에서 볼륨 설정 적용
4. 0일 때 음소거 아이콘 표시

### Find Strategies
- `grep -rn "volume" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "volume" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-volume-slider`

### Verification
- cargo check -p player
- 볼륨 슬라이더 조절 동작
