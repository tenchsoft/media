# Implementation Plan: AI Panel Close

## Feature ID
view/ai-panel-close

## Spec Reference
`plans/spec/view/ai-panel-close.md`

## Design Reference
`plans/design/view/ai-panel-close.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::show_ai_panel`

### Change Recipe
1. 닫기 버튼 클릭 시 show_ai_panel = false
2. AI 패널 닫힘, 이전 뷰 복원

### Find Strategies
- `grep -rn "show_ai_panel" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`ai-panel-close`

### Verification
- cargo check -p view
