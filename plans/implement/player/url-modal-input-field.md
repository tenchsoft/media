# Implementation Plan: URL Modal Input Field

## Feature ID
player/url-modal-input-field

## Spec Reference
`plans/spec/player/url-modal-input-field.md`

## Design Reference
`plans/design/player/url-modal-input-field.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::url_input`

### Change Recipe
1. URL 입력 필드 렌더링
2. 키 입력 시 url_input 업데이트
3. 유효한 URL 형식 검증

### Find Strategies
- `grep -rn "url_input" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`url-modal-input-field`

### Verification
- cargo check -p player
