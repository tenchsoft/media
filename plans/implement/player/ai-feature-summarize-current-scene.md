# Implementation Plan: AI Feature Summarize Current Scene

## Feature ID
player/ai-feature-summarize-current-scene

## Spec Reference
`plans/spec/player/ai-feature-summarize-current-scene.md`

## Design Reference
`plans/design/player/ai-feature-summarize-current-scene.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/ai_panel.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::ai_request`

### Change Recipe
1. AI 패널에서 "Summarize Current Scene" 기능 선택
2. 현재 프레임 + 주변 컨텍스트를 Engine에 전송
3. Engine 응답(장면 요약)을 AI 패널에 표시

### Find Strategies
- `grep -rn "ai_request" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-summarize-current-scene`

### Verification
- cargo check -p player
