# Implementation Plan: Export Execute

## Feature ID
composer/export-execute

## Spec Reference
`plans/spec/composer/export-execute.md`

## Design Reference
`plans/design/composer/export-execute.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/right_panel.rs`
- Search: `grep -n "export_execute\|start_export" apps/composer/src-tauri/src/ui/right_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ExportSettings`, `export_progress`

### Change Recipe
1. right_panel.rs에서 export 버튼 hit-test
2. 클릭 → export 프로세스 시작
3. 진행률 바 업데이트 (0~100%)
4. 완료 시 토스트 알림
5. 취소 버튼 지원

### Find Strategies
- `grep -rn "export_progress" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "export" apps/composer/src-tauri/src/ui/right_panel.rs`

### Debug ID
`export-execute`

### Verification
- cargo check -p composer
- 클릭 시 렌더링 시작 및 진행률 표시
