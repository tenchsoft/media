# Test: automatic-context-menu-hover-highlight

## 검증 대상
spec(`plans/spec/player/automatic-context-menu-hover-highlight.md`)의 acceptance criteria → 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 마우스 호버 시 항목이 강조된다 | `context_menu_hover_highlight_updates_on_move` |
| AC2: 마우스 이동 시 이전 항목 강조가 해제된다 | `context_menu_hover_highlight_clears_previous` |

## 테스트 파일 위치
`apps/player/src-tauri/tests/player_e2e.rs`

## Required Test Shape
- **Success**: 컨텍스트 메뉴 열기 → 메뉴 항목 위로 마우스 이동 → context_menu_hover = Some(idx) 확인 → 함수: `context_menu_hover_highlight_updates_on_move`
- **Negative**: 메뉴 영역 밖으로 마우스 이동 → context_menu_hover = None → 함수: `context_menu_hover_highlight_clears_outside_menu`
- **Edge**: 마지막 항목 아래로 이동 → hover = None → 함수: `context_menu_hover_highlight_boundary_below_items`

## 사용할 자동화 노드
implement(`plans/implement/player/automatic-context-menu-hover-highlight.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| 컨텍스트 메뉴 아이템 | 메뉴 열림 후 | hover 인덱스에 따라 하이라이트 배경색 적용 |

## 의존
- 선행 implement: `plans/implement/player/automatic-context-menu-hover-highlight.md`
- 픽스처: 불필요 (상태 기반 검증)
- 다이얼로그 주입: 불필요

### CI Notes
- PointerEvent::Move 시뮬레이션은 `harness.dispatch_pointer`로 처리
- 컨텍스트 메뉴는 state.context_menu를 직접 설정하여 열 수 있음

## Verification
```bash
cargo test -p tench-player context_menu_hover
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 — 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-player context_menu_hover` 통과.
5. `cargo check --workspace --locked` 통과.
