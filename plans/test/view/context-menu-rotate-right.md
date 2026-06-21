# Test: context-menu-rotate-right

## 검증 대상
spec(`plans/spec/view/context-menu-rotate-right.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: "오른쪽으로 회전" 선택 시 이미지가 시계 방향으로 90도 회전한다 | `ctx_rotate_right_changes_rotation` |
| AC2: 메뉴 외부 클릭 시 메뉴가 닫힌다 | `ctx_rotate_right_dismiss_on_outside_click` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 컨텍스트 메뉴 열기 -> "오른쪽으로 회전" 클릭 -> rotation 값 변화 -> 함수: `ctx_rotate_right_changes_rotation`
- **Negative**: 이미지 없을 때 컨텍스트 메뉴에 회전 항목 없음 -> 함수: `ctx_rotate_right_no_item_without_image`
- **Edge**: 왼쪽 회전 후 오른쪽 회전 -> 원래 상태 복원 -> 함수: `ctx_rotate_right_cancels_left`

## 사용할 자동화 노드
implement(`plans/implement/view/context-menu-rotate-right.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.ctx.rotate_right` | 컨텍스트 메뉴 표시 후 | present, enabled |

## 의존
- 선행 implement: `plans/implement/view/context-menu-rotate-right.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view ctx_rotate_right
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view ctx_rotate_right` 통과.
5. `cargo check --workspace --locked` 통과.
