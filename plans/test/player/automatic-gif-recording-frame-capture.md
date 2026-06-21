# Test: automatic-gif-recording-frame-capture

## 검증 대상
spec(`plans/spec/player/automatic-gif-recording-frame-capture.md`)의 acceptance criteria → 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 녹화 중 프레임이 자동 캡처된다 | `gif_recording_frame_capture_collects_frames` |
| AC2: 녹화 완료 시 GIF가 생성된다 | `gif_recording_frame_capture_stops_on_stop` |

## 테스트 파일 위치
`apps/player/src-tauri/tests/player_e2e.rs`

## Required Test Shape
- **Success**: GIF 녹화 시작 → gif_recording = true 확인 → gif_frames에 프레임 수집 확인 → 함수: `gif_recording_frame_capture_collects_frames`
- **Negative**: 녹화 아닐 때 프레임 수집 안 됨 → 함수: `gif_recording_frame_capture_no_frames_when_not_recording`
- **Edge**: FPS 제한으로 프레임 스킵 확인 → 함수: `gif_recording_frame_capture_respects_fps_limit`

## 사용할 자동화 노드
implement(`plans/implement/player/automatic-gif-recording-frame-capture.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `player.gif_modal.start` | GIF 캡처 다이얼로그 열림 후 | present |
| `player.gif_modal.stop` | 녹화 중 | present |

## 의존
- 선행 implement: `plans/implement/player/automatic-gif-recording-frame-capture.md`
- 픽스처: 불필요 (상태 기반 검증)
- 다이얼로그 주입: 불필요

### CI Notes
- FPS 제한 테스트는 `gif_last_frame_ms` 필드를 직접 검증하므로 wall clock에 의존하지 않음

## Verification
```bash
cargo test -p tench-player gif_recording_frame
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 — 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-player gif_recording_frame` 통과.
5. `cargo check --workspace --locked` 통과.
