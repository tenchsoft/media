# Implementation Plan: Export Bitrate

## Feature ID
composer/export-bitrate

## Spec Reference
`plans/spec/composer/export-bitrate.md`

## Design Reference
`plans/design/composer/export-bitrate.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/right_panel.rs`
- Search: `grep -n "bitrate" apps/composer/src-tauri/src/ui/right_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ExportSettings::bitrate`

### Change Recipe
1. right_panel.rs에서 비트레이트 슬라이더/입력 hit-test
2. 값 변경 → ExportSettings::bitrate 업데이트
3. 예상 파일 크기 표시 업데이트

### Find Strategies
- `grep -rn "bitrate" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "bitrate" apps/composer/src-tauri/src/ui/right_panel.rs`

### Debug ID
`export-bitrate`

### Verification
- cargo check -p composer
- 비트레이트 변경 시 예상 크기 업데이트
