# Implementation Plan: Window Resize Reflow

## Feature ID
view/window-resize-reflow

## Spec Reference
`plans/spec/view/window-resize-reflow.md`

## Design Reference
`plans/design/view/window-resize-reflow.md`

## Background Reference
`plans/background/view/window-resize-reflow.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/mod.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::window_size`

### Change Recipe
1. 창 크기 변경 이벤트 감지
2. window_size 업데이트
3. 캔버스, 도구 모음, 패널 레이아웃 자동 리플로우
4. fit-to-window 모드면 줌 레벨 자동 조정

### Find Strategies
- `grep -rn "window_size" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`window-resize-reflow`

### Verification
- cargo check -p view
