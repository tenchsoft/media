# Test: context-menu-properties

## 검증 대상
spec(`plans/spec/view/context-menu-properties.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: "속성" 선택 시 파일 속성 대화상자가 나타난다 | `ctx_properties_opens_file_info` |
| AC2: 파일 정보가 표시된다 | `ctx_properties_shows_file_details` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> 컨텍스트 메뉴 열기 -> "속성" 클릭 -> 파일 정보 패널 출현 -> 함수: `ctx_properties_opens_file_info`
- **Negative**: 이미지 없을 때 항목 없음 -> 함수: `ctx_properties_no_item_without_image`
- **Edge**: 속성 패널 닫기 후 다시 열기 -> 함수: `ctx_properties_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/context-menu-properties.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.ctx.properties` | 컨텍스트 메뉴 표시 후 | present, enabled |
| `view.file_info.close` | 속성 패널 열림 후 | present |

## 의존
- 선행 implement: `plans/implement/view/context-menu-properties.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view ctx_properties
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view ctx_properties` 통과.
5. `cargo check --workspace --locked` 통과.
