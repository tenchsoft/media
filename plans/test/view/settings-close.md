# Test: settings-close

## 검증 대상
spec(`plans/spec/view/settings-close.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 닫기 버튼으로 설정이 닫힌다 | `settings_close_click_dismisses_panel` |
| AC2: Esc로도 닫을 수 있다 | `settings_close_escape_dismisses_panel` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 설정 패널 열기 -> 닫기 버튼 클릭 -> 패널 소멸 -> 함수: `settings_close_click_dismisses_panel`
- **Negative**: 닫기 후 다른 오버레이가 영향받지 않음 -> 함수: `settings_close_no_side_effects`
- **Edge**: 열기->닫기->다시 열기 라운드트립 -> 함수: `settings_close_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/settings-close.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.settings.close` | 패널 표시 후 | present |
| `view.bottom.settings` | 패널 닫힌 후 | present (토글) |

## 의존
- 선행 implement: `plans/implement/view/settings-close.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view settings_close
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view settings_close` 통과.
5. `cargo check --workspace --locked` 통과.
