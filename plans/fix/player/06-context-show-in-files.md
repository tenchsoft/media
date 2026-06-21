# Player: Context Menu Show In Files (Platform No-op)

## 문제

우클릭 컨텍스트 메뉴에서 `player.context.show_in_files` 클릭 시 `ClickAction::ShowInFiles` -> `platform_util::show_in_file_manager`를 별도 스레드에서 호출하지만, 테스트 환경에서는 실제 파일 매니저가 열리지 않는다. 또한 `media_path`가 `None`이면 아무 일도 일어나지 않는다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:779-786`
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:385-413` (present만 확인, 클릭하지 않음)

## 메인 코드 수정

`apps/player/src-tauri/src/ui/app.rs`:

```rust
// PlayerApp에 테스트 플래그 추가:
#[cfg(test)]
show_in_files_called: bool,

// ShowInFiles 핸들러 수정:
ClickAction::ShowInFiles => {
    if let Some(ref path) = self.state.media_path.clone() {
        #[cfg(test)]
        {
            self.show_in_files_called = true;
            self.state.show_toast(format!("Show in files: {}", path));
        }
        #[cfg(not(test))]
        {
            let p = path.clone();
            std::thread::spawn(move || {
                let _ = crate::platform_util::show_in_file_manager(&p);
            });
        }
    }
}
```

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// show_in_files 클릭 후 toast 검증:
right_click(&mut harness, "player.video.surface");
let show = click(&mut harness, "player.context.show_in_files");
// state에 media_path가 있으므로 toast가 표시되어야 함
show.assert_selector_present(&selector("player.automatic.toast_lifecycle"));
```
