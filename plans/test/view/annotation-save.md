# Test: annotation-save

## 검증 대상
spec(`plans/spec/view/annotation-save.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 주석이 이미지에 합성된다 | `annotation_save_clears_annotations` |
| AC2: 파일로 저장된다 | `annotation_save_triggers_persist` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 주석 추가 -> 저장 버튼 클릭 -> 주석 목록 클리어 -> 함수: `annotation_save_clears_annotations`
- **Negative**: 주석 없을 때 저장 -> 부수효과 없음 -> 함수: `annotation_save_no_op_without_annotations`
- **Edge**: 저장 후 undo 불가 -> 함수: `annotation_save_clears_undo_stack`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-save.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.save` | 퀵 에디트 + 주석 추가 후 | present, enabled |
| `view.automatic.annotations_overlay` | 저장 후 | `"0"` |

## 의존
- 선행 implement: `plans/implement/view/annotation-save.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_save
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_save` 통과.
5. `cargo check --workspace --locked` 통과.
