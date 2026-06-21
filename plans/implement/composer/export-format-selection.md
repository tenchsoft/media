# Implementation Plan: Export Format Selection

## Feature ID
composer/export-format-selection

## Spec Reference
`plans/spec/composer/export-format-selection.md`

## Design Reference
`plans/design/composer/export-format-selection.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/right_panel.rs`
- Search: `grep -n "export_format" apps/composer/src-tauri/src/ui/right_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ExportSettings::format`

### Change Recipe
1. right_panel.rs에서 포맷 드롭다운 hit-test
2. 옵션: MP4, MOV, AVI, WebM, GIF
3. 선택 → ExportSettings::format 업데이트
4. 포맷에 따른 코덱 옵션 업데이트

### Find Strategies
- `grep -rn "ExportSettings" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "export_format" apps/composer/src-tauri/src/ui/right_panel.rs`

### Debug ID
`export-format-selection`

### Verification
- cargo check -p composer
- 포맷 선택 시 코덱 옵션 변경
