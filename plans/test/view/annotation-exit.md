# Test: annotation-exit

## 검증 대상
spec(`plans/spec/view/annotation-exit.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 나가기 시 저장 확인이 나타난다 | `annotation_exit_shows_confirm_with_annotations` |
| AC2: 주석 모드가 종료된다 | `annotation_exit_closes_mode` |
| AC3: 주석이 없으면 확인 없이 종료된다 | `annotation_exit_no_confirm_without_annotations` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 주석 추가 -> 나가기 버튼 클릭 -> 확인 대화상자 출현 -> 확인 클릭 -> 모드 종료 -> 함수: `annotation_exit_shows_confirm_with_annotations`
- **Negative**: 주석 없이 나가기 -> 확인 없이 종료 -> 함수: `annotation_exit_no_confirm_without_annotations`
- **Edge**: 취소로 돌아가기 -> 모드 유지 -> 함수: `annotation_exit_cancel_keeps_mode`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-exit.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.exit` | 퀵 에디트 열림 후 | present |
| `view.annotation.exit_confirm` | 나가기 클릭 후 | present |
| `view.annotation.exit_cancel` | 나가기 클릭 후 | present |

## 의존
- 선행 implement: `plans/implement/view/annotation-exit.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_exit
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_exit` 통과.
5. `cargo check --workspace --locked` 통과.
