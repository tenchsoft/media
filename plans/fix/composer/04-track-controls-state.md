# Composer: Track Controls (Mute/Lock/Hidden) 상태 검증 누락

## 문제

`composer.track.mute`, `composer.track.lock`, `composer.track.hidden`이 present-only로 확인되고 클릭하지 않는다.

## 관련 파일

- **소스**: `apps/composer/src-tauri/src/ui/mod.rs:280-296` (ToggleTrackMute, ToggleTrackLock, ToggleTrackHidden)
- **테스트**: `apps/composer/src-tauri/tests/plan_ui_e2e.rs:89-91` (present만 확인)

## 메인 코드 수정

필요 없음.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

```rust
// 기존 (라인 89-91): present만 확인
"composer.track.mute",
"composer.track.lock",
"composer.track.hidden",

// 수정: 클릭 후 상태 검증
let before_mute = state(&mut harness).tracks[0].muted;
click(&mut harness, "composer.track.mute");
assert_ne!(state(&mut harness).tracks[0].muted, before_mute);

let before_lock = state(&mut harness).tracks[0].locked;
click(&mut harness, "composer.track.lock");
assert_ne!(state(&mut harness).tracks[0].locked, before_lock);

let before_hidden = state(&mut harness).tracks[0].hidden;
click(&mut harness, "composer.track.hidden");
assert_ne!(state(&mut harness).tracks[0].hidden, before_hidden);
```
