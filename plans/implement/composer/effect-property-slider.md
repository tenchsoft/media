# Implementation Plan: Effect Property Slider

## Feature ID
composer/effect-property-slider

## Spec Reference
`plans/spec/composer/effect-property-slider.md`

## Design Reference
`plans/design/composer/effect-property-slider.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "effect_property\|slider" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Effect::properties`

### Change Recipe
1. inspector.rs에서 이펙트 속성 슬라이더 hit-test
2. 드래그 → 속성 값 업데이트
3. 실시간 프리뷰 반영
4. 리렌더

### Find Strategies
- `grep -rn "properties" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "slider" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`effect-property-slider`

### Verification
- cargo check -p composer
- 슬라이더 조정 시 이펙트 속성 변경
