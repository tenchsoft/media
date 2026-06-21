# Implementation Plan: Top Bar Save Button

## Feature ID
composer/top-bar-save-button

## Spec Reference
`plans/spec/composer/top-bar-save-button.md`

## Design Reference
`plans/design/composer/top-bar-save-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "save" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::project_path`

### Change Recipe
1. toolbar.rs에서 save 버튼 hit-test 영역 확인
2. 클릭 이벤트 → 프로젝트 파일에 현재 상태 직렬화하여 저장
3. 기존 경로가 없으면 save-as 동작
4. 저장 완료 토스트 표시

### Find Strategies
- `grep -rn "save" apps/composer/src-tauri/src/ui/toolbar.rs`
- `grep -rn "project_path" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`save-button`

### Verification
- cargo check -p composer
- 저장 시 프로젝트 파일 업데이트
