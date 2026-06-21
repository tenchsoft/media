# Test: subtitle-style-text-color-minus

## 검증 대상
spec(`plans/spec/player/subtitle-style-text-color-minus.md`)의 acceptance criteria → 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 이전 색상으로 전환된다 | `subtitle_style_text_color_minus_cycles_backward` |
| AC2: 미리보기에 즉시 반영된다 | `subtitle_style_text_color_minus_updates_display` |

## 테스트 파일 위치
`apps/player/src-tauri/tests/player_e2e.rs`

## Required Test Shape
- **Success**: 자막 스타일 열기 → text_color - 클릭 → text_color 이전 색상으로 변경 확인 → 함수: `subtitle_style_text_color_minus_cycles_backward`
- **Negative**: 해당 없음
- **Edge**: 첫 번째 색상에서 - 클릭 → 마지막 색상으로 순환 → 함수: `subtitle_style_text_color_minus_wraps_around`

## 사용할 자동화 노드
implement(`plans/implement/player/subtitle-style-text-color-minus.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `player.subtitle_style.text_color.minus` | 자막 스타일 열림 후 | present, 클릭 가능 |

## 의존
- 선행 implement: `plans/implement/player/subtitle-style-text-color-minus.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-player subtitle_style_text_color
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 — 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-player subtitle_style_text_color` 통과.
5. `cargo check --workspace --locked` 통과.
