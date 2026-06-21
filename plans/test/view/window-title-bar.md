# Test: window-title-bar

## 검증 대상
spec(`plans/spec/view/window-title-bar.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 파일 이름이 타이틀 바에 표시된다 | `window_title_bar_shows_filename` |
| AC2: 변경 사항 표시가 나타난다 | `window_title_bar_shows_dirty_marker` |
| AC3: 파일이 없으면 "View"만 표시된다 | `window_title_bar_default_without_file` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 타이틀 바에 파일명 표시 -> 함수: `window_title_bar_shows_filename`
- **Negative**: 이미지 없을 때 기본 타이틀 -> 함수: `window_title_bar_default_without_file`
- **Edge**: 편집 후 수정 표시자 (*) 표시 -> 함수: `window_title_bar_shows_dirty_marker`

## 사용할 자동화 노드
implement(`plans/implement/view/window-title-bar.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.automatic.window_title` | 이미지 로드 후 | 파일명 포함 |
| `view.automatic.window_title` | 편집 후 | `*` 포함 |

## 의존
- 선행 implement: `plans/implement/view/window-title-bar.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

### CI Notes
- 타이틀 바 텍스트는 자동화 상태 노드로 검증. 실제 OS 윈도우 타이틀 확인은 CI에서 불가하므로 자동화 트리 노드 값으로 대체.

## Verification
```bash
cargo test -p tench-view window_title_bar
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view window_title_bar` 통과.
5. `cargo check --workspace --locked` 통과.
