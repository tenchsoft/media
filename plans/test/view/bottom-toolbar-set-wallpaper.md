# Test: bottom-toolbar-set-wallpaper

## 검증 대상
spec(`plans/spec/view/bottom-toolbar-set-wallpaper.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 이미지가 바탕화면으로 설정된다 | `bottom_toolbar_set_wallpaper_triggers` |
| AC2: 완료 알림이 표시된다 | `bottom_toolbar_set_wallpaper_shows_status` |
| AC3: 이미지 없을 때 버튼이 없다 | `bottom_toolbar_set_wallpaper_no_button_without_image` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 배경화면 버튼 클릭 -> 상태 메시지 표시 -> 함수: `bottom_toolbar_set_wallpaper_triggers`
- **Negative**: 이미지 없을 때 버튼 미노출 -> 함수: `bottom_toolbar_set_wallpaper_no_button_without_image`
- **Edge**: 설정 후 다른 이미지로 전환 -> 함수: `bottom_toolbar_set_wallpaper_after_nav`

## 사용할 자동화 노드
implement(`plans/implement/view/bottom-toolbar-set-wallpaper.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.bottom.set_wallpaper` | 이미지 로드 후 | present, enabled |
| `view.automatic.status_message_lifecycle` | 배경화면 설정 후 | 완료 메시지 |

## 의존
- 선행 implement: `plans/implement/view/bottom-toolbar-set-wallpaper.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view bottom_toolbar_set_wallpaper
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view bottom_toolbar_set_wallpaper` 통과.
5. `cargo check --workspace --locked` 통과.
