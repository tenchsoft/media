# Implementation Plan: URL Modal Play

## Feature ID
player/url-modal-play

## Spec Reference
`plans/spec/player/url-modal-play.md`

## Design Reference
`plans/design/player/url-modal-play.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_panels.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::url_input`, `PlayerState::show_url_modal`
- Backend: `apps/player/src-tauri/src/gst_backend.rs`

### Change Recipe
1. 재생 버튼 클릭 시 url_input 값으로 스트림 열기
2. gst_backend에 URL 전달하여 재생 시작
3. show_url_modal = false, 모달 닫힘

### Find Strategies
- `grep -rn "url_input" apps/player/src-tauri/src/ui/state.rs`

### Debug ID
`url-modal-play`

### Verification
- cargo check -p player
