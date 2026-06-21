# Player: Playlist Add Files (Native Dialog No-op)

## 문제

`player.playlist.add_files` 버튼 클릭 시 `ClickAction::AddToPlaylist`가 디스패치되고, 이는 `self.open_file_dialog()`를 호출하지만 `app_handle`이 `None`이어서 아무 일도 일어나지 않는다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:995-996` (`ClickAction::AddToPlaylist`)
- **소스**: `apps/player/src-tauri/src/ui/app.rs:150-175` (`fn open_file_dialog`)
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:196`

## 메인 코드 수정

`apps/player/src-tauri/src/ui/app.rs`:

```rust
// PlayerApp에 테스트용 파일 주입 필드 추가:
pub struct PlayerApp {
    // ... 기존 필드
    #[cfg(test)]
    test_next_files: Vec<String>,
}

impl PlayerApp {
    pub fn inject_test_files(&mut self, paths: Vec<String>) {
        #[cfg(test)]
        {
            self.test_next_files = paths;
        }
    }
}

// open_file_dialog 수정:
fn open_file_dialog(&self) {
    #[cfg(test)]
    {
        if !self.test_next_files.is_empty() {
            if let Some(ref tx) = self.dialog_rx.as_ref() {
                // 테스트에서 주입한 파일들을 dialog 결과처럼 처리
            }
            return;
        }
    }
    // 기존 native dialog 코드
}
```

## E2E 라이브러리 수정

해당 없음. `TestHarness` 수준에서는 수정 필요 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 196):
click(&mut harness, "player.playlist.add_files");

// 수정:
let mut harness = harness();
// harness의 app에 테스트 파일 주입
let pod = harness.root_mut();
let app: &mut PlayerApp = pod.widget.downcast_mut().unwrap();
app.inject_test_files(vec!["/test/video1.mp4".to_string()]);

let before_count = /* playlist 길이 */;
click(&mut harness, "player.playlist.add_files");
let after = state(&mut harness);
assert_eq!(after.playlist.len(), before_count + 1);
```
