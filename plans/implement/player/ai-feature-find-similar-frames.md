# Implementation Plan: AI Feature Find Similar Frames

## Feature ID
player/ai-feature-find-similar-frames

## Spec Reference
`plans/spec/player/ai-feature-find-similar-frames.md`

## Design Reference
`plans/design/player/ai-feature-find-similar-frames.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/ai_panel.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::ai_request`

### Change Recipe
1. AI 패널에서 "Find Similar Frames" 기능 선택
2. 현재 프레임을 Engine에 전송하여 유사 프레임 탐색
3. 결과를 타임라인/챕터 목록에 표시

### Find Strategies
- `grep -rn "ai_request" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-find-similar-frames`

### Verification
- cargo check -p player
