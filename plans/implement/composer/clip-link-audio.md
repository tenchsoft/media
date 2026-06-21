# Implementation Plan: Clip Link Audio

## Feature ID
composer/clip-link-audio

## Spec Reference
`plans/spec/composer/clip-link-audio.md`

## Design Reference
`plans/design/composer/clip-link-audio.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "link" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::linked_audio_id`

### Change Recipe
1. 비디오 클립과 오디오 클립 동시 선택 후 link 액션
2. 비디오 클립의 linked_audio_id = 오디오 클립 ID
3. 이후 함께 이동/편집
4. 리렌더

### Find Strategies
- `grep -rn "linked_audio" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "link" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`clip-link-audio`

### Verification
- cargo check -p composer
- link 시 오디오/비디오 함께 이동
