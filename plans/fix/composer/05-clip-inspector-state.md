# Composer: Clip Inspector (Name/Speed/Reversed) 상태 검증 누락

## 문제

`composer.clip.name`, `composer.clip.speed`, `composer.clip.reversed`를 클릭하지만 `assert_capture_changed`만 있고 실제 state 값 변화를 검증하지 않는다.

## 관련 파일

- **소스**: `apps/composer/src-tauri/src/ui/mod.rs:352-370` (SetClipName, SetClipSpeed, ToggleClipReversed)
- **테스트**: `apps/composer/src-tauri/tests/plan_ui_e2e.rs:217-222`

## 메인 코드 수정

필요 없음.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

```rust
// 기존 (라인 217-222):
let renamed = click(&mut harness, "composer.clip.name");
assert_capture_changed(&selected, &renamed);
let speed = click(&mut harness, "composer.clip.speed");
assert_capture_changed(&renamed, &speed);
let reversed = click(&mut harness, "composer.clip.reversed");
assert_capture_changed(&speed, &reversed);

// 수정:
// name은 type_text 후 변경 검증
type_text(&mut harness, "composer.clip.name", "Renamed Clip");
assert!(state(&mut harness).selected_clip_name.as_ref().map_or(false, |n| n.contains("Renamed")));

// speed 변경 검증
let before_speed = state(&mut harness).selected_clip_speed;
click(&mut harness, "composer.clip.speed");
// speed가 변경되었는지 검증

// reversed 토글 검증
let before_reversed = state(&mut harness).selected_clip_reversed;
click(&mut harness, "composer.clip.reversed");
assert_ne!(state(&mut harness).selected_clip_reversed, before_reversed);
```
