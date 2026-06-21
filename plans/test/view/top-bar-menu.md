# Test: top-bar-menu

## 검증 대상
spec(`plans/spec/view/top-bar-menu.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 메뉴 클릭 시 드롭다운이 나타난다 | `top_bar_menu_toggle_opens_menu` |
| AC2: 항목 선택 시 해당 동작이 실행된다 | `top_bar_menu_item_triggers_action` |
| AC3: 메뉴 외부 클릭 시 메뉴가 닫힌다 | `top_bar_menu_dismiss_on_outside_click` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 메뉴 버튼 클릭 -> 드롭다운 출현 -> 함수: `top_bar_menu_toggle_opens_menu`
- **Negative**: 메뉴 외부 클릭 -> 메뉴 소멸 -> 함수: `top_bar_menu_dismiss_on_outside_click`
- **Edge**: 토글 라운드트립 -> 함수: `top_bar_menu_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/top-bar-menu.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.top.menu` | 기본 상태 | present |
| `view.menu.file` | 메뉴 열림 후 | present |
| `view.menu.edit` | 메뉴 열림 후 | present |
| `view.menu.view` | 메뉴 열림 후 | present |

## 의존
- 선행 implement: `plans/implement/view/top-bar-menu.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view top_bar_menu
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view top_bar_menu` 통과.
5. `cargo check --workspace --locked` 통과.
