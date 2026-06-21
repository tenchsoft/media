# Test: bottom-toolbar-settings

## 검증 대상
spec(`plans/spec/view/bottom-toolbar-settings.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 설정 대화상자가 열린다 | `bottom_toolbar_settings_click_opens_panel` |
| AC2: 다시 클릭 시 설정 패널이 닫힌다 | `bottom_toolbar_settings_toggle_closes` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 설정 버튼 클릭 -> 설정 패널 출현 -> 함수: `bottom_toolbar_settings_click_opens_panel`
- **Negative**: 이미지 없이도 설정 버튼 동작 -> 함수: `bottom_toolbar_settings_works_without_image`
- **Edge**: 토글 라운드트립 -> 함수: `bottom_toolbar_settings_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/bottom-toolbar-settings.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.bottom.settings` | 기본 상태 | present |
| `view.settings.close` | 설정 열림 후 | present |

## 의존
- 선행 implement: `plans/implement/view/bottom-toolbar-settings.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view bottom_toolbar_settings
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view bottom_toolbar_settings` 통과.
5. `cargo check --workspace --locked` 통과.
