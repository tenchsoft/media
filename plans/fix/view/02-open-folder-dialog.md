# View: Open Folder Dialog (Native Dialog No-op)

## 문제

`view.top.folder` / `view.overlay.open_folder` / `view.empty.open_folder` 버튼 클릭 시 `ClickAction::OpenFolderDialog` -> `self.open_folder_dialog()` -> `app_handle`이 `None`이라 no-op.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:2460-2461` (dispatch)
- **소스**: `apps/view/src-tauri/src/ui/app.rs:390-407` (open_folder_dialog)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:177` (`click("view.overlay.open_folder")` 후 검증 없음)

## 메인 코드 수정

`01-open-file-dialog.md`와 동일한 패턴. `ViewApp`에 `inject_next_folder()` 추가:

```rust
#[cfg(test)]
test_next_folder: Option<String>,

pub fn inject_next_folder(&mut self, path: String) {
    #[cfg(test)]
    {
        self.test_next_folder = Some(path);
    }
}
```

`open_folder_dialog`에서 `test_next_folder`가 있으면 `DialogResult::Folder` 전송.

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

```rust
{
    let pod = harness.root_mut();
    let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
    app.inject_next_folder("/test/images/".to_string());
}
click(&mut harness, "view.overlay.open_folder");
let s = state(&mut harness);
assert!(s.folder_images.len() > 0); // 폴더 내 이미지 로드 검증
```
