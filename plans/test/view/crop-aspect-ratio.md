# Test: crop-aspect-ratio

## 검증 대상
spec(`plans/spec/view/crop-aspect-ratio.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 비율 선택 시 자르기 영역이 해당 비율로 제한된다 | `crop_aspect_ratio_select_constrains` |
| AC2: "자유" 선택 시 비율 제한이 해제된다 | `crop_aspect_ratio_free_removes_constraint` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 크롭 모드 진입 -> 비율 선택 -> 비율 값 반영 -> 함수: `crop_aspect_ratio_select_constrains`
- **Negative**: 크롭 모드 아닐 때 비율 선택 불가 -> 함수: `crop_aspect_ratio_not_available_outside_crop`
- **Edge**: 비율 선택 라운드트립 -> 함수: `crop_aspect_ratio_round_trip`

## 사용할 자동화 노드
implement(`plans/implement/view/crop-aspect-ratio.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.crop.aspect.16_9` | 크롭 모드 + 비율 선택 후 | present |
| `view.crop.aspect.free` | 크롭 모드 + 자유 선택 후 | present |

## 의존
- 선행 implement: `plans/implement/view/crop-aspect-ratio.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view crop_aspect_ratio
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view crop_aspect_ratio` 통과.
5. `cargo check --workspace --locked` 통과.
