# Implementation Plan: Audio Volume Keyframe

## Feature ID
composer/audio-volume-keyframe

## Spec Reference
`plans/spec/composer/audio-volume-keyframe.md`

## Design Reference
`plans/design/composer/audio-volume-keyframe.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "volume_keyframe" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::volume_keyframes`

### Change Recipe
1. inspector.rs에서 볼륨 키프레임 추가 버튼 hit-test
2. 클릭 → 현재 playhead 위치에 볼륨 키프레임 추가
3. 키프레임 간 선형 보간으로 볼륨 변화
4. 타임라인에 볼륨 랙 표시

### Find Strategies
- `grep -rn "volume_keyframe" apps/composer/src-tauri/src/ui/state.rs`
- `grep -rn "volume" apps/composer/src-tauri/src/ui/inspector.rs`

### Debug ID
`audio-volume-keyframe`

### Verification
- cargo check -p composer
- 키프레임 추가 시 볼륨 자동화 적용
