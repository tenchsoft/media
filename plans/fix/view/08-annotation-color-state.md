# View: Annotation Color 상태 검증 누락

## 문제

`view.annotation.color.r255.g128.b0`를 클릭하지만 `state.annotation_color` 변화를 검증하지 않는다.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:1698-1706` (`ClickAction::SetAnnotationColor`)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:504`

## 메인 코드 수정

필요 없음.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

```rust
// 기존 (라인 504):
click(&mut harness, "view.annotation.color.r255.g128.b0");

// 수정:
click(&mut harness, "view.annotation.color.r255.g128.b0");
let s = state(&mut harness);
assert_eq!(s.annotation_color, Color::from_rgb(255, 128, 0));
```
