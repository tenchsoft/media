# Implementation Plan: Automatic Audio Only Visualizer

## Feature ID
player/automatic-audio-only-visualizer

## Spec Reference
`plans/spec/player/automatic-audio-only-visualizer.md`

## Design Reference
`plans/design/player/automatic-audio-only-visualizer.md`
## Background Reference
`plans/background/player/automatic-audio-only-visualizer.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_video.rs`
- Search: `grep -n "visualizer\|audio_only" apps/player/src-tauri/src/ui/paint_video.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::is_audio_only`

### Change Recipe
1. paint_video.rs에서 is_audio_only 확인
2. 오디오 전용 파일 시 비주얼라이저 렌더링
3. 주파수 스펙트럼 또는 파형 애니메이션
4. gst_backend.rs에서 오디오 데이터 수집

### Find Strategies
- `grep -rn "is_audio_only" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "visualizer" apps/player/src-tauri/src/ui/paint_video.rs`

### Debug ID
`automatic-audio-only-visualizer`

### Verification
- cargo check -p player
- 오디오 파일 재생 시 비주얼라이저 표시
