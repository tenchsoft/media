# Test: file-print

## 검증 대상
spec(`plans/spec/view/file-print.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 인쇄 대화상자가 나타난다 | `file_print_dialog_opens` |
| AC2: 취소 시 인쇄되지 않는다 | `file_print_cancel_no_op` |
| AC3: 인쇄 버튼 클릭 시 플랫폼 인쇄 호출 | `file_print_click_calls_platform` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 인쇄 버튼 클릭 -> 인쇄 대화상자 출현 -> 인쇄 버튼 클릭 -> 플랫폼 호출 -> 함수: `file_print_click_calls_platform`
- **Negative**: 취소 시 상태 변화 없음 -> 함수: `file_print_cancel_no_op`
- **Edge**: 인쇄 대화상자에서 용지 크기/방향 변경 -> 함수: `file_print_change_paper_settings`

## 사용할 자동화 노드
implement(`plans/implement/view/file-print.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.print.print` | 인쇄 대화상자 열림 후 | present, enabled |
| `view.print.cancel` | 인쇄 대화상자 열림 후 | present, enabled |
| `view.print.paper.a4` | 용지 선택 후 | present |
| `view.print.orientation.portrait` | 방향 선택 후 | present |

## 의존
- 선행 implement: `plans/implement/view/file-print.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view file_print
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view file_print` 통과.
5. `cargo check --workspace --locked` 통과.
