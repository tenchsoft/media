# Test: keyboard-shortcut-f2

## 검증 대상
spec(`plans/spec/view/keyboard-shortcut-f2.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: F2 키로 이름 바꾸기 대화상자가 나타난다 | `keyboard_f2_opens_rename_dialog` |
| AC2: 텍스트 입력 필드에 포커스가 있으면 동작하지 않는다 | `keyboard_f2_ignored_in_text_input` |

## 테스트 파일 위치
`apps/view/src-tauri/tests/view_e2e.rs`

## Required Test Shape
- **Success**: 이미지 로드 -> F2 키 입력 -> 이름 바꾸기 대화상자 출현 -> 함수: `keyboard_f2_opens_rename_dialog`
- **Negative**: 이미지 없을 때 F2 -> 부수효과 없음 -> 함수: `keyboard_f2_no_op_without_image`
- **Edge**: 이름 바꾸기 대화상자에서 F2 -> 중복 열기 방지 -> 함수: `keyboard_f2_no_double_open`

## 사용할 자동화 노드
implement(`plans/implement/view/keyboard-shortcut-f2.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `view.rename.confirm` | F2 후 대화상자 열림 | present |
| `view.rename.cancel` | F2 후 대화상자 열림 | present |

## 의존
- 선행 implement: `plans/implement/view/keyboard-shortcut-f2.md`
- 픽스처: 테스트 이미지 주입 (`inject_test_image`)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-view keyboard_f2
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 사용. selector 존재만 검증 금지.
4. `cargo test -p tench-view keyboard_f2` 통과.
5. `cargo check --workspace --locked` 통과.
