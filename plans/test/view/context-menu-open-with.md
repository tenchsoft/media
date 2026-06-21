# Test: context-menu-open-with

## 검증 대상
spec(`plans/spec/view/context-menu-open-with.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: "다른 프로그램으로 열기" 선택 시 시스템 대화상자가 나타난다 | `ctx_open_with_triggers_platform` |
| AC2: 취소 시 아무 일도 일어나지 않는다 | `ctx_open_with_cancel_no_op` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 컨텍스트 메뉴 열기 -> "다른 프로그램으로 열기" 클릭 -> 플랫폼 유틸 호출 확인 -> 함수: `ctx_open_with_triggers_platform`
- **Negative**: 이미지 없을 때 항목 없음 -> 함수: `ctx_open_with_no_item_without_image`
- **Edge**: 메뉴 닫기 후 재시도 -> 함수: `ctx_open_with_dismiss_and_retry`

## 사용할 자동화 노드
implement(`plans/implement/view/context-menu-open-with.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.ctx.open_with` | 컨텍스트 메뉴 표시 후 | present, enabled |

## 의존
- 선행 implement: `plans/implement/view/context-menu-open-with.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view ctx_open_with
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view ctx_open_with` 통과.
5. `cargo check --workspace --locked` 통과.
