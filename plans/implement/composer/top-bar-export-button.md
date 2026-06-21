# Implementation Plan: Top Bar Export Button

## Feature ID
composer/top-bar-export-button

## Spec Reference
`plans/spec/composer/top-bar-export-button.md`

## Design Reference
`plans/design/composer/top-bar-export-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "export" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState`

### Change Recipe
1. toolbar.rs에서 export 버튼 hit-test 영역 확인
2. 클릭 이벤트 → export 패널/모달 표시
3. 포맷/해상도/비트레이트 선택 후 렌더링 시작
4. 진행률 표시 및 완료 알림

### Find Strategies
- `grep -rn "export" apps/composer/src-tauri/src/ui/toolbar.rs`
- `grep -rn "export_state" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`export-button`

### Verification
- cargo check -p composer
- 버튼 클릭 시 export 패널 표시
