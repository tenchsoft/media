# Test: annotation-line-width

## 검증 대상
spec(`plans/spec/view/annotation-line-width.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 선택 시 선 두께가 변경된다 | `annotation_line_width_changes_value` |
| AC2: 이후 주석에 적용된다 | `annotation_line_width_applies_to_new` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 퀵 에디트 열기 -> 선 두께 버튼 클릭 -> 값 변화 -> 함수: `annotation_line_width_changes_value`
- **Negative**: 도구 미선택 시 선 두께 변경 불가 -> 함수: `annotation_line_width_requires_tool`
- **Edge**: 선 두께 라운드트립 -> 함수: `annotation_line_width_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-line-width.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.line_width` | 퀵 에디트 + 도구 선택 후 | present, value 변경됨 |

## 의존
- 선행 implement: `plans/implement/view/annotation-line-width.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_line_width
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_line_width` 통과.
5. `cargo check --workspace --locked` 통과.
