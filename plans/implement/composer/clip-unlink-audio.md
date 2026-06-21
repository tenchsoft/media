# Implementation Plan: Clip Unlink Audio

## Feature ID
composer/clip-unlink-audio

## Spec Reference
`plans/spec/composer/clip-unlink-audio.md`

## Design Reference
`plans/design/composer/clip-unlink-audio.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "unlink" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::linked_audio_id`

### Change Recipe
1. 비디오+오디오 연결 클립 선택 후 unlink 액션
2. linked_audio_id = None 설정
3. 두 클립이 독립적으로 이동/편집 가능
4. 리렌더

### Find Strategies
- `grep -rn "linked_audio" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "unlink" apps/composer/src-tauri/src/ui/timeline.rs`

### Debug ID
`clip-unlink-audio`

### Verification
- cargo check -p composer
- unlink 시 오디오/비디오 독립 이동
