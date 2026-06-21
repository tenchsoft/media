# Test: annotation-erase

## 검증 대상
spec(`plans/spec/view/annotation-erase.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 해당 주석이 제거된다 | `annotation_erase_removes_annotation` |
| AC2: 지우개 도구 선택 시 활성 도구 변경 | `annotation_erase_selects_tool` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 주석 추가 -> 지우개 도구 선택 -> 주석 영역 클릭 -> 주석 제거 -> 함수: `annotation_erase_removes_annotation`
- **Negative**: 주석 없을 때 지우개 클릭 -> 부수효과 없음 -> 함수: `annotation_erase_no_op_without_annotations`
- **Edge**: 지우개 후 undo로 복원 -> 함수: `annotation_erase_undo_restores`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-erase.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.eraser` | 퀵 에디트 열림 후 | present |
| `view.automatic.annotations_overlay` | 주석 제거 후 | count 감소 |

## 의존
- 선행 implement: `plans/implement/view/annotation-erase.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_erase
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_erase` 통과.
5. `cargo check --workspace --locked` 통과.
