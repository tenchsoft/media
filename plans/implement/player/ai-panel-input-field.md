# Implementation Plan: AI Panel Input Field

## Feature ID
player/ai-panel-input-field

## Spec Reference
`plans/spec/player/ai-panel-input-field.md`

## Design Reference
`plans/design/player/ai-panel-input-field.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/ai_panel.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::ai_input`

### Change Recipe
1. AI 패널 입력 필드 렌더링
2. 키 입력 시 ai_input 업데이트
3. Enter 키로 요청 전송

### Find Strategies
- `grep -rn "ai_input" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`ai-panel-input-field`

### Verification
- cargo check -p player
