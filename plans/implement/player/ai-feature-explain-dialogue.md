# Implementation Plan: AI Feature Explain Dialogue

## Feature ID
player/ai-feature-explain-dialogue

## Spec Reference
`plans/spec/player/ai-feature-explain-dialogue.md`

## Design Reference
`plans/design/player/ai-feature-explain-dialogue.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/ai_panel.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::ai_request`

### Change Recipe
1. AI 패널에서 "Explain Dialogue" 기능 선택
2. 현재 재생 위치의 오디오/자막을 Engine에 전송
3. Engine 응답을 AI 패널에 표시

### Find Strategies
- `grep -rn "ai_request" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-explain-dialogue`

### Verification
- cargo check -p player
