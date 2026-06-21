# View: Batch/Convert Browse Output (Native Dialog No-op)

## 문제

`view.batch.browse_output` / `view.convert.browse_output` 버튼 클릭 시 native save-as dialog가 열려야 하지만 `app_handle`이 `None`이라 no-op.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:432-` (open_save_as_dialog)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:443` (batch), `580` (convert)

## 메인 코드 수정

`ViewApp`에 `inject_save_as_path()` 추가:

```rust
#[cfg(test)]
test_save_as_path: Option<String>,

pub fn inject_save_as_path(&mut self, path: String) {
    #[cfg(test)]
    {
        self.test_save_as_path = Some(path);
    }
}
```

`open_save_as_dialog`에서 `test_save_as_path`가 있으면 해당 경로로 바로 저장.

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

```rust
// batch:
{
    let pod = harness.root_mut();
    let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
    app.inject_save_as_path("/test/output/".to_string());
}
click(&mut harness, "view.batch.browse_output");
let s = state(&mut harness);
assert!(s.batch_output_dir.is_some());

// convert:
{
    let pod = harness.root_mut();
    let app: &mut ViewApp = pod.widget.downcast_mut().unwrap();
    app.inject_save_as_path("/test/converted/".to_string());
}
click(&mut harness, "view.convert.browse_output");
```
