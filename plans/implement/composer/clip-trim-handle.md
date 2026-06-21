# Implementation Plan: Clip Trim Handle

## Feature ID
composer/clip-trim-handle

## Spec Reference
`plans/spec/composer/clip-trim-handle.md`

## Design Reference
`plans/design/composer/clip-trim-handle.md`

## Implementation Strategy

### Locate
- File: `apps/composer/src-tauri/src/ui/timeline.rs`
- Search: `grep -n "trim" apps/composer/src-tauri/src/ui/timeline.rs`
- State: `apps/composer/src-tauri/src/ui/state.rs` — `Clip::start_time`, `Clip::duration`

### Change Recipe
1. timeline.rs에서 클립 좌/우 가장자리 hit-test 영역 확인
2. 드래그 시작 → 트리밍 모드 진입
3. 좌측 핸들: in-point 변경, 우측 핸들: out-point 변경
4. 드롭 → 최종 트리밍 값 확정
5. 최소 지속 시간 보장

### Find Strategies
- `grep -rn "trim" apps/composer/src-tauri/src/ui/timeline.rs`
- `grep -rn "duration" apps/composer/src-tauri/src/ui/state.rs`

### Debug ID
`clip-trim-handle`

### Verification
- cargo check -p composer
- 핸들 드래그 시 클립 길이 변경
