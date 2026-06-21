# Implementation Plan: Color Correction Panel

## Feature ID
composer/color-correction-panel

## Spec Reference
`plans/spec/composer/color-correction-panel.md`

## Design Reference
`plans/design/composer/color-correction-panel.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/right_panel.rs`
- Search: `grep -n "color_correction" apps/composer/src-tauri/src/ui/right_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::color_correction`

### Change Recipe
1. right_panel.rs에서 색 보정 탭/패널 렌더링
2. 슬라이더: brightness, contrast, saturation, hue, temperature
3. 값 변경 → Clip::color_correction 업데이트
4. 프리뷰 실시간 반영

### Find Strategies
- `grep -rn "color_correction" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "color" apps/composer/src-tauri/src/ui/right_panel.rs`

### Debug ID
`color-correction-panel`

### Verification
- cargo check -p composer
- 슬라이더 조정 시 색 보정 적용
