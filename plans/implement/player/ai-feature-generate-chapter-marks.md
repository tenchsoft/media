# Implementation Plan: AI Feature Generate Chapter Marks

## Feature ID
player/ai-feature-generate-chapter-marks

## Spec Reference
`plans/spec/player/ai-feature-generate-chapter-marks.md`

## Design Reference
`plans/design/player/ai-feature-generate-chapter-marks.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/ai_panel.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::ai_request`, `PlayerState::chapters`

### Change Recipe
1. AI 패널에서 "Generate Chapter Marks" 기능 선택
2. 전체 영상을 Engine에 전송하여 장면 전환 감지
3. 감지된 구간을 chapters 벡터에 자동 추가

### Find Strategies
- `grep -rn "chapters" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`ai-feature-generate-chapter-marks`

### Verification
- cargo check -p player
