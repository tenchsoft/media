# Implementation Plan: Audio Waveform Display

## Feature ID
composer/audio-waveform-display

## Spec Reference
`plans/spec/composer/audio-waveform-display.md`

## Design Reference
`plans/design/composer/audio-waveform-display.md`
## Background Reference
`plans/background/composer/audio-waveform-display.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "waveform" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::waveform_data`

### Change Recipe
1. timeline.rs에서 오디오 클립 렌더링 시 waveform_data 읽기
2. 파형을 클립 영역 내에 막대 그래프로 렌더링
3. 줌 레벨에 따라 해상도 조정
4. 색상: 트랙 색상 기반

### Find Strategies
- `grep -rn "waveform" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "waveform" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`audio-waveform-display`

### Verification
- cargo check -p composer
- 오디오 클립에 파형 표시
