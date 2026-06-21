# View: URL Load 상태 검증 누락

## 문제

`view.url.load` 클릭 후 `show_url_dialog`가 여전히 `true`인지 검증하고 있으나, 실제로 URL 로드가 트리거되었는지는 검증하지 않는다. URL 입력 없이 load 버튼만 클릭하므로 실제 동작이 없다.

## 관련 파일

- **소스**: `apps/view/src-tauri/src/ui/app.rs:1675` (`ClickAction::LoadFromUrl`)
- **테스트**: `apps/view/src-tauri/tests/plan_ui_e2e.rs:282-284`

## 메인 코드 수정

필요 없음.

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

```rust
// 기존 (라인 282-284):
click(&mut harness, "view.url.load");
assert!(state(&mut harness).show_url_dialog);
click(&mut harness, "view.url.cancel");

// 수정:
// URL을 먼저 입력한 후 load 클릭
click(&mut harness, "view.top.url");
// url input에 텍스트 입력하는 동작이 필요
// ViewState에 url_input_text 필드가 있는지 확인 필요
// load 클릭 후 document가 로드되었는지 또는 status_message 검증
click(&mut harness, "view.url.cancel");
```

URL 입력 필드에 대한 type_text 동작이 현재 테스트에 없음. URL 다이얼로그의 input 필드에 대한 debug_id와 type_text 지원이 필요할 수 있음.
