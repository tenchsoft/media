# Player: Context Menu Open File (Native Dialog No-op)

## 문제

우클릭 컨텍스트 메뉴에서 `player.context.open_file` 클릭 시 `ClickAction::OpenFile` -> `self.open_file_dialog()` -> `app_handle`이 `None`이라 no-op.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:770-772`
- **소스**: `apps/player/src-tauri/src/ui/app.rs:150-175`
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:385-413` (컨텍스트 메뉴 항목 present만 확인)

## 메인 코드 수정

`01-playlist-add-files.md`와 동일한 `inject_test_files` 메커니즘 사용.

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 컨텍스트 메뉴 섹션에 open_file 클릭 추가:
right_click(&mut harness, "player.video.surface");
// 파일 주입 후 클릭
let pod = harness.root_mut();
let app: &mut PlayerApp = pod.widget.downcast_mut().unwrap();
app.inject_test_files(vec!["/test/new_video.mp4".to_string()]);
let opened = click(&mut harness, "player.context.open_file");
// 파일이 로드되었는지 검증
let s = state(&mut harness);
assert!(s.media_path.as_ref().map_or(false, |p| p.contains("new_video")));
```
