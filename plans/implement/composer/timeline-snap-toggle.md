# Implementation Plan: Timeline Snap Toggle

## Feature ID
composer/timeline-snap-toggle

## Spec Reference
`plans/spec/composer/timeline-snap-toggle.md`

## Design Reference
`plans/design/composer/timeline-snap-toggle.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "snap" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::snap_enabled`

### Change Recipe
1. timeline_panel.rs에서 snap 토글 버튼 hit-test 영역 확인
2. 클릭 이벤트 → snap_enabled 플래그 토글
3. 활성화 시 클립 이동/트리밍 중 가장 가까운 경계에 스냅
4. 리렌더

### Find Strategies
- `grep -rn "snap" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "snap" apps/composer/src-tauri/src/ui/timeline_panel.rs`

### Debug ID
`timeline-snap-toggle`

### Verification
- cargo check -p composer
- 토글 시 스냅 동작 on/off
