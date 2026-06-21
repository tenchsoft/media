# Test: equalizer-60hz-plus

## 검증 대상
spec(`plans/spec/player/equalizer-60hz-plus.md`)의 acceptance criteria → 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 60Hz 게인이 증가한다 | `equalizer_60hz_plus_increases_gain` |
| AC2: 오디오에 즉시 반영된다 | `equalizer_60hz_plus_calls_backend` |

## 테스트 파일 위치
`apps/player/src-tauri/tests/player_e2e.rs`

## Required Test Shape
- **Success**: EQ 패널 열기 → 60Hz + 버튼 클릭 → eq_bands[0] 증가 확인 → 함수: `equalizer_60hz_plus_increases_gain`
- **Negative**: 최대 게인(+12dB) 도달 시 클릭 → 값 변화 없음 → 함수: `equalizer_60hz_plus_clamped_at_max`
- **Edge**: 연속 클릭 → +12dB에서 정지 → 함수: `equalizer_60hz_plus_stops_at_boundary`

## 사용할 자동화 노드
implement(`plans/implement/player/equalizer-60hz-plus.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `player.info.equalizer` | EQ 패널 열기 전 | absent → present |
| `player.equalizer.band.0.plus` | EQ 패널 열림 후 | present, 클릭 가능 |

## 의존
- 선행 implement: `plans/implement/player/equalizer-60hz-plus.md`
- 픽스처: 불필요 (상태 기반 검증)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-player equalizer_60hz
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 — 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-player equalizer_60hz` 통과.
5. `cargo check --workspace --locked` 통과.
