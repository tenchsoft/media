# Implementation Plan: Settings Open

## Feature ID
composer/settings-open

## Spec Reference
`plans/spec/composer/settings-open.md`

## Design Reference
`plans/design/composer/settings-open.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "settings" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::settings_open`

### Change Recipe
1. toolbar.rs에서 설정 버튼 hit-test
2. 클릭 → settings_open = true
3. 설정 모달/패널 표시
4. 닫기 버튼 또는 Esc로 해제

### Find Strategies
- `grep -rn "settings_open" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "settings" apps/composer/src-tauri/src/ui/toolbar.rs`

### Debug ID
`settings-open`

### Verification
- cargo check -p composer
- 클릭 시 설정 패널 표시
