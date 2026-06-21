# Test: annotation-redo

## 검증 대상
spec(`plans/spec/view/annotation-redo.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 취소한 주석 동작이 다시 적용된다 | `annotation_redo_restores_undone` |
| AC2: 다시 실행할 동작이 없으면 버튼이 비활성화된다 | `annotation_redo_disabled_when_empty` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 주석 추가 -> undo -> redo 클릭 -> 주석 복원 -> 함수: `annotation_redo_restores_undone`
- **Negative**: redo 스택 비어있을 때 비활성화 -> 함수: `annotation_redo_disabled_when_empty`
- **Edge**: undo 후 새 주석 추가 시 redo 스택 클리어 -> 함수: `annotation_redo_cleared_on_new_action`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-redo.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.redo` | undo 후 | present |
| `view.automatic.annotations_overlay` | redo 후 | count 복원 |

## 의존
- 선행 implement: `plans/implement/view/annotation-redo.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_redo
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_redo` 통과.
5. `cargo check --workspace --locked` 통과.
