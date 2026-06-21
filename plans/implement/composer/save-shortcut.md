# Implementation Plan: Save Shortcut

## Feature ID
composer/save-shortcut

## Spec Reference
`plans/spec/composer/save-shortcut.md`

## Design Reference
`plans/design/composer/save-shortcut.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/mod.rs`
- Search: `grep -n "save\|Ctrl.*S" apps/composer/src-tauri/src/ui/mod.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::project_path`

### Change Recipe
1. mod.rs에서 Ctrl+S 키 이벤트 감지
2. 프로젝트 상태 직렬화 → 파일 저장
3. 기존 경로가 없으면 save-as 동작
4. 저장 완료 토스트

### Find Strategies
- `grep -rn "save" apps/composer/src-tauri/src/ui/mod.rs`
- `grep -rn "project_path" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`save-shortcut`

### Verification
- cargo check -p composer
- Ctrl+S 시 프로젝트 저장
