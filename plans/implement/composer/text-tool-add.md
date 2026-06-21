# Implementation Plan: Text Tool Add

## Feature ID
composer/text-tool-add

## Spec Reference
`plans/spec/composer/text-tool-add.md`

## Design Reference
`plans/design/composer/text-tool-add.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/preview.rs`
- Search: `grep -n "text_tool\|text_add" apps/composer/src-tauri/src/ui/preview.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::text_overlays`

### Change Recipe
1. preview.rs에서 텍스트 도구 선택 후 클릭
2. 새 TextOverlay 생성 → text_overlays에 추가
3. 기본 텍스트 "Text" 입력
4. 인라인 편집 모드 진입
5. 리렌더

### Find Strategies
- `grep -rn "text_overlay" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "text_tool" apps/composer/src-tauri/src/ui/preview.rs`

### Debug ID
`text-tool-add`

### Verification
- cargo check -p composer
- 클릭 시 텍스트 오버레이 생성
