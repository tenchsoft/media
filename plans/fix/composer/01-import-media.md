# Composer: Import Media (Native Dialog No-op)

## 문제

`composer.toolbar.import` 버튼 클릭 시 `ClickAction::ImportMedia` -> `self.open_file_dialog()` -> `app_handle`이 `None`이라 no-op.

## 관련 파일

- **소스**: `apps/composer/src-tauri/src/ui/mod.rs:236-238`
- **테스트**: `apps/composer/src-tauri/tests/plan_ui_e2e.rs:72` (present만 확인, 클릭하지 않음)

## 메인 코드 수정

`apps/composer/src-tauri/src/ui/mod.rs`:

```rust
// ComposerApp에 테스트용 미디어 주입 추가:
#[cfg(test)]
test_next_media: Option<(String, String, f64)>, // (path, title, duration)

pub fn inject_test_media(&mut self, path: String, title: String, duration: f64) {
    #[cfg(test)]
    {
        self.test_next_media = Some((path, title, duration));
    }
}

// ImportMedia 핸들러 수정:
ClickAction::ImportMedia => {
    #[cfg(test)]
    {
        if let Some((path, title, duration)) = self.test_next_media.take() {
            self.state.add_media_asset(path, title, duration);
            return;
        }
    }
    self.open_file_dialog();
}
```

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

```rust
{
    let pod = harness.root_mut();
    let app: &mut ComposerApp = pod.widget.downcast_mut().unwrap();
    app.inject_test_media("/test/video.mp4".to_string(), "Test Video".to_string(), 30.0);
}
let before_count = state(&mut harness).media_assets.len();
click(&mut harness, "composer.toolbar.import");
assert_eq!(state(&mut harness).media_assets.len(), before_count + 1);
```
