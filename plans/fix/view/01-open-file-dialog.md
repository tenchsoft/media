# View: Open File Dialog (Native Dialog No-op)

## 문제

`view.top.open` / `view.overlay.open_file` / `view.empty.open_image` 버튼 클릭 시 `ClickAction::OpenFileDialog` -> `self.open_file_dialog()` -> `app_handle`이 `None`이라 no-op.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:2457-2458` (dispatch)
- **소스**: `apps/view/src-tauri/src/ui/app.rs:362-387` (open_file_dialog)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:176` (`click("view.overlay.open_file")` 후 검증 없음)

## 메인 코드 수정

`apps/view/src-tauri/src/ui/app.rs`:

```rust
// ViewApp에 테스트용 파일 경로 주입 추가:
#[cfg(test)]
test_next_file: Option<String>,

pub fn inject_next_file(&mut self, path: String) {
    #[cfg(test)]
    {
        self.test_next_file = Some(path);
    }
}

// open_file_dialog 수정:
fn open_file_dialog(&self) {
    #[cfg(test)]
    {
        if let Some(path) = self.test_next_file.take() {
            if let Some(ref tx) = self.dialog_tx {
                let _ = tx.send(DialogResult::File(path));
            }
            return;
        }
    }
    // 기존 native dialog 코드...
}
```

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

`apps/view/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 176):
click(&mut harness, "view.overlay.open_file");

// 수정:
{
    let pod = harness.root_mut();
    let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
    app.inject_next_file("/test/sample.png".to_string());
}
click(&mut harness, "view.overlay.open_file");
let s = state(&mut harness);
assert!(s.document.is_some());
assert_eq!(s.document.as_ref().unwrap().path, "/test/sample.png");
```
