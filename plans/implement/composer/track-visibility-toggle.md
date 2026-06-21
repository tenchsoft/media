# Implementation Plan: Track Visibility Toggle

## Feature ID
composer/track-visibility-toggle

## Spec Reference
`plans/spec/composer/track-visibility-toggle.md`

## Design Reference
`plans/design/composer/track-visibility-toggle.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "visibility\|visible" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::visible`

### Change Recipe
1. timeline_panel.rs에서 트랙 행의 눈 아이콘 hit-test 영역 확인
2. 클릭 이벤트 → Track::visible 플래그 토글
3. 숨김 시 프리뷰에서 해당 트랙 제외
4. 리렌더

### Find Strategies
- `grep -rn "visible" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "visibility" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`track-visibility-toggle`

### Verification
- cargo check -p composer
- 토글 시 프리뷰에서 트랙 표시/숨김
