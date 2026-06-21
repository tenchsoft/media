# Implementation Plan: Audio Fade Handle

## Feature ID
composer/audio-fade-handle

## Spec Reference
`plans/spec/composer/audio-fade-handle.md`

## Design Reference
`plans/design/composer/audio-fade-handle.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "fade" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::fade_in`, `Clip::fade_out`

### Change Recipe
1. timeline.rs에서 오디오 클립 좌/우 상단 핸들 hit-test
2. 드래그 → fade_in 또는 fade_out 값 변경
3. 페이드 구간 시각적 오버레이 표시
4. 리렌더

### Find Strategies
- `grep -rn "fade_in\|fade_out" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "fade" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`audio-fade-handle`

### Verification
- cargo check -p composer
- 핸들 드래그 시 페이드 인/아웃 적용
