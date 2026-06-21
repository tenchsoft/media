# Test: slideshow-loop

## 검증 대상
spec(`plans/spec/view/slideshow-loop.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 반복 켜짐 시 마지막 이미지 후 처음으로 돌아간다 | `slideshow_loop_toggle_changes_state` |
| AC2: 반복 꺼짐 시 마지막 이미지에서 종료된다 | `slideshow_loop_off_stops_at_end` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 슬라이드쇼 재생 -> 루프 토글 클릭 -> 상태 변화 -> 함수: `slideshow_loop_toggle_changes_state`
- **Negative**: 루프 off 상태에서 마지막 이미지 도달 시 정지 -> 함수: `slideshow_loop_off_stops_at_end`
- **Edge**: 토글 라운드트립 -> 함수: `slideshow_loop_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/slideshow-loop.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.slideshow.loop` | 슬라이드쇼 재생 후 | present, value `"off"` -> `"on"` |

## 의존
- 선행 implement: `plans/implement/view/slideshow-loop.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view slideshow_loop
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view slideshow_loop` 통과.
5. `cargo check --workspace --locked` 통과.
