# Player: Repeat/Shuffle/Aspect/AB-Loop 상태 검증 누락

## 문제

`player.controls.repeat`, `player.controls.shuffle`, `player.controls.aspect`, `player.controls.ab_loop` 버튼을 클릭하지만, 각각의 상태 변화를 검증하지 않고 `player.automatic.toast_lifecycle` 존재만 확인한다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:884-891` (CycleRepeat, ToggleShuffle, CycleAspect)
- **소스**: `apps/player/src-tauri/src/ui/app.rs:766-769` (ToggleABLoop)
- **소스**: `apps/player/src-tauri/src/ui/state.rs:1112` (cycle_repeat_mode), `1122` (toggle_shuffle), `1178` (cycle_aspect), `1404` (toggle_ab_loop)
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:160-173`

## 메인 코드 수정

필요 없음. 관련 state 필드가 public.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 160-173):
for debug_id in [
    "player.controls.repeat",
    "player.controls.shuffle",
    "player.controls.aspect",
    ...
] {
    let current = click(&mut harness, debug_id);
    current.assert_selector_present(&selector("player.automatic.toast_lifecycle"));
}

// 수정:
// Repeat
let before_repeat = state(&mut harness).repeat_mode.clone();
click(&mut harness, "player.controls.repeat");
assert_ne!(state(&mut harness).repeat_mode, before_repeat);

// Shuffle
let before_shuffle = state(&mut harness).shuffle;
click(&mut harness, "player.controls.shuffle");
assert_ne!(state(&mut harness).shuffle, before_shuffle);

// Aspect
let before_aspect = state(&mut harness).aspect_ratio.clone();
click(&mut harness, "player.controls.aspect");
assert_ne!(state(&mut harness).aspect_ratio, before_aspect);

// AB Loop
let before_ab = state(&mut harness).ab_loop;
click(&mut harness, "player.controls.ab_loop");
assert_ne!(state(&mut harness).ab_loop, before_ab);
```
