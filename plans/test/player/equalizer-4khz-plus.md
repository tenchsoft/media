# Test: equalizer-4khz-plus

## 검증 대상
spec(`plans/spec/player/equalizer-4khz-plus.md`)의 acceptance criteria → 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 4kHz 게인이 증가한다 | `equalizer_4khz_plus_increases_gain` |

## 테스트 파일 위치
`apps/player/src-tauri/tests/player_e2e.rs`

## Required Test Shape
- **Success**: EQ 패널 열기 → 4kHz + 버튼 클릭 → eq_bands[3] 증가 확인 → 함수: `equalizer_4khz_plus_increases_gain`
- **Negative**: 최대 게인(+12dB) 도달 시 클릭 → 값 변화 없음 → 함수: `equalizer_4khz_plus_clamped_at_max`

## 사용할 자동화 노드
| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `player.equalizer.band.3.plus` | EQ 패널 열림 후 | present, 클릭 가능 |

## 의존
- 선행 implement: `plans/implement/player/equalizer-4khz-plus.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-player equalizer_4khz
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인.
3. 각 시나리오 함수 작성 — 행위 검증 패턴 사용.
4. `cargo test -p tench-player equalizer_4khz` 통과.
5. `cargo check --workspace --locked` 통과.
