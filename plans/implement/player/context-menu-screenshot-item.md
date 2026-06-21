# Implementation Plan: Context Menu Screenshot Item

## Feature ID
player/context-menu-screenshot-item

## Spec Reference
`plans/spec/player/context-menu-screenshot-item.md`

## Design Reference
`plans/design/player/context-menu-screenshot-item.md`

## Implementation Strategy

### Locate
- File: `apps/player/src-tauri/src/ui/paint_overlays.rs`
- Search: `grep -n "context_menu\|screenshot" apps/player/src-tauri/src/ui/paint_overlays.rs`
- State: `apps/player/src-tauri/src/ui/state.rs` — `PlayerState::video_texture`

### Change Recipe
1. paint_overlays.rs에서 컨텍스트 메뉴에 스크린샷 항목 추가
2. 클릭 시 현재 프레임 캡처
3. 기본 경로에 이미지 파일 저장
4. 토스트로 저장 위치 알림

### Find Strategies
- `grep -rn "screenshot" apps/player/src-tauri/src/ui/state.rs`
- `grep -rn "context_menu" apps/player/src-tauri/src/ui/paint_overlays.rs`

### Debug ID
`context-menu-screenshot-item`

### Verification
- cargo check -p player
- 컨텍스트 메뉴에서 스크린샷 캡처
