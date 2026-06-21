# Implementation Plan: AI Feature BG Remove

## Feature ID
view/ai-feature-bg-remove

## Spec Reference
`plans/spec/view/ai-feature-bg-remove.md`

## Design Reference
`plans/design/view/ai-feature-bg-remove.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::ai_request`

### Change Recipe
1. AI 패널에서 "Remove Background" 기능 선택
2. 현재 이미지를 Engine에 전송하여 배경 제거
3. 결과 이미지를 캔버스에 표시

### Find Strategies
- `grep -rn "ai_request" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-bg-remove`

### Verification
- cargo check -p view
