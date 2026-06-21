# Test: canvas-checkerboard-bg

## 검증 대상
spec(`plans/spec/view/canvas-checkerboard-bg.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 투명 이미지에서 체크무늬 배경이 표시된다 | `canvas_checkerboard_toggle_changes_state` |
| AC2: 투명/불투명 영역이 구분된다 | `canvas_checkerboard_visible_with_transparent` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 체크무늬 토글 클릭 -> 상태 변화 -> 함수: `canvas_checkerboard_toggle_changes_state`
- **Negative**: 토글 off 시 체크무늬 미표시 -> 함수: `canvas_checkerboard_off_hides_pattern`
- **Edge**: 토글 라운드트립 -> 함수: `canvas_checkerboard_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/canvas-checkerboard-bg.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.bottom.checkerboard` | 이미지 로드 후 | present, value `"off"` -> `"on"` |

## 의존
- 선행 implement: `plans/implement/view/canvas-checkerboard-bg.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view canvas_checkerboard
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view canvas_checkerboard` 통과.
5. `cargo check --workspace --locked` 통과.
