# Player: Screenshot (No Video Frame)

## 문제

`player.controls.screenshot` 버튼 클릭 시 `ClickAction::Screenshot` -> `self.take_screenshot()`이 호출되지만, `video_frame`이 `None`이면 "No video frame to capture" toast만 표시된다. 테스트에서는 toast_lifecycle 존재만 확인하고 메시지 내용을 검증하지 않는다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:762-765, 642-661`
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:165`

## 메인 코드 수정

수정 필요 없음. 현재 동작이 정상. 테스트 환경에서 video_frame이 없으므로 toast가 정상적으로 표시됨.

## E2E 라이브러리 수정

`UiAutomationCapture`에 toast 메시지 텍스트를 읽는 기능이 필요:

```rust
// tench-ui-automation-core에 추가:
pub fn toast_text(&self) -> Option<&str> {
    // ui_tree에서 toast 노드의 label/value 반환
}
```

또는 기존 `find_node`로 toast 노드를 찾아서 label 검증.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 165):
"player.controls.screenshot",

// 수정: screenshot 클릭 후 toast 메시지 검증
let screenshot_result = click(&mut harness, "player.controls.screenshot");
// toast 노드가 나타났는지, 그리고 메시지가 "No video frame"인지 검증
let toast_node = screenshot_result.find_node(&selector("player.automatic.toast_lifecycle"));
assert!(toast_node.is_some());
// 또는 state를 직접 검증:
let s = state(&mut harness);
assert!(s.toast_message.as_ref().map_or(false, |m| m.contains("No video frame")));
```
