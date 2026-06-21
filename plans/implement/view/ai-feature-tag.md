# Implementation Plan: AI Feature Tag

## Feature ID
view/ai-feature-tag

## Spec Reference
`plans/spec/view/ai-feature-tag.md`

## Design Reference
`plans/design/view/ai-feature-tag.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::ai_request`

### Change Recipe
1. AI 패널에서 "Auto Tag" 기능 선택
2. 현재 이미지를 Engine에 전송하여 태그 생성
3. 결과 태그 목록을 AI 패널에 표시

### Find Strategies
- `grep -rn "ai_request" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-tag`

### Verification
- cargo check -p view
