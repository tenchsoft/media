# Composer: Timeline Toggle (Snap/Ripple/Magnet) 상태 검증 누락

## 문제

`composer.timeline.snap`, `composer.timeline.ripple`, `composer.timeline.magnet`을 클릭하지만 `assert_capture_changed`만 있고 `state.snap`, `state.ripple`, `state.magnetic` 불리언 값을 직접 검증하지 않는다.

## 관련 파일

- **소스**: `apps/composer/src-tauri/src/ui/mod.rs:275-277`
- **테스트**: `apps/composer/src-tauri/tests/plan_ui_e2e.rs:118-123`

## 메인 코드 수정

필요 없음. `state.snap`, `state.ripple`, `state.magnetic`이 이미 public.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

```rust
// 기존 (라인 118-123):
let snap = click(&mut harness, "composer.timeline.snap");
assert_capture_changed(&played, &snap);
let ripple = click(&mut harness, "composer.timeline.ripple");
assert_capture_changed(&snap, &ripple);
let magnet = click(&mut harness, "composer.timeline.magnet");
assert_capture_changed(&ripple, &magnet);

// 수정:
let before_snap = state(&mut harness).snap;
click(&mut harness, "composer.timeline.snap");
assert_ne!(state(&mut harness).snap, before_snap);

let before_ripple = state(&mut harness).ripple;
click(&mut harness, "composer.timeline.ripple");
assert_ne!(state(&mut harness).ripple, before_ripple);

let before_magnet = state(&mut harness).magnetic;
click(&mut harness, "composer.timeline.magnet");
assert_ne!(state(&mut harness).magnetic, before_magnet);
```
