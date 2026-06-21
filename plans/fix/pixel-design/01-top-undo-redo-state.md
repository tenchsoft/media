# Pixel Design: Top Bar Undo/Redo 상태 검증 누락

## 문제

`pd.top.undo` / `pd.top.redo` 버튼을 클릭하지만 `state.history_index` 변화를 검증하지 않는다. `pd.history.undo` / `pd.history.redo`에서는 검증하고 있으나, top bar 버튼도 같은 동작을 해야 한다.

## 관련 파일

- **소스**: `apps/pixel-design/src-tauri/src/ui/mod.rs:793-806` (Ctrl+Z/Ctrl+Y 키보드 핸들러)
- **소스**: `apps/pixel-design/src-tauri/src/ui/mod.rs`의 `handle_top_bar_click` 메서드 (undo/redo 버튼)
- **테스트**: `apps/pixel-design/src-tauri/tests/pixel_design_e2e.rs:356-357`

## 메인 코드 수정

필요 없음. top bar undo/redo 버튼이 `state.undo()` / `state.redo()`를 호출하면 됨. 현재 구현이 이미 그렇게 되어 있는지 확인 필요.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

```rust
// 기존 (라인 356-357):
click(&mut harness, "pd.top.undo");
click(&mut harness, "pd.top.redo");

// 수정:
let before_idx = state(&mut harness).history_index;
click(&mut harness, "pd.top.undo");
assert!(state(&mut harness).history_index < before_idx);
click(&mut harness, "pd.top.redo");
assert_eq!(state(&mut harness).history_index, before_idx);
```
