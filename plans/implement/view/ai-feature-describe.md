# Implementation Plan: AI Feature Describe

## Feature ID
view/ai-feature-describe

## Spec Reference
`plans/spec/view/ai-feature-describe.md`

## Design Reference
`plans/design/view/ai-feature-describe.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::ai_request`

### Change Recipe
1. AI 패널에서 "Describe Image" 기능 선택
2. 현재 이미지를 Engine에 전송하여 설명 생성
3. 결과 텍스트를 AI 패널에 표시

### Find Strategies
- `grep -rn "ai_request" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-describe`

### Verification
- cargo check -p view
