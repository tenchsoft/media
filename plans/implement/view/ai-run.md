# Implementation Plan: AI Run

## Feature ID
view/ai-run

## Spec Reference
`plans/spec/view/ai-run.md`

## Design Reference
`plans/design/view/ai-run.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::ai_request`

### Change Recipe
1. AI 패널에서 선택된 기능 실행 버튼 클릭
2. 현재 이미지 + 기능 정보를 Engine에 전송
3. 진행 상태 표시, 완료 시 결과 표시

### Find Strategies
- `grep -rn "ai_request" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`ai-run`

### Verification
- cargo check -p view
