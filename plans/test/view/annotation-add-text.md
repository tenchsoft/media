# Test: annotation-add-text

## 검증 대상
spec(`plans/spec/view/annotation-add-text.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 위치에 텍스트가 표시된다 | `annotation_add_text_creates_text_annotation` |
| AC2: 선택한 색상이 적용된다 | `annotation_add_text_applies_color` |
| AC3: 빈 텍스트는 추가되지 않는다 | `annotation_add_text_empty_is_no_op` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 텍스트 도구 선택 -> 이미지 위 클릭 -> 텍스트 입력 -> 확인 -> 주석 추가 -> 함수: `annotation_add_text_creates_text_annotation`
- **Negative**: 빈 텍스트 확인 -> 주석 미추가 -> 함수: `annotation_add_text_empty_is_no_op`
- **Edge**: 텍스트 입력 후 취소 -> 함수: `annotation_add_text_cancel_no_op`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-add-text.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.text` | 퀵 에디트 열림 후 | present |
| `view.automatic.annotations_overlay` | 텍스트 확인 후 | count 증가 |

## 의존
- 선행 implement: `plans/implement/view/annotation-add-text.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_add_text
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_add_text` 통과.
5. `cargo check --workspace --locked` 통과.
