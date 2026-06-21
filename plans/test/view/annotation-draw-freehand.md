# Test: annotation-draw-freehand

## 검증 대상
spec(`plans/spec/view/annotation-draw-freehand.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 드래그 시 선이 그려진다 | `annotation_draw_freehand_creates_path` |
| AC2: 선택한 색상과 두께가 적용된다 | `annotation_draw_freehand_applies_style` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 자유 그리기 도구 선택 -> 이미지 위에서 드래그 -> 주석 추가 -> 함수: `annotation_draw_freehand_creates_path`
- **Negative**: 이미지 영역 밖에서 드래그 -> 주석 미추가 -> 함수: `annotation_draw_freehand_outside_no_op`
- **Edge**: 짧은 드래그 -> 최소 경로 주석 -> 함수: `annotation_draw_freehand_short_stroke`

## 사용할 자동화 노드
implement(`plans/implement/view/annotation-draw-freehand.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.quick_edit.annotation.freeform` | 퀵 에디트 열림 후 | present |
| `view.automatic.annotations_overlay` | 드래그 후 | count 증가 |

## 의존
- 선행 implement: `plans/implement/view/annotation-draw-freehand.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view annotation_draw_freehand
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view annotation_draw_freehand` 통과.
5. `cargo check --workspace --locked` 통과.
