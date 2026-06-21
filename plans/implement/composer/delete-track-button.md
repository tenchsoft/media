# Implementation Plan: Delete Track Button

## Feature ID
composer/delete-track-button

## Spec Reference
`plans/spec/composer/delete-track-button.md`

## Design Reference
`plans/design/composer/delete-track-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "delete_track" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::tracks`

### Change Recipe
1. timeline_panel.rs에서 선택된 트랙의 delete 버튼 hit-test 영역 확인
2. 클릭 이벤트 → tracks 벡터에서 선택된 Track 제거
3. 관련 클립도 함께 제거
4. 타임라인 리렌더

### Find Strategies
- `grep -rn "delete_track" apps/composer/src-tauri/src/ui/`
- `grep -rn "selected_track" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`delete-track-button`

### Verification
- cargo check -p composer
- 클릭 시 선택 트랙 및 클립 제거
