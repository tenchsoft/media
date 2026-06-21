# Implementation Plan: AI Feature Smart Crop

## Feature ID
view/ai-feature-smart-crop

## Spec Reference
`plans/spec/view/ai-feature-smart-crop.md`

## Design Reference
`plans/design/view/ai-feature-smart-crop.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::ai_request`

### Change Recipe
1. AI 패널에서 "Smart Crop" 기능 선택
2. 현재 이미지를 Engine에 전송하여 주요 피사체 감지
3. 감지된 영역 기반 크롭 제안 표시

### Find Strategies
- `grep -rn "ai_request" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-smart-crop`

### Verification
- cargo check -p view
