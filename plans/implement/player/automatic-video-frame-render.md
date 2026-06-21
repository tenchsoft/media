# Implementation Plan: Automatic Video Frame Render

## Feature ID
player/automatic-video-frame-render

## Spec Reference
`plans/spec/player/automatic-video-frame-render.md`

## Design Reference
`plans/design/player/automatic-video-frame-render.md`
## Background Reference
`plans/background/player/automatic-video-frame-render.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_video.rs`
- Search: `grep -n "frame\|render\|surface" apps/player/src-tauri/src/ui/paint_video.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::video_texture`

### Change Recipe
1. gst_backend.rs에서 비디오 프레임을 텍스처에 렌더링
2. paint_video.rs에서 텍스처를 캔버스에 그리기
3. 비디오 비율에 맞게 레터박스/핏 계산
4. 프레임 갱신 시 리페인트 요청

### Find Strategies
- `grep -rn "video_texture" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "render_frame" apps/player/src-tauri/src/ui/paint_video.rs`

### Debug ID
`automatic-video-frame-render`

### Verification
- cargo check -p player
- 비디오 프레임이 캔버스에 렌더링
