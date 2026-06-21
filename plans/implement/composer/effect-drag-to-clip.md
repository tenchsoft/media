# Implementation Plan: Effect Drag To Clip

## Feature ID
composer/effect-drag-to-clip

## Spec Reference
`plans/spec/composer/effect-drag-to-clip.md`

## Design Reference
`plans/design/composer/effect-drag-to-clip.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/left_panel.rs`, `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "effect_drag" apps/composer/src-tauri/src/ui/`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::effects`

### Change Recipe
1. left_panel.rs에서 이펙트 아이템 드래그 시작
2. timeline.rs에서 클립에 드롭
3. Clip::effects 벡터에 새 이펙트 추가
4. 기본 속성값 설정
5. 리렌더

### Find Strategies
- `grep -rn "effects" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "drag" apps/composer/src-tauri/src/ui/left_panel.rs`

### Debug ID
`effect-drag-to-clip`

### Verification
- cargo check -p composer
- 드래그 앤 드롭 시 클립에 이펙트 추가
