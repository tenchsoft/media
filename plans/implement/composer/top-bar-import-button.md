# Implementation Plan: Top Bar Import Button

## Feature ID
composer/top-bar-import-button

## Spec Reference
`plans/spec/composer/top-bar-import-button.md`

## Design Reference
`plans/design/composer/top-bar-import-button.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/toolbar.rs`
- Search: `grep -n "import" apps/composer/src-tauri/src/ui/toolbar.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState`

### Change Recipe
1. toolbar.rs에서 import 버튼 hit-test 영역 확인
2. 클릭 이벤트 → 파일 다이얼로그 오픈 → 미디어 파일 선택
3. 선택된 파일을 `media_bin` 벡터에 추가
4. 상태 업데이트 후 리렌더

### Find Strategies
- `grep -rn "import" apps/composer/src-tauri/src/ui/toolbar.rs`
- `grep -rn "media_bin" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "file_dialog" apps/composer/src-tauri/src/`

### Debug ID
`import-button`

### Verification
- cargo check -p composer
- 버튼 클릭 시 파일 다이얼로그 표시
- 파일 선택 후 미디어 빈에 항목 추가
