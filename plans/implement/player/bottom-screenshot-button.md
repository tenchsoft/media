# Implementation Plan: Bottom Screenshot Button

## Feature ID
player/bottom-screenshot-button

## Spec Reference
`plans/spec/player/bottom-screenshot-button.md`

## Design Reference
`plans/design/player/bottom-screenshot-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_controls.rs`
- Search: `grep -n "screenshot\|capture" apps/player/src-tauri/src/ui/paint_controls.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::video_texture`

### Change Recipe
1. paint_controls.rs에서 스크린샷 버튼 렌더링
2. 클릭 시 현재 프레임 캡처
3. gst_backend.rs에서 현재 프레임을 이미지로 저장
4. 파일 대화상자 없이 기본 경로에 저장 + 토스트 알림

### Find Strategies
- `grep -rn "screenshot" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "screenshot" apps/player/src-tauri/src/ui/paint_controls.rs`

### Debug ID
`bottom-screenshot-button`

### Verification
- cargo check -p player
- 스크린샷 저장 동작
