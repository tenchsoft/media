# View: Convert Format 항목 상태 검증 누락

## 문제

`view.convert.format.png` ~ `view.convert.format.tiff` (5개)를 클릭하지만 `state.convert_output_format` 변화를 검증하지 않는다.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:1749-1750` (ClickAction::ConvertApply)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:577-579`

## 메인 코드 수정

필요 없음. `state.convert_output_format`이 이미 public.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

```rust
// 기존 (라인 577-579):
for fmt in ["png", "jpg", "webp", "bmp", "tiff"] {
    click(&mut harness, &format!("view.convert.format.{fmt}"));
}

// 수정:
for fmt in ["png", "jpg", "webp", "bmp", "tiff"] {
    click(&mut harness, &format!("view.convert.format.{fmt}"));
    assert_eq!(state(&mut harness).convert_output_format, fmt);
}
```
