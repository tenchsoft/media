# View: Open Archive Dialog (Native Dialog No-op)

## 문제

`view.top.archive` / `view.empty.open_archive` 버튼 클릭 시 `ClickAction::OpenArchiveDialog` -> `self.open_archive_dialog()` -> `app_handle`이 `None`이라 no-op.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:2463-2464` (dispatch)
- **소스**: `apps/view/src-tauri/src/ui/app.rs:411-430` (open_archive_dialog)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:178` (`click("view.empty.open_archive")` 후 검증 없음)

## 메인 코드 수정

`01-open-file-dialog.md`와 동일한 패턴. `ViewApp`에 `inject_next_archive()` 추가. archive도 `DialogResult::File`로 처리되므로 `test_next_file`을 재사용 가능.

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

```rust
{
    let pod = harness.root_mut();
    let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
    app.inject_next_file("/test/archive.zip".to_string());
}
click(&mut harness, "view.empty.open_archive");
let s = state(&mut harness);
assert!(s.document.is_some());
```
