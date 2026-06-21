# Implementation Plan: Top Bar AI Panel Button

## Feature ID
player/top-bar-ai-panel-button

## Spec Reference
`plans/spec/player/top-bar-ai-panel-button.md`

## Design Reference
`plans/design/player/top-bar-ai-panel-button.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::show_ai_panel`
- AI Panel: `apps/player/src-tauri/src/ui/ai_panel.rs`

### Change Recipe
1. 상단 바 AI 아이콘 버튼 렌더링
2. 클릭 시 show_ai_panel 토글
3. AI 패널 열림/닫힘

### Find Strategies
- `grep -rn "show_ai_panel" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`top-bar-ai-panel-button`

### Verification
- cargo check -p player
