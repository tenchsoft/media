# Implementation Plan: Track Lock Toggle

## Feature ID
composer/track-lock-toggle

## Spec Reference
`plans/spec/composer/track-lock-toggle.md`

## Design Reference
`plans/design/composer/track-lock-toggle.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "lock" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::locked`

### Change Recipe
1. timeline_panel.rs에서 트랙 행의 자물쇠 아이콘 hit-test 영역 확인
2. 클릭 이벤트 → Track::locked 플래그 토글
3. 잠금 시 해당 트랙의 클립 편집/이동 차단
4. 리렌더

### Find Strategies
- `grep -rn "locked" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "lock" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`track-lock-toggle`

### Verification
- cargo check -p composer
- 잠금 시 클립 조작 불가
