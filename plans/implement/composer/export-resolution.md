# Implementation Plan: Export Resolution

## Feature ID
composer/export-resolution

## Spec Reference
`plans/spec/composer/export-resolution.md`

## Design Reference
`plans/design/composer/export-resolution.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/right_panel.rs`
- Search: `grep -n "resolution" apps/composer/src-tauri/src/ui/right_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ExportSettings::width`, `ExportSettings::height`

### Change Recipe
1. right_panel.rs에서 해상도 프리셋/커스텀 입력 hit-test
2. 프리셋: 4K, 1080p, 720p, 480p
3. 커스텀: width/height 직접 입력
4. 비율 유지 옵션

### Find Strategies
- `grep -rn "ExportSettings" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "resolution\|width\|height" apps/composer/src-tauri/src/ui/right_panel.rs`

### Debug ID
`export-resolution`

### Verification
- cargo check -p composer
- 해상도 변경 시 export 설정 업데이트
