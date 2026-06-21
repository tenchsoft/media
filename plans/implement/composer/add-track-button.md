# Implementation Plan: Add Track Button

## Feature ID
composer/add-track-button

## Spec Reference
`plans/spec/composer/add-track-button.md`

## Design Reference
`plans/design/composer/add-track-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline_panel.rs`
- Search: `grep -n "add_track" apps/composer/src-tauri/src/ui/timeline_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::tracks`

### Change Recipe
1. timeline_panel.rs에서 add-track 버튼 hit-test 영역 확인
2. 클릭 이벤트 → tracks 벡터에 새 Track 추가
3. 기본 트랙 속성(이름, 타입, 볼륨) 설정
4. 타임라인 리렌더

### Find Strategies
- `grep -rn "add_track" apps/composer/src-tauri/src/ui/`
- `grep -rn "tracks" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`add-track-button`

### Verification
- cargo check -p composer
- 클릭 시 새 트랙 행 추가
