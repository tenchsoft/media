# View: Annotation Tool 상태 검증 누락

## 문제

`view.quick_edit.annotation.arrow` ~ `view.quick_edit.annotation.blur` (6개)를 클릭하지만 `state.active_annotation_tool` 변화를 검증하지 않는다.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:1725-1727` (`ClickAction::SelectAnnotationTool(tool)`)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:478-480`

## 메인 코드 수정

필요 없음. `state.active_annotation_tool`이 이미 public.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

```rust
// 기존 (라인 478-480):
for tool in ["rect", "circle", "text", "draw", "blur", "arrow"] {
    click(&mut harness, &format!("view.quick_edit.annotation.{tool}"));
}

// 수정:
for tool in ["rect", "circle", "text", "draw", "blur", "arrow"] {
    click(&mut harness, &format!("view.quick_edit.annotation.{tool}"));
    let s = state(&mut harness);
    assert_eq!(s.active_annotation_tool.as_ref().map(|t| t.as_str()), Some(tool));
}
```
