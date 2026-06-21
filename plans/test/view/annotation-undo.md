# Test: annotation-undo

## 검증 대상
spec(`plans/spec/view/annotation-undo.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 마지막 주석 동작이 취소된다 | `annotation_undo_removes_last` |
| AC2: 이전 상태로 복원된다 | `annotation_undo_restores_previous` |
| AC3: 취소할 동작이 없으면 버튼이 비활성화된다 | `annotation_undo_disabled_when_empty` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 주석 추가 -> undo 클릭 -> 주석 수 감소 -> 함수: `annotation_undo_removes_last`
- **Negative**: undo 스택 비어있을 때 비활성화 -> 함수: `annotation_undo_disabled_when_empty`
- **Edge**: 여러 주석 추가 후 연속 undo -> 함수: `annotation_undo_multiple`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-undo.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.undo` | 퀵 에디트 + 도구 선택 후 | present |
| `view.automatic.annotations_overlay` | undo 후 | count 감소 |

## 의존
- 선행 implement: `plans/implement/view/annotation-undo.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_undo
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_undo` 통과.
5. `cargo check --workspace --locked` 통과.
