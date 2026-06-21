# Test: bottom-toolbar-delete

## 검증 대상
spec(`plans/spec/view/bottom-toolbar-delete.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 확인 시 파일이 휴지통으로 이동한다 | `bottom_toolbar_delete_confirm_removes_file` |
| AC2: 다음 이미지가 표시된다 | `bottom_toolbar_delete_shows_next_image` |
| AC3: 취소 시 파일이 유지된다 | `bottom_toolbar_delete_cancel_keeps_file` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 삭제 버튼 클릭 -> 확인 대화상자 출현 -> 확인 클릭 -> 이미지 제거 -> 함수: `bottom_toolbar_delete_confirm_removes_file`
- **Negative**: 취소 시 파일 유지 -> 함수: `bottom_toolbar_delete_cancel_keeps_file`
- **Edge**: 이미지 없을 때 삭제 버튼 미노출 -> 함수: `bottom_toolbar_delete_no_button_without_image`

## 사용할 자동화 노드
implement(`plans/implement/view/bottom-toolbar-delete.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.bottom.delete` | 이미지 로드 후 | present, enabled |
| `view.delete.confirm` | 확인 대화상자 후 | present |
| `view.delete.cancel` | 확인 대화상자 후 | present |

## 의존
- 선행 implement: `plans/implement/view/bottom-toolbar-delete.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view bottom_toolbar_delete
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view bottom_toolbar_delete` 통과.
5. `cargo check --workspace --locked` 통과.
