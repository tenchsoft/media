# Implementation Plan: Top Bar Save As Button

## Feature ID
composer/top-bar-save-as-button

## Spec Reference
`plans/spec/composer/top-bar-save-as-button.md`

## Design Reference
`plans/design/composer/top-bar-save-as-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "save_as\|save-as" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::project_path`

### Change Recipe
1. toolbar.rs에서 save-as 버튼 hit-test 영역 확인
2. 클릭 이벤트 → 파일 다이얼로그 오픈 → 새 경로 선택
3. 프로젝트 상태 직렬화하여 새 파일에 저장
4. project_path 업데이트

### Find Strategies
- `grep -rn "save_as" apps/composer/src-tauri/src/ui/toolbar.rs`
- `grep -rn "project_path" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`save-as-button`

### Verification
- cargo check -p composer
- 다른 이름으로 저장 시 새 파일 생성
