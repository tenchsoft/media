# Implementation Plan: Timeline Scroll Horizontal

## Feature ID
composer/timeline-scroll-horizontal

## Spec Reference
`plans/spec/composer/timeline-scroll-horizontal.md`

## Design Reference
`plans/design/composer/timeline-scroll-horizontal.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "scroll" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `ComposerState::scroll_offset`

### Change Recipe
1. timeline.rs에서 가로 스크롤 이벤트 감지
2. 휠/트랙패드 → scroll_offset 값 업데이트
3. 타임라인 가시 영역 이동
4. 리렌더

### Find Strategies
- `grep -rn "scroll_offset" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "scroll" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`timeline-scroll-horizontal`

### Verification
- cargo check -p composer
- 스크롤 시 타임라인 가로 이동
