# Test: bottom-toolbar-share

## 검증 대상
spec(`plans/spec/view/bottom-toolbar-share.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 클릭 시 시스템 공유 시트가 표시된다 | `bottom_toolbar_share_triggers_platform` |
| AC2: 이미지 없을 때 버튼이 없다 | `bottom_toolbar_share_no_button_without_image` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 공유 버튼 클릭 -> 플랫폼 공유 호출 확인 -> 함수: `bottom_toolbar_share_triggers_platform`
- **Negative**: 이미지 없을 때 공유 버튼 미노출 -> 함수: `bottom_toolbar_share_no_button_without_image`
- **Edge**: 공유 후 상태 유지 -> 함수: `bottom_toolbar_share_preserves_state`

## 사용할 자동화 노드
implement(`plans/implement/view/bottom-toolbar-share.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.bottom.share` | 이미지 로드 후 | present, enabled |

## 의존
- 선행 implement: `plans/implement/view/bottom-toolbar-share.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view bottom_toolbar_share
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view bottom_toolbar_share` 통과.
5. `cargo check --workspace --locked` 통과.
