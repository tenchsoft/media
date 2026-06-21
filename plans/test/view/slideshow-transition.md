# Test: slideshow-transition

## 검증 대상
spec(`plans/spec/view/slideshow-transition.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 전환 효과 선택 시 슬라이드쇼에 반영된다 | `slideshow_transition_cycle_changes_value` |
| AC2: "없음" 선택 시 즉시 전환된다 | `slideshow_transition_none_is_instant` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 슬라이드쇼 재생 중 -> 전환 효과 버튼 클릭 -> 전환 효과 값 변화 -> 함수: `slideshow_transition_cycle_changes_value`
- **Negative**: 슬라이드쇼 정지 중에도 전환 설정 가능 -> 함수: `slideshow_transition_available_when_stopped`
- **Edge**: 전환 효과 순환 라운드트립 -> 함수: `slideshow_transition_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/slideshow-transition.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.slideshow.transition` | 슬라이드쇼 재생 후 | present, value 변경됨 |
| `view.automatic.slideshow_transition` | 전환 효과 변경 후 | 변경된 효과 이름 |

## 의존
- 선행 implement: `plans/implement/view/slideshow-transition.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view slideshow_transition
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view slideshow_transition` 통과.
5. `cargo check --workspace --locked` 통과.
