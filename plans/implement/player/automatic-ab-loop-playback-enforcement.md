# Implementation Plan: Automatic AB Loop Playback Enforcement

## Feature ID
player/automatic-ab-loop-playback-enforcement

## Spec Reference
`plans/spec/player/automatic-ab-loop-playback-enforcement.md`

## Design Reference
`plans/design/player/automatic-ab-loop-playback-enforcement.md`
## Background Reference
`plans/background/player/automatic-ab-loop-playback-enforcement.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/state.rs`
- Search: `grep -n "ab_loop\|loop_start\|loop_end" apps/player/src-tauri/src/ui/state.rs`
- Backend: `apps/player/src-tauri/src/gst_backend.rs`

### Change Recipe
1. state.rs에서 ab_loop_start, ab_loop_end 필드 확인
2. 재생 중 playhead가 loop_end에 도달하면 loop_start로 시크
3. gst_backend.rs에서 시크 명령 실행
4. 루프 해제 시 정상 재생 복원

### Find Strategies
- `grep -rn "ab_loop" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "seek" apps/player/src-tauri/src/gst_backend.rs`

### Debug ID
`automatic-ab-loop-playback-enforcement`

### Verification
- cargo check -p player
- A-B 루프 설정 시 반복 재생
