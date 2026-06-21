# Implementation Plan: Overwrite Clip

## Feature ID
composer/overwrite-clip

## Spec Reference
`plans/spec/composer/overwrite-clip.md`

## Design Reference
`plans/design/composer/overwrite-clip.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "overwrite" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Track::clips`

### Change Recipe
1. playhead 위치에서 overwrite 액션
2. 소스 클립을 playhead 위치에 덮어쓰기
3. 겹치는 기존 클립 부분 제거
4. 리렌더

### Find Strategies
- `grep -rn "overwrite" apps/composer/src-tauri/src/ui/timeline.rs`
- `grep -rn "clips" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`overwrite-clip`

### Verification
- cargo check -p composer
- 덮어쓰기 시 겹치는 부분 교체
