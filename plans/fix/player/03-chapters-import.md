# Player: Chapters Import (Native Dialog No-op)

## 문제

`player.chapters.import` 버튼 클릭 시 `ClickAction::ImportChapters`가 디스패치되고, `app_handle`이 필요한 file picker를 호출하지만 `app_handle`이 `None`이면 early return된다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:1161-1183`
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:205` (present만 확인, 클릭하지 않음)

## 메인 코드 수정

`apps/player/src-tauri/src/ui/app.rs`:

```rust
// PlayerApp에 테스트용 chapters JSON 주입 필드 추가:
#[cfg(test)]
test_import_chapters_json: Option<String>,

pub fn inject_test_chapters_json(&mut self, json: String) {
    #[cfg(test)]
    {
        self.test_import_chapters_json = Some(json);
    }
}

// ImportChapters 핸들러 수정:
ClickAction::ImportChapters => {
    if let Some(ref handle) = self.app_handle {
        // 기존 native dialog 코드
    } else {
        // 테스트 환경: 주입된 JSON으로 chapters 로드
        #[cfg(test)]
        if let Some(json) = self.test_import_chapters_json.take() {
            self.state.import_chapters_from_json(&json);
            self.state.show_toast("Chapters imported (test)");
        }
    }
}
```

`apps/player/src-tauri/src/ui/state.rs`에 `import_chapters_from_json` 메서드 추가 필요.

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// import 전에 테스트 JSON 주입:
let pod = harness.root_mut();
let app: &mut PlayerApp = pod.widget.downcast_mut().unwrap();
app.inject_test_chapters_json(r#"[{"title":"Test Chapter","time":10.0}]"#.to_string());

let before_count = state(&mut harness).chapters.len();
click(&mut harness, "player.chapters.import");
let after = state(&mut harness);
assert_eq!(after.chapters.len(), before_count + 1);
```
