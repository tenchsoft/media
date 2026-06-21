# Implementation Plan: Clip Speed Change

## Feature ID
composer/clip-speed-change

## Spec Reference
`plans/spec/composer/clip-speed-change.md`

## Design Reference
`plans/design/composer/clip-speed-change.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/inspector.rs`
- Search: `grep -n "speed" apps/composer/src-tauri/src/ui/inspector.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::speed`

### Change Recipe
1. inspector.rs에서 speed 컨트롤 hit-test 영역 확인
2. 슬라이더/입력으로 Clip::speed 값 변경 (0.1x~10x)
3. 속도 변경에 따라 타임라인 상 클립 시각적 길이 업데이트
4. 리렌더

### Find Strategies
- `grep -rn "speed" apps/composer/src-tauri/src/ui/inspector.rs`
- `grep -rn "speed" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`clip-speed-change`

### Verification
- cargo check -p composer
- 속도 변경 시 클립 길이 조정
