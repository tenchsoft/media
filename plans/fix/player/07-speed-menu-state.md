# Player: Speed Menu 항목 상태 검증 누락

## 문제

10개 speed 메뉴 항목 (`player.speed.0_25x` ~ `player.speed.4x`)을 클릭하지만, 클릭 후 `state.playback_rate` 값 변화를 검증하지 않는다. 클릭 후 speed_menu를 다시 여는 것만 반복한다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:733-740` (`ClickAction::SetSpeed(rate)`)
- **소스**: `apps/player/src-tauri/src/ui/state.rs:1020` (`pub fn set_playback_rate`)
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:144-158`

## 메인 코드 수정

필요 없음. `state.playback_rate`가 이미 public.

## E2E 라이브러리 수정

필요 없음. `state()` 헬퍼로 직접 검증 가능.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 144-158):
for speed in [
    "player.speed.0_25x", ...
] {
    click(&mut harness, speed);
    click(&mut harness, "player.controls.speed_menu");
}

// 수정:
let expected_rates = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];
let speed_ids = [
    "player.speed.0_25x",
    "player.speed.0_5x",
    "player.speed.0_75x",
    "player.speed.1x",
    "player.speed.1_25x",
    "player.speed.1_5x",
    "player.speed.1_75x",
    "player.speed.2x",
    "player.speed.3x",
    "player.speed.4x",
];
for (id, expected_rate) in speed_ids.iter().zip(expected_rates.iter()) {
    click(&mut harness, "player.controls.speed_menu");
    click(&mut harness, id);
    let s = state(&mut harness);
    assert!((s.playback_rate - expected_rate).abs() < 0.01,
        "expected rate {}, got {}", expected_rate, s.playback_rate);
}
```

PlayerState에 `playback_rate: f64` 필드가 public이므로 `state()` 헬퍼로 바로 검증 가능.
