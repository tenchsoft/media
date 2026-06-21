# View: Batch Format 항목 상태 검증 누락

## 문제

`view.batch.format.png`, `view.batch.format.jpg`, `view.batch.format.webp`를 클릭하지만 `state.batch_output_format` 변화를 검증하지 않는다.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:1655` (`ClickAction::BatchSelectFormat(fmt)`)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:437-438`

## 메인 코드 수정

필요 없음. `state.batch_output_format`이 이미 public.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

`apps/view/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 437-438):
click(&mut harness, "view.batch.format.jpg");
click(&mut harness, "view.batch.format.webp");

// 수정:
click(&mut harness, "view.batch.format.jpg");
assert_eq!(state(&mut harness).batch_output_format, "jpg");
click(&mut harness, "view.batch.format.webp");
assert_eq!(state(&mut harness).batch_output_format, "webp");
```
