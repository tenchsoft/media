# Implementation Plan: Clip Copy Paste

## Feature ID
composer/clip-copy-paste

## Spec Reference
`plans/spec/composer/clip-copy-paste.md`

## Design Reference
`plans/design/composer/clip-copy-paste.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/mod.rs`, `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "copy\|paste\|clipboard" apps/composer/src-tauri/src/ui/`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::clipboard`

### Change Recipe
1. 클립 선택 후 Ctrl+C → clipboard에 클립 데이터 복사
2. playhead 위치에서 Ctrl+V → clipboard에서 클립 붙여넣기
3. 새 Clip 생성 (deep copy)
4. 리렌더

### Find Strategies
- `grep -rn "clipboard" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "copy\|paste" apps/composer/src-tauri/src/ui/mod.rs`

### Debug ID
`clip-copy-paste`

### Verification
- cargo check -p composer
- 복사/붙여넣기 시 동일 클립 생성
