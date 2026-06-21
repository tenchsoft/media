# Implementation Plan: Automatic Gapless Next Track

## Feature ID
player/automatic-gapless-next-track

## Spec Reference
`plans/spec/player/automatic-gapless-next-track.md`

## Design Reference
`plans/design/player/automatic-gapless-next-track.md`
## Background Reference
`plans/background/player/automatic-gapless-next-track.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/gst_backend.rs`
- Search: `grep -n "about_to_finish\|gapless" apps/player/src-tauri/src/gst_backend.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::playlist`

### Change Recipe
1. gst_backend.rs에서 about-to-finish 시그널 핸들러
2. 다음 트랙 URI를 미리 큐에 등록
3. 재생 종료 시 자동으로 다음 트랙 시작
4. 크로스페이드 없이 즉시 전환

### Find Strategies
- `grep -rn "about_to_finish" apps/player/src-tauri/src/gst_backend.rs`
- `grep -rn "gapless" apps/player/src-tauri/src/gst_backend.rs`

### Debug ID
`automatic-gapless-next-track`

### Verification
- cargo check -p player
- 트랙 종료 시 다음 트랙 자동 재생
