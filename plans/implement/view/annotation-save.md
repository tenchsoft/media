# Implementation Plan: Annotation Save

## Feature ID
view/annotation-save

## Spec Reference
`plans/spec/view/annotation-save.md`

## Design Reference
`plans/design/view/annotation-save.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/tools.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::annotations`, `ViewState::current_image`

### Change Recipe
1. 주석 저장 버튼 렌더링
2. 클릭 시 주석을 이미지에 합성
3. 결과 이미지를 current_image에 반영

### Find Strategies
- `grep -rn "annotations" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`annotation-save`

### Verification
- cargo check -p view
