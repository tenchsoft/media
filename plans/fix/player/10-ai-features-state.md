# Player: AI Feature 항목 상태 검증 누락

## 문제

4개 AI feature 버튼 (`player.ai.feature.summarize_current_scene`, `find_similar_frames`, `generate_chapter_marks`, `explain_dialogue`)을 클릭하지만 `state.ai_chat_log` 변화를 검증하지 않는다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:998-1013` (`ClickAction::SendAiPrompt`)
- **소스**: `apps/player/src-tauri/src/ui/state.rs:573-576` (ai_input_text, ai_chat_log, ai_request_pending)
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:376-383`

## 메인 코드 수정

필요 없음. AI feature 클릭 시 `state.ai_chat_log`에 메시지가 추가되는지 확인.

현재 소스를 보면 `ClickAction::SendAiPrompt(prompt)`만 chat_log에 메시지를 추가하고, AI feature 버튼들은 별도의 ClickAction enum variant로 처리될 가능성이 높음. feature 버튼들이 prompt를 자동으로 채워서 send하는 방식이라면 `ai_chat_log` 길이 증가를 검증.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 376-383):
for feature in [
    "player.ai.feature.summarize_current_scene",
    ...
] {
    click(&mut harness, feature);
}

// 수정:
let before_count = state(&mut harness).ai_chat_log.len();
for feature in [
    "player.ai.feature.summarize_current_scene",
    "player.ai.feature.find_similar_frames",
    "player.ai.feature.generate_chapter_marks",
    "player.ai.feature.explain_dialogue",
] {
    click(&mut harness, feature);
}
let after = state(&mut harness);
assert!(after.ai_chat_log.len() > before_count);
```
