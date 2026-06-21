# Test: settings-file-association

## 검증 대상
spec(`plans/spec/view/settings-file-association.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 파일 연결 설정이 표시된다 | `settings_file_assoc_tab_shows_formats` |
| AC2: 토글로 연결을 켜거나 끌 수 있다 | `settings_file_assoc_toggle_changes_state` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 설정 패널 열기 -> 파일 연결 탭 선택 -> 포맷 체크박스 표시 -> 함수: `settings_file_assoc_tab_shows_formats`
- **Negative**: 취소 후 이전 값 유지 -> 함수: `settings_file_assoc_cancel_keeps_values`
- **Edge**: 탭 전환 라운드트립 -> 함수: `settings_file_assoc_tab_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/settings-file-association.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.settings.tab.file_association` | 탭 전환 후 | selected |
| `view.settings.close` | 패널 표시 후 | present |

## 의존
- 선행 implement: `plans/implement/view/settings-file-association.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view settings_file_assoc
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view settings_file_assoc` 통과.
5. `cargo check --workspace --locked` 통과.
