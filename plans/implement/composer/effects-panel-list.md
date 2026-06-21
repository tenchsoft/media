# Implementation Plan: Effects Panel List

## Feature ID
composer/effects-panel-list

## Spec Reference
`plans/spec/composer/effects-panel-list.md`

## Design Reference
`plans/design/composer/effects-panel-list.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`
- Search: `grep -n "effects\|effect_list" apps/composer/src-tauri/src/ui/left_panel.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::available_effects`

### Change Recipe
1. left_panel.rs에서 이펙트 탭/리스트 렌더링
2. 카테고리별 그룹화 (색상 보정, 블러, 트랜지션 등)
3. 각 이펙트: 아이콘 + 이름
4. 드래그 시작 지원

### Find Strategies
- `grep -rn "available_effects" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "effects" apps/composer/src-tauri/src/ui/left_panel.rs`

### Debug ID
`effects-panel-list`

### Verification
- cargo check -p composer
- 이펙트 목록 카테고리별 표시
